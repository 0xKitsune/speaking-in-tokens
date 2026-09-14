use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::post,
};
use serde_json::{Value, json};
use std::{
    collections::BTreeMap,
    fs,
    path::PathBuf,
    process::{Child, Command, Stdio},
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicUsize, Ordering},
    },
    time::{Duration, Instant},
};
use tokio::task::JoinHandle;
#[derive(Clone)]
struct Proxy {
    enabled: Arc<AtomicBool>,
    target: String,
    client: reqwest::Client,
}
async fn forward(
    State(p): State<Proxy>,
    Path(method): Path<String>,
    Json(body): Json<Value>,
) -> (StatusCode, Json<Value>) {
    if !p.enabled.load(Ordering::SeqCst) {
        return (StatusCode::SERVICE_UNAVAILABLE, Json(json!({})));
    }
    match p
        .client
        .post(format!("{}/raft/{method}", p.target))
        .json(&body)
        .send()
        .await
    {
        Ok(r) => {
            let status = r.status();
            (status, Json(r.json().await.unwrap_or(json!({}))))
        }
        Err(_) => (StatusCode::SERVICE_UNAVAILABLE, Json(json!({}))),
    }
}
struct Cluster {
    dir: PathBuf,
    urls: Vec<String>,
    children: Vec<Option<Child>>,
    links: BTreeMap<(usize, usize), Arc<AtomicBool>>,
    tasks: Vec<JoinHandle<()>>,
    client: reqwest::Client,
}
impl Drop for Cluster {
    fn drop(&mut self) {
        for child in &mut self.children {
            if let Some(mut c) = child.take() {
                let _ = c.kill();
                let _ = c.wait();
            }
        }
        for t in &self.tasks {
            t.abort();
        }
        if !std::thread::panicking() {
            let _ = fs::remove_dir_all(&self.dir);
        } else {
            eprintln!("HTTP failure artifacts: {}", self.dir.display());
        }
    }
}
impl Cluster {
    async fn new() -> Self {
        let dir = std::env::temp_dir().join(format!(
            "raft-http-{}-{}",
            std::process::id(),
            rand::random::<u64>()
        ));
        fs::create_dir_all(&dir).unwrap();
        let mut listeners = vec![];
        for _ in 0..3 {
            listeners.push(std::net::TcpListener::bind("127.0.0.1:0").unwrap());
        }
        let urls: Vec<_> = listeners
            .iter()
            .map(|l| format!("http://{}", l.local_addr().unwrap()))
            .collect();
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(5))
            .build()
            .unwrap();
        let mut x = Self {
            dir,
            urls,
            children: vec![],
            links: BTreeMap::new(),
            tasks: vec![],
            client,
        };
        for i in 0..3 {
            let mut peers = BTreeMap::new();
            for j in 0..3 {
                let url = if i == j {
                    x.urls[j].clone()
                } else {
                    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
                    let url = format!("http://{}", listener.local_addr().unwrap());
                    let enabled = Arc::new(AtomicBool::new(true));
                    x.links.insert((i, j), enabled.clone());
                    let app = Router::new()
                        .route("/raft/{method}", post(forward))
                        .with_state(Proxy {
                            enabled,
                            target: x.urls[j].clone(),
                            client: x.client.clone(),
                        });
                    x.tasks.push(tokio::spawn(async move {
                        axum::serve(listener, app).await.unwrap();
                    }));
                    url
                };
                peers.insert((j + 1).to_string(), url);
            }
            let conf = json!({"cluster":"http-test","id":i+1,"bind":x.urls[i].trim_start_matches("http://"),"data_dir":x.dir.join(format!("data{i}")),"peers":peers});
            fs::write(
                x.dir.join(format!("{i}.json")),
                serde_json::to_vec(&conf).unwrap(),
            )
            .unwrap();
            x.children.push(None);
        }
        drop(listeners);
        for i in 0..3 {
            x.start(i);
        }
        x
    }
    fn start(&mut self, i: usize) {
        let log = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(self.dir.join(format!("{i}.log")))
            .unwrap();
        self.children[i] = Some(
            Command::new(env!("CARGO_BIN_EXE_speaking-raft"))
                .arg(self.dir.join(format!("{i}.json")))
                .stdout(Stdio::null())
                .stderr(log)
                .spawn()
                .unwrap(),
        );
    }
    fn kill(&mut self, i: usize) {
        if let Some(mut c) = self.children[i].take() {
            c.kill().unwrap();
            c.wait().unwrap();
        }
    }
    fn isolate(&self, i: usize, on: bool) {
        for (&(a, b), v) in &self.links {
            if a == i || b == i {
                v.store(!on, Ordering::SeqCst);
            }
        }
    }
    async fn status(&self, i: usize) -> Option<Value> {
        self.client
            .get(format!("{}/status", self.urls[i]))
            .send()
            .await
            .ok()?
            .json()
            .await
            .ok()
    }
    async fn leader(&self, excluded: Option<usize>) -> usize {
        let end = Instant::now() + Duration::from_secs(12);
        while Instant::now() < end {
            for i in 0..3 {
                if Some(i) != excluded
                    && self.children[i].is_some()
                    && let Some(s) = self.status(i).await
                    && s["role"] == "Leader"
                {
                    return i;
                }
            }
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
        panic!("no leader");
    }
    async fn command(&self, i: usize, c: Value) -> (u16, Value) {
        let r = self
            .client
            .post(format!("{}/kv", self.urls[i]))
            .json(&c)
            .send()
            .await
            .unwrap();
        let code = r.status().as_u16();
        (code, r.json().await.unwrap_or(Value::Null))
    }
    async fn success(&self, c: Value, excluded: Option<usize>) -> Value {
        let end = Instant::now() + Duration::from_secs(12);
        while Instant::now() < end {
            let l = self.leader(excluded).await;
            let (code, v) = self.command(l, c.clone()).await;
            if code == 200 {
                return v;
            }
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
        panic!("no command progress {c}")
    }
}
#[derive(Clone, Debug)]
struct Operation {
    input: Value,
    start: usize,
    end: usize,
    code: u16,
    result: Value,
}
fn linearizable(ops: &[Operation]) -> bool {
    fn search(ops: &[Operation], used: u64, value: Option<String>) -> bool {
        if used == (1 << ops.len()) - 1 {
            return true;
        }
        for (i, op) in ops.iter().enumerate() {
            if used & (1 << i) != 0
                || ops.iter().enumerate().any(|(j, p)| {
                    used & (1 << j) == 0
                        && p.end < op.start
                        && !(p.code != 200 && p.result["unknown_outcome"] == true)
                })
            {
                continue;
            }
            let next = match op.input["op"].as_str().unwrap() {
                "put" => Some(op.input["value"].as_str().unwrap().to_owned()),
                "get" => value.clone(),
                "delete" => None,
                _ => panic!(),
            };
            let result = if op.input["op"] == "put" {
                next.clone()
            } else {
                value.clone()
            };
            if op.code == 200 {
                if op.result["value"] != json!(result) {
                    continue;
                }
                if search(ops, used | (1 << i), next) {
                    return true;
                }
            } else {
                // Unknown-outcome commands may have executed or had no effect.
                if search(ops, used | (1 << i), value.clone())
                    || (op.result["unknown_outcome"] == true && search(ops, used | (1 << i), next))
                {
                    return true;
                }
            }
        }
        false
    }
    search(ops, 0, None)
}
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn processes_crashes_partitions_linearizability_and_admission() {
    let mut c = Cluster::new().await;
    let l = c.leader(None).await;
    assert_eq!(
        c.success(json!({"op":"put","key":"durable","value":"ack"}), None)
            .await["value"],
        "ack"
    );
    c.kill(l);
    assert_eq!(
        c.success(json!({"op":"get","key":"durable"}), Some(l))
            .await["value"],
        "ack"
    );
    c.start(l);
    let new = c.leader(None).await;
    assert_eq!(
        c.success(json!({"op":"delete","key":"durable"}), None)
            .await["value"],
        "ack"
    );
    c.success(json!({"op":"put","key":"durable","value":"after"}), None)
        .await;
    // Concurrent history oracle uses invocation/response order and a sequential KV spec, not Raft indexes.
    let clock = Arc::new(AtomicUsize::new(0));
    let mut history = vec![];
    for batch in 0..3 {
        let mut tasks = vec![];
        for j in 0..3 {
            let client = c.client.clone();
            let url = c.urls[new].clone();
            let clock = clock.clone();
            let input = match j {
                0 => json!({"op":"put","key":"concurrent","value":format!("v{batch}")}),
                1 => json!({"op":"get","key":"concurrent"}),
                _ => json!({"op":"delete","key":"concurrent"}),
            };
            tasks.push(tokio::spawn(async move {
                let start = clock.fetch_add(1, Ordering::SeqCst);
                let r = client
                    .post(format!("{url}/kv"))
                    .json(&input)
                    .send()
                    .await
                    .unwrap();
                let code = r.status().as_u16();
                let result = r.json().await.unwrap();
                let end = clock.fetch_add(1, Ordering::SeqCst);
                Operation {
                    input,
                    start,
                    end,
                    code,
                    result,
                }
            }));
        }
        for t in tasks {
            history.push(t.await.unwrap());
        }
    }
    assert!(
        history.iter().all(|o| o.code == 200),
        "healthy history {history:?}"
    );
    assert!(linearizable(&history), "nonlinearizable {history:?}");
    // In-flight proposal must not inherit a replacement entry's success after stepdown.
    let old = c.leader(None).await;
    c.isolate(old, true);
    let client = c.client.clone();
    let url = c.urls[old].clone();
    let waiting = tokio::spawn(async move {
        let r = client
            .post(format!("{url}/kv"))
            .json(&json!({"op":"put","key":"orphan","value":"old"}))
            .send()
            .await
            .unwrap();
        let status = r.status().as_u16();
        (status, r.json::<Value>().await.unwrap())
    });
    tokio::time::sleep(Duration::from_millis(100)).await;
    let majority = c.leader(Some(old)).await;
    let value = c
        .command(majority, json!({"op":"put","key":"orphan","value":"new"}))
        .await;
    assert_eq!(value.0, 200);
    let client = c.client.clone();
    let url = c.urls[old].clone();
    let read = tokio::spawn(async move {
        let r = client
            .post(format!("{url}/kv"))
            .json(&json!({"op":"get","key":"orphan"}))
            .send()
            .await
            .unwrap();
        (r.status().as_u16(), r.json::<Value>().await.unwrap())
    });
    tokio::time::sleep(Duration::from_millis(150)).await;
    assert!(!read.is_finished(), "isolated leader completed a new read");
    assert!(
        !waiting.is_finished(),
        "waiter must still be pending before replacement"
    );
    c.isolate(old, false);
    let r = read.await.unwrap();
    assert_ne!(r.0, 200, "isolated old leader served read");
    assert_eq!(r.1["unknown_outcome"], true);
    let stale = waiting.await.unwrap();
    assert_ne!(stale.0, 200, "old proposal received success: {stale:?}");
    assert_eq!(stale.1["unknown_outcome"], true);
    assert_eq!(
        c.success(json!({"op":"get","key":"orphan"}), None).await["value"],
        "new"
    );
    // Admission on a leader unable to commit; rejected requests must not append.
    let l = c.leader(None).await;
    c.isolate(l, true);
    let before = c.status(l).await.unwrap()["log_len"].as_u64().unwrap();
    let mut tasks = vec![];
    for i in 0..160 {
        let client = c.client.clone();
        let url = c.urls[l].clone();
        tasks.push(tokio::spawn(async move {
            let r = client
                .post(format!("{url}/kv"))
                .json(&json!({"op":"put","key":format!("pending{i}"),"value":"x"}))
                .send()
                .await
                .unwrap();
            r.status().as_u16()
        }));
    }
    tokio::time::sleep(Duration::from_millis(300)).await;
    let mut rejected = 0;
    for t in tasks {
        match t.await.unwrap() {
            429 => rejected += 1,
            504 => {}
            s => panic!("unexpected isolated admission {s}"),
        }
    }
    assert!(rejected >= 32);
    let after = c.status(l).await.unwrap();
    assert!(after["pending"].as_u64().unwrap() <= 128);
    assert!(after["log_len"].as_u64().unwrap() - before <= 128);
    c.isolate(l, false);
    let l = c.leader(None).await;
    for bad in [
        json!({"op":"get","key":""}),
        json!({"op":"get","key":"x".repeat(MAX_KEY+1)}),
        json!({"op":"put","key":"x","value":"x".repeat(MAX_VALUE+1)}),
    ] {
        assert_eq!(c.command(l, bad).await.0, 400);
    }
    let r = c
        .client
        .post(format!("{}/kv", c.urls[l]))
        .header("content-type", "application/json")
        .body("x".repeat(1_048_577))
        .send()
        .await
        .unwrap();
    assert_eq!(r.status().as_u16(), 413);
    let before = c.status(l).await.unwrap();
    let r = c
        .client
        .post(format!("{}/raft/vote", c.urls[l]))
        .json(&json!({"term":999999,"candidate":99,"last_term":0,"last_index":0}))
        .send()
        .await
        .unwrap();
    assert_eq!(r.status().as_u16(), 400);
    assert_eq!(c.status(l).await.unwrap()["term"], before["term"]);
    // Complete-cluster kill/restart preserves acknowledged values and catches up former leader.
    c.success(json!({"op":"put","key":"restart","value":"survives"}), None)
        .await;
    for i in 0..3 {
        c.kill(i);
    }
    for i in 0..3 {
        c.start(i);
    }
    assert_eq!(
        c.success(json!({"op":"get","key":"restart"}), None).await["value"],
        "survives"
    );
    assert_eq!(
        c.success(json!({"op":"get","key":"durable"}), None).await["value"],
        "after"
    );
    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        let mut commits = vec![];
        for i in 0..3 {
            if let Some(s) = c.status(i).await {
                commits.push(s["commit"].as_u64().unwrap());
            }
        }
        if commits.len() == 3 && commits.iter().all(|x| *x == commits[0]) {
            break;
        }
        assert!(Instant::now() < deadline, "followers did not catch up");
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}
use speaking_raft::{MAX_KEY, MAX_VALUE};
#[test]
fn history_oracle_rejects_stale_reads_and_accepts_unknown_writes() {
    let write = Operation {
        input: json!({"op":"put","value":"new"}),
        start: 0,
        end: 1,
        code: 200,
        result: json!({"value":"new"}),
    };
    let stale = Operation {
        input: json!({"op":"get"}),
        start: 2,
        end: 3,
        code: 200,
        result: json!({"value":null}),
    };
    assert!(!linearizable(&[write.clone(), stale.clone()]));
    let unknown = Operation {
        code: 504,
        result: json!({"unknown_outcome":true}),
        ..write
    };
    assert!(linearizable(&[unknown.clone(), stale]));
    assert!(linearizable(&[
        unknown,
        Operation {
            input: json!({"op":"get"}),
            start: 2,
            end: 3,
            code: 200,
            result: json!({"value":"new"})
        }
    ]));
}

#[test]
fn history_oracle_preserves_rejections_and_late_unknown_completion() {
    let rejected = Operation {
        input: json!({"op":"put","value":"x"}),
        start: 0,
        end: 1,
        code: 429,
        result: json!({"unknown_outcome":false}),
    };
    let read = Operation {
        input: json!({"op":"get"}),
        start: 2,
        end: 3,
        code: 200,
        result: json!({"value":"x"}),
    };
    assert!(!linearizable(&[rejected.clone(), read]));
    let unknown = Operation {
        code: 504,
        result: json!({"unknown_outcome":true}),
        ..rejected
    };
    let read_old = Operation {
        input: json!({"op":"get"}),
        start: 2,
        end: 3,
        code: 200,
        result: json!({"value":null}),
    };
    let read_new = Operation {
        input: json!({"op":"get"}),
        start: 4,
        end: 5,
        code: 200,
        result: json!({"value":"x"}),
    };
    assert!(
        linearizable(&[unknown, read_old, read_new]),
        "timed-out write may commit after first read"
    );
}
