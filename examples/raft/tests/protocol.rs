use speaking_raft::*;
use std::{collections::BTreeMap, io};
#[derive(Clone)]
struct Memory {
    saved: Durable,
    fail: bool,
}
impl Persistence for Memory {
    fn save(&mut self, s: &Durable) -> io::Result<()> {
        if self.fail {
            Err(io::Error::other("injected"))
        } else {
            self.saved = s.clone();
            Ok(())
        }
    }
}
fn state(id: u64, n: u64) -> Durable {
    Durable::empty("test".into(), id, (1..=n).collect()).unwrap()
}
fn node(s: Durable) -> Node<Memory> {
    Node::new(
        s.clone(),
        Memory {
            saved: s,
            fail: false,
        },
    )
    .unwrap()
}
fn put(v: &str) -> Command {
    Command::Put {
        key: "k".into(),
        value: v.into(),
    }
}
fn entry(t: u64, v: &str) -> Entry {
    Entry {
        term: t,
        command: Some(put(v)),
    }
}
fn vote(t: u64, c: u64, lt: u64, li: usize) -> Event {
    Event::Vote(VoteRequest {
        term: t,
        candidate: c,
        last_term: lt,
        last_index: li,
    })
}
fn granted(e: Effects) -> bool {
    match e.reply.unwrap() {
        Reply::Vote(v) => v.granted,
        _ => panic!(),
    }
}
fn append(t: u64, p: usize, pt: u64, entries: Vec<Entry>, commit: usize) -> Event {
    Event::Append(AppendRequest {
        term: t,
        leader: 2,
        prev_index: p,
        prev_term: pt,
        entries,
        leader_commit: commit,
    })
}
fn elected(n: u64) -> Node<Memory> {
    let mut x = node(state(1, n));
    let e = x.step(Event::ElectionTimeout).unwrap();
    if n > 1 {
        let request = match &e.outbound[0] {
            Outbound::Vote { request, .. } => request.clone(),
            _ => panic!(),
        };
        for peer in 2..=n / 2 + 1 {
            x.step(Event::VoteReply {
                peer,
                request: request.clone(),
                response: VoteResponse {
                    term: 1,
                    granted: true,
                },
            })
            .unwrap();
        }
    }
    assert_eq!(x.core.role, Role::Leader);
    x
}
fn ack(
    x: &mut Node<Memory>,
    peer: u64,
    prev: usize,
    entries: Vec<Entry>,
    success: bool,
) -> Effects {
    let term = x.core.durable.term;
    let end = prev + entries.len();
    let prev_term = x.core.term_at(prev).unwrap();
    x.step(Event::AppendReply {
        peer,
        request: AppendRequest {
            term,
            leader: 1,
            prev_index: prev,
            prev_term,
            entries,
            leader_commit: 0,
        },
        response: AppendResponse {
            term,
            success,
            matched: if success { end } else { 0 },
        },
    })
    .unwrap()
}
#[test]
fn election_unique_members_and_durable_vote() {
    let mut x = node(state(1, 5));
    let e = x.step(Event::ElectionTimeout).unwrap();
    let r = match &e.outbound[0] {
        Outbound::Vote { request, .. } => request.clone(),
        _ => panic!(),
    };
    for _ in 0..3 {
        x.step(Event::VoteReply {
            peer: 2,
            request: r.clone(),
            response: VoteResponse {
                term: 1,
                granted: true,
            },
        })
        .unwrap();
    }
    assert_eq!(x.core.role, Role::Candidate);
    let before = x.core.durable.clone();
    assert!(x
        .step(Event::VoteReply {
            peer: 99,
            request: r.clone(),
            response: VoteResponse {
                term: 1,
                granted: true
            }
        })
        .is_err());
    assert_eq!(before, x.core.durable);
    x.step(Event::VoteReply {
        peer: 3,
        request: r,
        response: VoteResponse {
            term: 1,
            granted: true,
        },
    })
    .unwrap();
    assert_eq!(x.core.role, Role::Leader);
    let mut f = node(state(1, 3));
    assert!(granted(f.step(vote(7, 2, 0, 0)).unwrap()));
    let mut f = node(f.persistence.saved);
    assert!(!granted(f.step(vote(7, 3, 0, 0)).unwrap()));
    assert!(granted(f.step(vote(8, 3, 0, 0)).unwrap()));
}
#[test]
fn freshness_terms_stepdown_and_stale_timer() {
    let mut s = state(1, 3);
    s.term = 4;
    s.log = vec![entry(2, "a"), entry(3, "b")];
    let mut x = node(s);
    assert!(!granted(x.step(vote(5, 2, 2, 100)).unwrap()));
    assert!(!granted(x.step(vote(5, 2, 3, 1)).unwrap()));
    assert!(granted(x.step(vote(5, 2, 4, 1)).unwrap()));
    assert!(!x.step(vote(4, 3, 3, 2)).unwrap().reset_election);
    assert!(!x.step(append(4, 0, 0, vec![], 0)).unwrap().reset_election);
    let mut l = elected(3);
    let e = l.step(vote(2, 2, 1, 1)).unwrap();
    assert!(e.stepped_down);
    assert_eq!(l.core.role, Role::Follower);
    let d = l.core.durable.clone();
    l.step(Event::VoteReply {
        peer: 3,
        request: VoteRequest {
            term: 1,
            candidate: 1,
            last_index: 0,
            last_term: 0,
        },
        response: VoteResponse {
            term: 1,
            granted: true,
        },
    })
    .unwrap();
    assert_eq!(l.core.durable, d);
    assert_eq!(l.core.role, Role::Follower);
}
#[test]
fn append_matching_conflicts_and_commit_bound() {
    let mut s = state(1, 3);
    s.term = 3;
    s.log = vec![entry(1, "a"), entry(2, "b"), entry(2, "c")];
    s.commit = 1;
    let mut x = node(s);
    let e = x.step(append(3, 3, 1, vec![], 3)).unwrap();
    assert!(matches!(
        e.reply,
        Some(Reply::Append(AppendResponse { success: false, .. }))
    ));
    x.step(append(3, 1, 1, vec![], 3)).unwrap();
    assert_eq!(x.core.durable.commit, 1);
    assert_eq!(x.core.durable.log.len(), 3);
    x.step(append(3, 1, 1, vec![entry(2, "b")], 3)).unwrap();
    assert_eq!(x.core.durable.commit, 2);
    assert_eq!(x.core.durable.log.len(), 3);
    let before = x.core.durable.clone();
    assert!(x.step(append(3, 0, 0, vec![entry(3, "wrong")], 1)).is_err());
    assert_eq!(x.core.durable, before);
    x.step(append(3, 2, 2, vec![entry(3, "replacement")], 3))
        .unwrap();
    assert_eq!(x.core.durable.log[2], entry(3, "replacement"));
    assert_eq!(x.core.kv["k"], "replacement");
}
#[test]
fn quorum_current_term_and_reply_correlation() {
    for n in [3, 5] {
        let mut x = elected(n);
        let first = x.core.durable.log.clone();
        ack(&mut x, 2, 0, first.clone(), true);
        assert_eq!(x.core.durable.commit, if n == 3 { 1 } else { 0 });
        if n == 5 {
            ack(&mut x, 3, 0, first, true);
            assert_eq!(x.core.durable.commit, 1);
        }
        x.step(Event::Propose(put("a"))).unwrap();
        let suffix = x.core.durable.log[1..].to_vec();
        ack(&mut x, 2, 1, suffix, true);
        let next = x.core.next[&2];
        ack(&mut x, 2, 1, vec![], false);
        assert_eq!(x.core.next[&2], next);
    }
    let mut s = state(1, 3);
    s.term = 1;
    s.log = vec![entry(1, "inherited")];
    let mut x = node(s);
    let e = x.step(Event::ElectionTimeout).unwrap();
    let r = match &e.outbound[0] {
        Outbound::Vote { request, .. } => request.clone(),
        _ => panic!(),
    };
    x.step(Event::VoteReply {
        peer: 2,
        request: r,
        response: VoteResponse {
            term: 2,
            granted: true,
        },
    })
    .unwrap();
    assert_eq!(
        x.core.durable.log[1],
        Entry {
            term: 2,
            command: None
        }
    );
    ack(&mut x, 2, 1, vec![], true);
    assert_eq!(x.core.matched[&2], 1);
    assert_eq!(x.core.durable.commit, 0);
    let log = x.core.durable.log.clone();
    ack(&mut x, 2, 0, log, true);
    assert_eq!(x.core.durable.commit, 2);
    assert_eq!(x.core.kv["k"], "inherited");
}
#[test]
fn exact_application_results_and_restart_prefix() {
    let mut s = state(1, 3);
    s.term = 1;
    s.log = vec![
        entry(1, "a"),
        Entry {
            term: 1,
            command: Some(Command::Get { key: "k".into() }),
        },
        entry(1, "b"),
        Entry {
            term: 1,
            command: Some(Command::Delete { key: "k".into() }),
        },
        entry(1, "uncommitted"),
    ];
    let mut x = node(s);
    let e = x.step(append(1, 4, 1, vec![], 4)).unwrap();
    assert_eq!(
        e.applied.iter().map(|a| a.index).collect::<Vec<_>>(),
        vec![1, 2, 3, 4]
    );
    assert_eq!(
        e.applied
            .iter()
            .map(|a| a.value.as_deref())
            .collect::<Vec<_>>(),
        vec![Some("a"), Some("a"), Some("b"), Some("b")]
    );
    let r = node(x.persistence.saved);
    assert_eq!(r.core.applied, 4);
    assert!(r.core.kv.is_empty());
    let mut one = elected(1);
    let e = one.step(Event::Propose(put("solo"))).unwrap();
    assert_eq!(e.applied.len(), 1);
    assert_eq!(e.proposal.unwrap().index, e.applied[0].index);
}
#[test]
fn persistence_fences_every_dependent_effect() {
    for event in [
        Event::ElectionTimeout,
        vote(1, 2, 0, 0),
        append(1, 0, 0, vec![entry(1, "x")], 1),
    ] {
        let mut x = node(state(1, 3));
        x.persistence.fail = true;
        let before = x.core.durable.clone();
        assert!(x.step(event).is_err());
        assert!(x.fenced);
        assert_eq!(before, x.core.durable);
        assert_eq!(before, x.persistence.saved);
        assert!(x.step(Event::Heartbeat).is_err());
    }
    let mut x = elected(1);
    x.persistence.fail = true;
    let before = x.core.durable.clone();
    assert!(x.step(Event::Propose(put("never"))).is_err());
    assert_eq!(before, x.core.durable);
    assert!(x.fenced);
    let mut x = elected(3);
    let entries = x.core.durable.log.clone();
    x.persistence.fail = true;
    let event = Event::AppendReply {
        peer: 2,
        request: AppendRequest {
            term: 1,
            leader: 1,
            prev_index: 0,
            prev_term: 0,
            entries,
            leader_commit: 0,
        },
        response: AppendResponse {
            term: 1,
            success: true,
            matched: 1,
        },
    };
    assert!(x.step(event).is_err());
    assert_eq!(x.core.durable.commit, 0);
    assert!(x.fenced);
}
#[test]
fn malformed_inputs_are_atomic_and_batches_bounded() {
    let mut x = elected(3);
    for ev in [
        vote(100, 99, 0, 0),
        append(100, 0, 0, vec![entry(101, "x")], 0),
        Event::Propose(Command::Get { key: "".into() }),
        Event::Propose(Command::Put {
            key: "k".repeat(MAX_KEY + 1),
            value: "v".into(),
        }),
        Event::Propose(Command::Put {
            key: "k".into(),
            value: "v".repeat(MAX_VALUE + 1),
        }),
    ] {
        let before = x.core.durable.clone();
        assert!(x.step(ev).is_err());
        assert_eq!(before, x.core.durable);
    }
    for i in 0..100 {
        let e = x.step(Event::Propose(put(&i.to_string()))).unwrap();
        for m in e.outbound {
            if let Outbound::Append { request, .. } = m {
                assert!(request.entries.len() <= MAX_BATCH);
            }
        }
    }
}

#[derive(Clone, Debug)]
struct Wire {
    from: u64,
    to: u64,
    event: Event,
}
struct Simulation {
    nodes: Vec<Node<Memory>>,
    queue: Vec<Wire>,
    elected: BTreeMap<u64, u64>,
    applied: BTreeMap<usize, (u64, Option<Command>)>,
    trace: Vec<String>,
    terms: Vec<u64>,
}
impl Simulation {
    fn new(n: u64) -> Self {
        Self {
            nodes: (1..=n).map(|i| node(state(i, n))).collect(),
            queue: vec![],
            elected: BTreeMap::new(),
            applied: BTreeMap::new(),
            trace: vec![],
            terms: vec![0; n as usize],
        }
    }
    fn step(&mut self, id: u64, event: Event) {
        self.trace.push(format!("{id}: {event:?}"));
        let original = event.clone();
        let x = &mut self.nodes[id as usize - 1];
        let old_role = x.core.role;
        let old_term = x.core.durable.term;
        let e = x.step(event).unwrap();
        assert!(x.core.durable.term >= self.terms[id as usize - 1]);
        self.terms[id as usize - 1] = x.core.durable.term;
        if x.core.role == Role::Leader
            && (old_role != Role::Leader || old_term != x.core.durable.term)
        {
            if let Some(prior) = self.elected.insert(x.core.durable.term, id) {
                assert_eq!(prior, id, "R1");
            }
        }
        for a in e.applied {
            if let Some(prior) = self.applied.insert(a.index, (a.term, a.command.clone())) {
                assert_eq!(prior, (a.term, a.command), "R5");
            }
        }
        assert!(
            x.core.applied <= x.core.durable.commit
                && x.core.durable.commit <= x.core.durable.log.len()
        );
        assert_eq!(x.core.durable, x.persistence.saved);
        for outbound in e.outbound {
            match outbound {
                Outbound::Vote { peer, request } => self.queue.push(Wire {
                    from: id,
                    to: peer,
                    event: Event::Vote(request),
                }),
                Outbound::Append { peer, request } => self.queue.push(Wire {
                    from: id,
                    to: peer,
                    event: Event::Append(request),
                }),
            }
        }
        match (original, e.reply) {
            (Event::Vote(request), Some(Reply::Vote(response))) => self.queue.push(Wire {
                from: id,
                to: request.candidate,
                event: Event::VoteReply {
                    peer: id,
                    request,
                    response,
                },
            }),
            (Event::Append(request), Some(Reply::Append(response))) => self.queue.push(Wire {
                from: id,
                to: request.leader,
                event: Event::AppendReply {
                    peer: id,
                    request,
                    response,
                },
            }),
            _ => {}
        }
    }
    fn drain(&mut self) {
        for _ in 0..10000 {
            if self.queue.is_empty() {
                return;
            }
            let w = self.queue.remove(0);
            self.step(w.to, w.event);
        }
        panic!("queue did not quiesce")
    }
    fn restart(&mut self, id: u64) {
        self.trace.push(format!("restart {id}"));
        let s = self.nodes[id as usize - 1].persistence.saved.clone();
        let x = node(s);
        for i in 0..x.core.applied {
            let e = &x.core.durable.log[i];
            if let Some(prior) = self.applied.insert(i + 1, (e.term, e.command.clone())) {
                assert_eq!(prior, (e.term, e.command.clone()));
            }
        }
        self.nodes[id as usize - 1] = x;
    }
}
#[test]
fn seeded_fault_schedules_preserve_safety_then_progress() {
    use rand::{Rng, SeedableRng};
    for n in [3, 5] {
        for seed in 0..24 {
            let mut sim = Simulation::new(n);
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
                for tick in 0..700 {
                    let id = rng.gen_range(1..=n);
                    match rng.gen_range(0..10) {
                        0 => sim.step(id, Event::ElectionTimeout),
                        1 => sim.restart(id),
                        2 => {
                            if sim.nodes[id as usize - 1].core.role == Role::Leader {
                                sim.step(id, Event::Propose(put(&format!("{seed}-{tick}"))));
                            }
                        }
                        3 => sim.step(id, Event::Heartbeat),
                        _ => {
                            if !sim.queue.is_empty() {
                                let i = rng.gen_range(0..sim.queue.len());
                                let w = sim.queue.swap_remove(i);
                                let partition = tick % 200 < 100 && ((w.from == 1) != (w.to == 1));
                                if !partition && rng.gen_ratio(4, 5) {
                                    if rng.gen_ratio(1, 8) {
                                        sim.queue.push(w.clone());
                                    }
                                    sim.step(w.to, w.event);
                                }
                            }
                        }
                    }
                }
                sim.queue.clear();
                for _ in 0..30 {
                    let mut order: Vec<_> = (1..=n).collect();
                    order.sort_by_key(|&id| {
                        let d = &sim.nodes[id as usize - 1].core.durable;
                        (d.log.last().map(|e| e.term).unwrap_or(0), d.log.len())
                    });
                    let id = *order.last().unwrap();
                    sim.step(id, Event::ElectionTimeout);
                    sim.drain();
                    if sim.nodes[id as usize - 1].core.role == Role::Leader {
                        sim.step(id, Event::Propose(put("healed")));
                        sim.drain();
                        sim.step(id, Event::Heartbeat);
                        sim.drain();
                        assert!(sim
                            .nodes
                            .iter()
                            .all(|x| x.core.kv.get("k").map(String::as_str) == Some("healed")));
                        return;
                    }
                }
                panic!("R8 no progress after healing");
            }));
            if let Err(e) = result {
                let path =
                    std::env::temp_dir().join(format!("speaking-raft-failure-{n}-{seed}.trace"));
                std::fs::write(&path, sim.trace.join("\n")).unwrap();
                eprintln!("FAILED n={n} seed={seed}, trace {}", path.display());
                std::panic::resume_unwind(e);
            }
        }
    }
}

#[test]
fn stale_append_reply_cannot_change_new_leader_replication() {
    let mut x = elected(3);
    let old = AppendRequest {
        term: 1,
        leader: 1,
        prev_index: 0,
        prev_term: 0,
        entries: x.core.durable.log.clone(),
        leader_commit: 0,
    };
    let e = x
        .step(Event::AppendReply {
            peer: 2,
            request: old.clone(),
            response: AppendResponse {
                term: 2,
                success: false,
                matched: 0,
            },
        })
        .unwrap();
    assert!(e.stepped_down);
    assert_eq!(x.core.role, Role::Follower);
    assert_eq!(x.core.durable.voted_for, None);
    let e = x.step(Event::ElectionTimeout).unwrap();
    let request = match &e.outbound[0] {
        Outbound::Vote { request, .. } => request.clone(),
        _ => panic!(),
    };
    x.step(Event::VoteReply {
        peer: 2,
        request,
        response: VoteResponse {
            term: 3,
            granted: true,
        },
    })
    .unwrap();
    let next = x.core.next.clone();
    let matched = x.core.matched.clone();
    let durable = x.core.durable.clone();
    x.step(Event::AppendReply {
        peer: 2,
        request: old,
        response: AppendResponse {
            term: 1,
            success: true,
            matched: 1,
        },
    })
    .unwrap();
    assert_eq!(x.core.next, next);
    assert_eq!(x.core.matched, matched);
    assert_eq!(x.core.durable, durable);
}
#[test]
fn leader_transition_noop_must_persist_before_replication() {
    let mut x = node(state(1, 3));
    let e = x.step(Event::ElectionTimeout).unwrap();
    let request = match &e.outbound[0] {
        Outbound::Vote { request, .. } => request.clone(),
        _ => panic!(),
    };
    let before = x.persistence.saved.clone();
    x.persistence.fail = true;
    assert!(x
        .step(Event::VoteReply {
            peer: 2,
            request,
            response: VoteResponse {
                term: 1,
                granted: true
            }
        })
        .is_err());
    assert!(x.fenced);
    assert_eq!(x.core.role, Role::Candidate);
    assert_eq!(x.persistence.saved, before);
    assert_eq!(x.core.durable, before);
}
