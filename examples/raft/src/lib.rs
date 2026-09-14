//! Deterministic Raft transitions. Only `Node::step` releases persisted effects.
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::io;

pub mod storage;
pub const MAX_KEY: usize = 1024;
pub const MAX_VALUE: usize = 16384;
pub const MAX_BATCH: usize = 32;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "lowercase", deny_unknown_fields)]
pub enum Command {
    Put { key: String, value: String },
    Get { key: String },
    Delete { key: String },
}
impl Command {
    pub fn valid(&self) -> bool {
        let (key, value) = match self {
            Self::Put { key, value } => (key, Some(value)),
            Self::Get { key } | Self::Delete { key } => (key, None),
        };
        !key.is_empty() && key.len() <= MAX_KEY && value.is_none_or(|v| v.len() <= MAX_VALUE)
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Entry {
    pub term: u64,
    pub command: Option<Command>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Durable {
    pub cluster: String,
    pub id: u64,
    pub members: BTreeSet<u64>,
    pub term: u64,
    pub voted_for: Option<u64>,
    pub log: Vec<Entry>,
    pub commit: usize,
}
impl Durable {
    pub fn empty(cluster: String, id: u64, members: BTreeSet<u64>) -> Result<Self, String> {
        let s = Self {
            cluster,
            id,
            members,
            term: 0,
            voted_for: None,
            log: vec![],
            commit: 0,
        };
        s.validate()?;
        Ok(s)
    }
    pub fn validate(&self) -> Result<(), String> {
        if self.cluster.is_empty()
            || ![1, 3, 5].contains(&self.members.len())
            || !self.members.contains(&self.id)
            || self.commit > self.log.len()
            || self.voted_for.is_some_and(|v| !self.members.contains(&v))
            || (self.term == 0 && self.voted_for.is_some())
        {
            return Err("invalid durable metadata".into());
        }
        let mut previous = 0;
        for entry in &self.log {
            if entry.term == 0
                || entry.term < previous
                || entry.term > self.term
                || entry.command.as_ref().is_some_and(|c| !c.valid())
            {
                return Err("invalid durable log".into());
            }
            previous = entry.term;
        }
        Ok(())
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum Role {
    Follower,
    Candidate,
    Leader,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VoteRequest {
    pub term: u64,
    pub candidate: u64,
    pub last_index: usize,
    pub last_term: u64,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VoteResponse {
    pub term: u64,
    pub granted: bool,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppendRequest {
    pub term: u64,
    pub leader: u64,
    pub prev_index: usize,
    pub prev_term: u64,
    pub entries: Vec<Entry>,
    pub leader_commit: usize,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppendResponse {
    pub term: u64,
    pub success: bool,
    pub matched: usize,
}
#[derive(Clone, Debug)]
pub enum Event {
    ElectionTimeout,
    Heartbeat,
    Propose(Command),
    Vote(VoteRequest),
    Append(AppendRequest),
    VoteReply {
        peer: u64,
        request: VoteRequest,
        response: VoteResponse,
    },
    AppendReply {
        peer: u64,
        request: AppendRequest,
        response: AppendResponse,
    },
}
#[derive(Clone, Debug)]
pub enum Outbound {
    Vote { peer: u64, request: VoteRequest },
    Append { peer: u64, request: AppendRequest },
}
#[derive(Clone, Debug)]
pub enum Reply {
    Vote(VoteResponse),
    Append(AppendResponse),
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Identity {
    pub index: usize,
    pub term: u64,
    pub command: Command,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Applied {
    pub index: usize,
    pub term: u64,
    pub command: Option<Command>,
    pub value: Option<String>,
}
#[derive(Clone, Debug, Default)]
pub struct Effects {
    pub outbound: Vec<Outbound>,
    pub reply: Option<Reply>,
    pub proposal: Option<Identity>,
    pub applied: Vec<Applied>,
    pub reset_election: bool,
    pub stepped_down: bool,
}
#[derive(Clone, Debug)]
pub struct Core {
    pub durable: Durable,
    pub role: Role,
    pub leader: Option<u64>,
    pub applied: usize,
    pub kv: BTreeMap<String, String>,
    pub next: BTreeMap<u64, usize>,
    pub matched: BTreeMap<u64, usize>,
    votes: BTreeSet<u64>,
}
impl Core {
    pub fn new(durable: Durable) -> Result<Self, String> {
        durable.validate()?;
        let mut core = Self {
            durable,
            role: Role::Follower,
            leader: None,
            applied: 0,
            kv: BTreeMap::new(),
            next: BTreeMap::new(),
            matched: BTreeMap::new(),
            votes: BTreeSet::new(),
        };
        core.apply(&mut Effects::default());
        Ok(core)
    }
    pub fn term_at(&self, index: usize) -> Option<u64> {
        if index == 0 {
            Some(0)
        } else {
            self.durable.log.get(index - 1).map(|e| e.term)
        }
    }
    fn majority(&self) -> usize {
        self.durable.members.len() / 2 + 1
    }
    fn peer(&self, id: u64) -> bool {
        id != self.durable.id && self.durable.members.contains(&id)
    }
    fn follow(&mut self, term: u64, e: &mut Effects) {
        if self.role != Role::Follower {
            e.stepped_down = true;
        }
        if term > self.durable.term {
            self.durable.term = term;
            self.durable.voted_for = None;
        }
        self.role = Role::Follower;
        self.leader = None;
        self.votes.clear();
        self.next.clear();
        self.matched.clear();
    }
    fn broadcast(&self, e: &mut Effects) {
        for &peer in &self.durable.members {
            if self.peer(peer) {
                self.send_append(peer, e);
            }
        }
    }
    fn send_append(&self, peer: u64, e: &mut Effects) {
        let next = self.next[&peer];
        let prev_index = next - 1;
        e.outbound.push(Outbound::Append {
            peer,
            request: AppendRequest {
                term: self.durable.term,
                leader: self.durable.id,
                prev_index,
                prev_term: self.term_at(prev_index).unwrap(),
                entries: self
                    .durable
                    .log
                    .iter()
                    .skip(prev_index)
                    .take(MAX_BATCH)
                    .cloned()
                    .collect(),
                leader_commit: self.durable.commit,
            },
        });
    }
    fn become_leader(&mut self, e: &mut Effects) {
        self.role = Role::Leader;
        self.leader = Some(self.durable.id);
        let next = self.durable.log.len() + 1;
        for &p in &self.durable.members {
            if p != self.durable.id {
                self.next.insert(p, next);
                self.matched.insert(p, 0);
            }
        }
        self.durable.log.push(Entry {
            term: self.durable.term,
            command: None,
        });
        self.advance_commit();
        self.broadcast(e);
    }
    fn advance_commit(&mut self) {
        for index in (self.durable.commit + 1..=self.durable.log.len()).rev() {
            if self.term_at(index) == Some(self.durable.term)
                && 1 + self.matched.values().filter(|&&v| v >= index).count() >= self.majority()
            {
                self.durable.commit = index;
                break;
            }
        }
    }
    fn apply(&mut self, e: &mut Effects) {
        while self.applied < self.durable.commit {
            let entry = &self.durable.log[self.applied];
            let value = match &entry.command {
                Some(Command::Put { key, value }) => {
                    self.kv.insert(key.clone(), value.clone());
                    Some(value.clone())
                }
                Some(Command::Get { key }) => self.kv.get(key).cloned(),
                Some(Command::Delete { key }) => self.kv.remove(key),
                None => None,
            };
            self.applied += 1;
            e.applied.push(Applied {
                index: self.applied,
                term: entry.term,
                command: entry.command.clone(),
                value,
            });
        }
    }
    // Private: callers must use Node::step to preserve durability-before-effects.
    fn transition(&mut self, event: Event) -> Result<Effects, String> {
        let mut e = Effects::default();
        match event {
            Event::ElectionTimeout => {
                if self.role == Role::Leader {
                    self.broadcast(&mut e);
                    return Ok(e);
                }
                self.durable.term = self.durable.term.checked_add(1).ok_or("term exhausted")?;
                self.durable.voted_for = Some(self.durable.id);
                self.role = Role::Candidate;
                self.leader = None;
                self.votes = BTreeSet::from([self.durable.id]);
                e.reset_election = true;
                if self.votes.len() >= self.majority() {
                    self.become_leader(&mut e);
                } else {
                    let request = VoteRequest {
                        term: self.durable.term,
                        candidate: self.durable.id,
                        last_index: self.durable.log.len(),
                        last_term: self.term_at(self.durable.log.len()).unwrap(),
                    };
                    for &peer in &self.durable.members {
                        if self.peer(peer) {
                            e.outbound.push(Outbound::Vote {
                                peer,
                                request: request.clone(),
                            });
                        }
                    }
                }
            }
            Event::Heartbeat => {
                if self.role == Role::Leader {
                    self.broadcast(&mut e);
                }
            }
            Event::Propose(command) => {
                if !command.valid() {
                    return Err("invalid command".into());
                }
                if self.role != Role::Leader {
                    return Err("not leader".into());
                }
                self.durable.log.push(Entry {
                    term: self.durable.term,
                    command: Some(command.clone()),
                });
                e.proposal = Some(Identity {
                    index: self.durable.log.len(),
                    term: self.durable.term,
                    command,
                });
                self.advance_commit();
                self.broadcast(&mut e);
            }
            Event::Vote(r) => {
                if !self.peer(r.candidate)
                    || r.term == 0
                    || r.last_term > r.term
                    || (r.last_index == 0) != (r.last_term == 0)
                {
                    return Err("malformed vote".into());
                }
                if r.term > self.durable.term {
                    self.follow(r.term, &mut e);
                }
                let fresh = (r.last_term, r.last_index)
                    >= (
                        self.term_at(self.durable.log.len()).unwrap(),
                        self.durable.log.len(),
                    );
                let granted = r.term == self.durable.term
                    && fresh
                    && self.durable.voted_for.is_none_or(|v| v == r.candidate);
                if granted {
                    self.durable.voted_for = Some(r.candidate);
                    e.reset_election = true;
                }
                e.reply = Some(Reply::Vote(VoteResponse {
                    term: self.durable.term,
                    granted,
                }));
            }
            Event::VoteReply {
                peer,
                request,
                response,
            } => {
                if !self.peer(peer)
                    || request.candidate != self.durable.id
                    || response.term < request.term
                {
                    return Err("malformed vote reply".into());
                }
                if response.term > self.durable.term {
                    self.follow(response.term, &mut e);
                }
                if self.role == Role::Candidate
                    && request.term == self.durable.term
                    && response.term == self.durable.term
                    && response.granted
                {
                    self.votes.insert(peer);
                    if self.votes.len() >= self.majority() {
                        self.become_leader(&mut e);
                    }
                }
            }
            Event::Append(r) => {
                let end = r
                    .prev_index
                    .checked_add(r.entries.len())
                    .ok_or("index overflow")?;
                if !self.peer(r.leader)
                    || r.term == 0
                    || r.prev_term > r.term
                    || (r.prev_index == 0) != (r.prev_term == 0)
                    || r.entries.len() > MAX_BATCH
                {
                    return Err("malformed append".into());
                }
                let mut last = r.prev_term;
                for x in &r.entries {
                    if x.term == 0
                        || x.term < last
                        || x.term > r.term
                        || x.command.as_ref().is_some_and(|c| !c.valid())
                    {
                        return Err("malformed entry".into());
                    }
                    last = x.term;
                }
                // Reject contradictory same-term content, and all committed conflicts, before any mutation.
                for (offset, x) in r.entries.iter().enumerate() {
                    let index = r.prev_index + offset + 1;
                    if let Some(old) = self.durable.log.get(index - 1)
                        && old != x
                        && (old.term == x.term || index <= self.durable.commit)
                    {
                        return Err("conflicting committed or same-term entry".into());
                    }
                }
                if r.term > self.durable.term {
                    self.follow(r.term, &mut e);
                }
                let mut success = false;
                if r.term == self.durable.term {
                    self.follow(r.term, &mut e);
                    self.leader = Some(r.leader);
                    e.reset_election = true;
                    if self.term_at(r.prev_index) == Some(r.prev_term) {
                        for (offset, x) in r.entries.into_iter().enumerate() {
                            let index = r.prev_index + offset;
                            if self
                                .durable
                                .log
                                .get(index)
                                .is_some_and(|old| old.term != x.term)
                            {
                                self.durable.log.truncate(index);
                            }
                            if index == self.durable.log.len() {
                                self.durable.log.push(x);
                            }
                        }
                        self.durable.commit = self.durable.commit.max(r.leader_commit.min(end));
                        success = true;
                    }
                }
                e.reply = Some(Reply::Append(AppendResponse {
                    term: self.durable.term,
                    success,
                    matched: if success { end } else { 0 },
                }));
            }
            Event::AppendReply {
                peer,
                request,
                response,
            } => {
                let end = request
                    .prev_index
                    .checked_add(request.entries.len())
                    .ok_or("index overflow")?;
                if !self.peer(peer)
                    || request.leader != self.durable.id
                    || response.term < request.term
                    || (response.success && response.matched != end)
                {
                    return Err("malformed append reply".into());
                }
                if response.term > self.durable.term {
                    self.follow(response.term, &mut e);
                }
                if self.role == Role::Leader
                    && request.term == self.durable.term
                    && response.term == self.durable.term
                {
                    if end > self.durable.log.len() {
                        return Err("unrecognized append reply".into());
                    }
                    if response.success {
                        let matched = self.matched.get_mut(&peer).unwrap();
                        *matched = (*matched).max(end);
                        self.next.insert(peer, *matched + 1);
                        self.advance_commit();
                        if self.next[&peer] <= self.durable.log.len() {
                            self.send_append(peer, &mut e);
                        }
                    } else if request.prev_index + 1 == self.next[&peer]
                        && request.prev_index > self.matched[&peer]
                    {
                        self.next
                            .insert(peer, request.prev_index.max(self.matched[&peer] + 1));
                        self.send_append(peer, &mut e);
                    }
                }
            }
        }
        self.apply(&mut e);
        Ok(e)
    }
}

pub trait Persistence {
    fn save(&mut self, state: &Durable) -> io::Result<()>;
}
pub struct Node<P> {
    pub core: Core,
    pub persistence: P,
    pub fenced: bool,
}
impl<P: Persistence> Node<P> {
    pub fn new(state: Durable, persistence: P) -> Result<Self, String> {
        Ok(Self {
            core: Core::new(state)?,
            persistence,
            fenced: false,
        })
    }
    pub fn step(&mut self, event: Event) -> Result<Effects, String> {
        if self.fenced {
            return Err("node fenced after storage failure".into());
        }
        let mut next = self.core.clone();
        let effects = next.transition(event)?;
        if next.durable != self.core.durable
            && let Err(err) = self.persistence.save(&next.durable)
        {
            self.fenced = true;
            return Err(format!("storage failure; node fenced: {err}"));
        }
        self.core = next;
        Ok(effects)
    }
}
