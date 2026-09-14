use speaking_raft::{storage::FileStorage, *};
use std::{fs, path::PathBuf};
struct Temp(PathBuf);
impl Temp {
    fn new() -> Self {
        let p = std::env::temp_dir().join(format!(
            "raft-storage-{}-{}",
            std::process::id(),
            rand::random::<u64>()
        ));
        fs::create_dir_all(&p).unwrap();
        Self(p)
    }
}
impl Drop for Temp {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
fn initial() -> Durable {
    Durable::empty("disk".into(), 1, [1, 2, 3].into()).unwrap()
}
#[test]
fn durable_roundtrip_lock_and_identity() {
    let t = Temp::new();
    let (mut disk, mut s) = FileStorage::open(&t.0, initial()).unwrap();
    assert!(FileStorage::open(&t.0, initial()).is_err());
    s.term = 7;
    s.voted_for = Some(2);
    s.log.push(Entry {
        term: 7,
        command: Some(Command::Put {
            key: "k".into(),
            value: "v".into(),
        }),
    });
    s.commit = 1;
    disk.save(&s).unwrap();
    drop(disk);
    let (disk, loaded) = FileStorage::open(&t.0, initial()).unwrap();
    assert_eq!(s, loaded);
    drop(disk);
    for expected in [
        Durable {
            cluster: "other".into(),
            ..initial()
        },
        Durable { id: 2, ..initial() },
        Durable {
            members: [1, 2, 4].into(),
            ..initial()
        },
    ] {
        assert!(FileStorage::open(&t.0, expected).is_err());
    }
}
#[test]
fn invalid_truncated_and_missing_state_fail_closed() {
    for kind in 0..5 {
        let t = Temp::new();
        let (mut disk, mut s) = FileStorage::open(&t.0, initial()).unwrap();
        s.term = 7;
        s.voted_for = Some(2);
        disk.save(&s).unwrap();
        drop(disk);
        match kind {
            0 => fs::write(t.0.join("state.json"), b"{\"term\":").unwrap(),
            1 => {
                s.commit = 10;
                fs::write(t.0.join("state.json"), serde_json::to_vec(&s).unwrap()).unwrap();
            }
            2 => fs::remove_file(t.0.join("state.json")).unwrap(),
            3 => {
                s.log.push(Entry {
                    term: 8,
                    command: None,
                });
                fs::write(t.0.join("state.json"), serde_json::to_vec(&s).unwrap()).unwrap();
            }
            _ => {
                s.voted_for = Some(99);
                fs::write(t.0.join("state.json"), serde_json::to_vec(&s).unwrap()).unwrap();
            }
        }
        assert!(FileStorage::open(&t.0, initial()).is_err(), "case {kind}");
    }
}
#[test]
fn interrupted_initialization_and_stale_temp() {
    let t = Temp::new();
    fs::write(t.0.join("LOCK"), b"").unwrap();
    assert!(FileStorage::open(&t.0, initial()).is_err());
    let t = Temp::new();
    fs::write(t.0.join("state.tmp"), b"partial").unwrap();
    assert!(FileStorage::open(&t.0, initial()).is_err());
    let t = Temp::new();
    let (disk, s) = FileStorage::open(&t.0, initial()).unwrap();
    drop(disk);
    fs::write(t.0.join("state.tmp"), b"partial").unwrap();
    let (_, loaded) = FileStorage::open(&t.0, initial()).unwrap();
    assert_eq!(s, loaded);
}
