use axum::{
    Json, Router,
    extract::{DefaultBodyLimit, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
};
use serde::Deserialize;
use serde_json::{Value, json};
use speaking_raft::{storage::FileStorage, *};
use std::{
    collections::BTreeMap,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::{Semaphore, mpsc, oneshot};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Config {
    cluster: String,
    id: u64,
    bind: String,
    data_dir: String,
    peers: BTreeMap<u64, String>,
}
type Answer = Result<Value, (StatusCode, Value)>;
enum Input {
    Event(Event, Option<oneshot::Sender<Answer>>),
    Status(oneshot::Sender<Answer>),
}
#[derive(Clone)]
struct Api {
    tx: mpsc::Sender<Input>,
    slots: Arc<Semaphore>,
}
fn error(
    code: StatusCode,
    message: &str,
    leader: Option<u64>,
    unknown: bool,
) -> (StatusCode, Value) {
    (
        code,
        json!({"error":message,"leader":leader,"unknown_outcome":unknown}),
    )
}
fn response(answer: Answer) -> Response {
    match answer {
        Ok(value) => (StatusCode::OK, Json(value)).into_response(),
        Err((code, value)) => (code, Json(value)).into_response(),
    }
}
async fn request(api: Api, event: Event) -> Response {
    let permit = match api.slots.try_acquire() {
        Ok(p) => p,
        Err(_) => {
            return response(Err(error(
                StatusCode::TOO_MANY_REQUESTS,
                "request capacity exhausted",
                None,
                false,
            )));
        }
    };
    let (tx, rx) = oneshot::channel();
    if api.tx.try_send(Input::Event(event, Some(tx))).is_err() {
        return response(Err(error(
            StatusCode::SERVICE_UNAVAILABLE,
            "node unavailable or overloaded",
            None,
            false,
        )));
    }
    let result = match tokio::time::timeout(Duration::from_secs(3), rx).await {
        Ok(Ok(answer)) => answer,
        _ => Err(error(
            StatusCode::GATEWAY_TIMEOUT,
            "request outcome unknown",
            None,
            true,
        )),
    };
    drop(permit);
    response(result)
}
async fn kv(State(api): State<Api>, Json(command): Json<Command>) -> Response {
    request(api, Event::Propose(command)).await
}
async fn vote(State(api): State<Api>, Json(r): Json<VoteRequest>) -> Response {
    request(api, Event::Vote(r)).await
}
async fn append(State(api): State<Api>, Json(r): Json<AppendRequest>) -> Response {
    request(api, Event::Append(r)).await
}
async fn status(State(api): State<Api>) -> Response {
    let (tx, rx) = oneshot::channel();
    if api.tx.try_send(Input::Status(tx)).is_err() {
        return response(Err(error(
            StatusCode::SERVICE_UNAVAILABLE,
            "node unavailable",
            None,
            false,
        )));
    }
    response(
        match tokio::time::timeout(Duration::from_secs(1), rx).await {
            Ok(Ok(x)) => x,
            _ => Err(error(
                StatusCode::SERVICE_UNAVAILABLE,
                "node unavailable",
                None,
                false,
            )),
        },
    )
}
struct Waiting {
    identity: Identity,
    answer: oneshot::Sender<Answer>,
    deadline: Instant,
}
fn election_deadline() -> Instant {
    Instant::now() + Duration::from_millis(rand::random::<u64>() % 400 + 600)
}
async fn owner(
    mut node: Node<FileStorage>,
    config: Config,
    mut rx: mpsc::Receiver<Input>,
    tx: mpsc::Sender<Input>,
) {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_millis(250))
        .build()
        .expect("HTTP client");
    let limits: BTreeMap<_, _> = config
        .peers
        .keys()
        .map(|&id| (id, Arc::new(Semaphore::new(1))))
        .collect();
    let mut pending: Vec<Waiting> = vec![];
    let mut election = election_deadline();
    let mut heartbeat = Instant::now();
    let mut ticker = tokio::time::interval(Duration::from_millis(25));
    ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
    loop {
        pending.retain(|w| !w.answer.is_closed());
        let input = tokio::select! {
            input=rx.recv()=>match input {Some(x)=>x,None=>break},
            _=ticker.tick()=>{
                let now=Instant::now();
                let mut keep=vec![];
                for w in pending.drain(..) {if now>=w.deadline {let _=w.answer.send(Err(error(StatusCode::GATEWAY_TIMEOUT,"commit timeout; outcome unknown",node.core.leader,true)));} else {keep.push(w);}}
                pending=keep;
                if node.fenced {continue;}
                if node.core.role==Role::Leader && now>=heartbeat {heartbeat=now+Duration::from_millis(100);Input::Event(Event::Heartbeat,None)}
                else if node.core.role!=Role::Leader && now>=election {election=election_deadline();Input::Event(Event::ElectionTimeout,None)}
                else {continue;}
            }
        };
        let (event, mut answer) = match input {
            Input::Status(answer) => {
                let d = &node.core.durable;
                let _=answer.send(Ok(json!({"id":d.id,"term":d.term,"role":node.core.role,"leader":node.core.leader,"commit":d.commit,"applied":node.core.applied,"log_len":d.log.len(),"fenced":node.fenced,"pending":pending.len()})));
                continue;
            }
            Input::Event(event, answer) => (event, answer),
        };
        if matches!(event, Event::Propose(_)) && pending.len() >= 128 {
            if let Some(a) = answer {
                let _ = a.send(Err(error(
                    StatusCode::TOO_MANY_REQUESTS,
                    "pending command limit",
                    node.core.leader,
                    false,
                )));
            }
            continue;
        }
        let is_proposal = matches!(event, Event::Propose(_));
        let effects = match node.step(event) {
            Ok(e) => e,
            Err(message) => {
                let code = if node.fenced || (is_proposal && node.core.role != Role::Leader) {
                    StatusCode::SERVICE_UNAVAILABLE
                } else {
                    StatusCode::BAD_REQUEST
                };
                if let Some(a) = answer {
                    let _ = a.send(Err(error(code, &message, node.core.leader, node.fenced)));
                }
                if node.fenced {
                    for w in pending.drain(..) {
                        let _ = w.answer.send(Err(error(
                            StatusCode::SERVICE_UNAVAILABLE,
                            "storage failure; outcome unknown",
                            None,
                            true,
                        )));
                    }
                }
                continue;
            }
        };
        if effects.reset_election {
            election = election_deadline();
        }
        if effects.stepped_down {
            for w in pending.drain(..) {
                let _ = w.answer.send(Err(error(
                    StatusCode::SERVICE_UNAVAILABLE,
                    "leadership changed; outcome unknown",
                    node.core.leader,
                    true,
                )));
            }
        }
        if let Some(identity) = effects.proposal
            && let Some(answer) = answer.take()
        {
            pending.push(Waiting {
                identity,
                answer,
                deadline: Instant::now() + Duration::from_millis(2800),
            });
        }
        for applied in effects.applied {
            if let Some(pos) = pending.iter().position(|w| {
                w.identity.index == applied.index
                    && w.identity.term == applied.term
                    && Some(&w.identity.command) == applied.command.as_ref()
            }) {
                let w = pending.swap_remove(pos);
                let _ = w.answer.send(Ok(
                    json!({"index":applied.index,"term":applied.term,"value":applied.value}),
                ));
            }
        }
        if let (Some(a), Some(reply)) = (answer, effects.reply) {
            let value = match reply {
                Reply::Vote(r) => serde_json::to_value(r),
                Reply::Append(r) => serde_json::to_value(r),
            };
            let _ = a.send(Ok(value.expect("serializable reply")));
        }
        for outbound in effects.outbound {
            let peer = match &outbound {
                Outbound::Vote { peer, .. } | Outbound::Append { peer, .. } => *peer,
            };
            let Ok(permit) = limits[&peer].clone().try_acquire_owned() else {
                continue;
            };
            let base = config.peers[&peer].clone();
            let client = client.clone();
            let tx = tx.clone();
            tokio::spawn(async move {
                let event = match outbound {
                    Outbound::Vote { peer, request } => match client
                        .post(format!("{base}/raft/vote"))
                        .json(&request)
                        .send()
                        .await
                    {
                        Ok(r) if r.status().is_success() => r
                            .json::<VoteResponse>()
                            .await
                            .ok()
                            .map(|response| Event::VoteReply {
                                peer,
                                request,
                                response,
                            }),
                        _ => None,
                    },
                    Outbound::Append { peer, request } => match client
                        .post(format!("{base}/raft/append"))
                        .json(&request)
                        .send()
                        .await
                    {
                        Ok(r) if r.status().is_success() => {
                            r.json::<AppendResponse>().await.ok().map(|response| {
                                Event::AppendReply {
                                    peer,
                                    request,
                                    response,
                                }
                            })
                        }
                        _ => None,
                    },
                };
                // Release before delivering the reply so immediate catch-up can send another batch.
                drop(permit);
                if let Some(e) = event {
                    let _ = tx.try_send(Input::Event(e, None));
                }
            });
        }
    }
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args()
        .nth(1)
        .ok_or("usage: speaking-raft CONFIG.json")?;
    let config: Config = serde_json::from_slice(&std::fs::read(path)?)?;
    if config
        .peers
        .values()
        .any(|url| !url.starts_with("http://") || url.ends_with('/'))
    {
        return Err("peer URLs must be http:// URLs without trailing slash".into());
    }
    let expected = Durable::empty(
        config.cluster.clone(),
        config.id,
        config.peers.keys().copied().collect(),
    )?;
    let (storage, durable) = FileStorage::open(&config.data_dir, expected)?;
    let node = Node::new(durable, storage)?;
    let listener = tokio::net::TcpListener::bind(&config.bind).await?;
    let (tx, rx) = mpsc::channel(256);
    let api = Api {
        tx: tx.clone(),
        slots: Arc::new(Semaphore::new(128)),
    };
    eprintln!("node {} listening on {}", config.id, listener.local_addr()?);
    tokio::spawn(owner(node, config, rx, tx));
    let app = Router::new()
        .route("/kv", post(kv))
        .route("/raft/vote", post(vote))
        .route("/raft/append", post(append))
        .route("/status", get(status))
        .layer(DefaultBodyLimit::max(1_048_576))
        .with_state(api);
    axum::serve(listener, app).await?;
    Ok(())
}
