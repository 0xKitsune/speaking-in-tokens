---
sut_path: /Users/daniel/kit/speaking-in-tokens/examples/raft
commit: fdea7f6bab7e43c121d5a7984b66ea5064a20f1b
updated: 2026-09-14
stage: uncommitted-greenfield-design
external_scope_answer: "No answer received; proceeding with stated assistant assumption"
external_references:
  - path: https://raft.github.io/raft.pdf
    why: Assistant-proposed original protocol reference; not a user-confirmed reference list.
  - path: https://antithesis.com/docs/concepts/properties_assertions/assertions.md
    why: Retrieved official assertion semantics.
  - path: https://antithesis.com/docs/concepts/properties_assertions/sometimes_assertions.md
    why: Retrieved semantic reachability and exploration guidance.
  - path: https://antithesis.com/docs/reference/sdk/define_test_properties.md
    why: Retrieved assertion cataloging requirements.
---

# Property catalog

Design-stage requirements with high confidence in relevance and no confidence claim about implementation correctness. All assertions are proposed; no SDK assertions or tests existed at scan time. P0 blocks acceptance; P1 must be covered or explicitly deferred with a reason.

## one-vote-per-term

- **Priority / type:** P0 / Always
- **Property:** A node grants at most one distinct candidate a vote per term, including across restarts.
- **Invariant and rationale:** `Always` with unique message `one-vote-per-term`. Safety must hold at every evaluated event; also ensure the assertion is encountered.
- **Oracle / fault scenario:** Record granted votes by (node,term) outside SUT; grant A, crash after reply, restart, ask for B in same term.
- **Instrumentation:** Vote grant and durable-save boundary; term/self-vote persistence before outgoing election requests.
- **Why it matters / evidence:** DESIGN.md R1; violation undermines the specified API or protocol boundary. [Evidence](properties/one-vote-per-term.md).
- **Open Questions:** Hosted termination fault availability is unconfirmed; local kill/restart tests are required regardless.

## one-elected-leader-per-term

- **Priority / type:** P0 / Always
- **Property:** At most one node records election victory in a given term.
- **Invariant and rationale:** `Always` with unique message `one-elected-leader-per-term`. Safety must hold at every evaluated event; also ensure the assertion is encountered.
- **Oracle / fault scenario:** Collect immutable election events; duplicate/reorder vote responses and reject nonmember voters; audit distinct voter majority.
- **Instrumentation:** Election transition and configured unique vote set, not asynchronous status snapshots.
- **Why it matters / evidence:** DESIGN.md R1; violation undermines the specified API or protocol boundary. [Evidence](properties/one-elected-leader-per-term.md).
- **Open Questions:** None; implementation hook names are pending source creation.

## terms-never-regress

- **Priority / type:** P0 / Always
- **Property:** Each node term never decreases and higher-term replies fence old leadership before further effects.
- **Invariant and rationale:** `Always` with unique message `terms-never-regress`. Safety must hold at every evaluated event; also ensure the assertion is encountered.
- **Oracle / fault scenario:** Replay old and higher-term vote/append responses after a new election; compare durable/restarted term history.
- **Instrumentation:** RPC/reply ingestion and persistent term transition.
- **Why it matters / evidence:** DESIGN.md R2; violation undermines the specified API or protocol boundary. [Evidence](properties/terms-never-regress.md).
- **Open Questions:** None; implementation hook names are pending source creation.

## vote-requires-current-log

- **Priority / type:** P0 / Always
- **Property:** Granted candidates have a lexicographically current enough last-log term/index.
- **Invariant and rationale:** `Always` with unique message `vote-requires-current-log`. Safety must hold at every evaluated event; also ensure the assertion is encountered.
- **Oracle / fault scenario:** Construct longer old-term log and shorter newer-term log; send vote requests in both directions.
- **Instrumentation:** Vote decision; deterministic tests are primary, network delay adds composition coverage.
- **Why it matters / evidence:** DESIGN.md R1; violation undermines the specified API or protocol boundary. [Evidence](properties/vote-requires-current-log.md).
- **Open Questions:** None; implementation hook names are pending source creation.

## matching-prefix-is-preserved

- **Priority / type:** P0 / Always
- **Property:** Matching entries are retained until an actual uncommitted conflict; committed entries never change.
- **Invariant and rationale:** `Always` with unique message `matching-prefix-is-preserved`. Safety must hold at every evaluated event; also ensure the assertion is encountered.
- **Oracle / fault scenario:** Deliver shorter delayed matching append after longer append, then a true conflicting suffix; compare committed entries against immutable history.
- **Instrumentation:** Append conflict/truncation branch; capture old committed prefix.
- **Why it matters / evidence:** DESIGN.md R3; violation undermines the specified API or protocol boundary. [Evidence](properties/matching-prefix-is-preserved.md).
- **Open Questions:** None; implementation hook names are pending source creation.

## follower-commit-is-request-bounded

- **Priority / type:** P0 / Always
- **Property:** Follower commit cannot advance beyond the prefix matched by the received append request.
- **Invariant and rationale:** `Always` with unique message `follower-commit-is-request-bounded`. Safety must hold at every evaluated event; also ensure the assertion is encountered.
- **Oracle / fault scenario:** Follower has divergent tail; deliver empty or short append with large leaderCommit; assert tail not applied.
- **Instrumentation:** Append commit update with request prev index plus entries length.
- **Why it matters / evidence:** DESIGN.md R3; violation undermines the specified API or protocol boundary. [Evidence](properties/follower-commit-is-request-bounded.md).
- **Open Questions:** None; implementation hook names are pending source creation.

## leader-commit-needs-current-term-quorum

- **Priority / type:** P0 / Always
- **Property:** New leader commit indexes require a durable majority match for a current-term entry.
- **Invariant and rationale:** `Always` with unique message `leader-commit-needs-current-term-quorum`. Safety must hold at every evaluated event; also ensure the assertion is encountered.
- **Oracle / fault scenario:** Reproduce five-node old-term-majority scenario; old-term replication alone does not commit; current-term no-op enables prefix commit.
- **Instrumentation:** Commit calculation; independently recompute quorum and term from recorded acknowledgments.
- **Why it matters / evidence:** DESIGN.md R4; violation undermines the specified API or protocol boundary. [Evidence](properties/leader-commit-needs-current-term-quorum.md).
- **Open Questions:** None; implementation hook names are pending source creation.

## stale-replies-cannot-inflate-match

- **Priority / type:** P0 / Always
- **Property:** Acknowledged match indexes do not exceed the sent request prefix/entries and obsolete responses cannot advance leadership state.
- **Invariant and rationale:** `Always` with unique message `stale-replies-cannot-inflate-match`. Safety must hold at every evaluated event; also ensure the assertion is encountered.
- **Oracle / fault scenario:** Success on heartbeat with long follower tail; delayed failure after newer success; delayed old-term success after reelection.
- **Instrumentation:** Correlated outbound request context and reply processing.
- **Why it matters / evidence:** DESIGN.md R2,R4; violation undermines the specified API or protocol boundary. [Evidence](properties/stale-replies-cannot-inflate-match.md).
- **Open Questions:** None; implementation hook names are pending source creation.

## same-index-same-applied-command

- **Priority / type:** P0 / Always
- **Property:** Any two application events at the same index carry the same command and term; within each process incarnation application indexes advance consecutively and applied <= commit <= log length.
- **Invariant and rationale:** `Always` with unique message `same-index-same-applied-command`. Safety must hold at every evaluated event; also ensure the assertion is encountered.
- **Oracle / fault scenario:** Partition, elect replacement, repair suffix, restart; independent index-to-command ledger rejects disagreement.
- **Instrumentation:** Apply event with index/term/command; replay across incarnations is allowed if identical.
- **Why it matters / evidence:** DESIGN.md R5; violation undermines the specified API or protocol boundary. [Evidence](properties/same-index-same-applied-command.md).
- **Open Questions:** None; implementation hook names are pending source creation.

## acknowledged-operations-survive

- **Priority / type:** P0 / Always
- **Property:** Successful client operations remain in the committed history after crash/replacement.
- **Invariant and rationale:** `Always` with unique message `acknowledged-operations-survive`. Safety must hold at every evaluated event; also ensure the assertion is encountered.
- **Oracle / fault scenario:** Unique key put, receive success, kill leader, elect successor, quorum read; repeat full cluster restart retaining directories.
- **Instrumentation:** Workload success history and independent KV oracle; later overwrites are accounted for, never assert a superseded value remains current.
- **Why it matters / evidence:** DESIGN.md R6,R7; violation undermines the specified API or protocol boundary. [Evidence](properties/acknowledged-operations-survive.md).
- **Open Questions:** Hosted termination fault availability is unconfirmed; local kill/restart tests are required regardless.

## client-results-match-proposals

- **Priority / type:** P0 / Always
- **Property:** A successful response belongs to the original proposal and returns its result at that log position.
- **Invariant and rationale:** `Always` with unique message `client-results-match-proposals`. Safety must hold at every evaluated event; also ensure the assertion is encountered.
- **Oracle / fault scenario:** Old leader proposal times out/steps down; another command replaces index; old waiter must not succeed for replacement. Batch put/get/delete/put before handlers resume to test result capture.
- **Instrumentation:** Waiter registration/completion and apply results keyed by original proposal identity.
- **Why it matters / evidence:** DESIGN.md R6; violation undermines the specified API or protocol boundary. [Evidence](properties/client-results-match-proposals.md).
- **Open Questions:** None; implementation hook names are pending source creation.

## isolated-leader-cannot-complete-new-commands

- **Priority / type:** P0 / Always
- **Property:** A freshly submitted operation on an isolated leader cannot succeed without durable quorum replication.
- **Invariant and rationale:** `Always` with unique message `isolated-leader-cannot-complete-new-commands`. Safety must hold at every evaluated event; also ensure the assertion is encountered.
- **Oracle / fault scenario:** Commit new value through majority, then submit new get/put on isolated former leader; require unavailable/unknown outcome, never stale success.
- **Instrumentation:** Workload partition control and request start boundaries; operations already committed before isolation may complete.
- **Why it matters / evidence:** DESIGN.md R6; violation undermines the specified API or protocol boundary. [Evidence](properties/isolated-leader-cannot-complete-new-commands.md).
- **Open Questions:** None; implementation hook names are pending source creation.

## failed-persistence-releases-no-success

- **Priority / type:** P0 / Always
- **Property:** A failed durable save prevents dependent RPC/client successes and future consensus activity until restart.
- **Invariant and rationale:** `Always` with unique message `failed-persistence-releases-no-success`. Safety must hold at every evaluated event; also ensure the assertion is encountered.
- **Oracle / fault scenario:** Inject errors before write, after temp write, rename and directory sync; record emitted effects and fenced state; restart from whichever complete generation exists.
- **Instrumentation:** Injectable storage and owner effect release boundary; filesystem corruption is separate from process crash.
- **Why it matters / evidence:** DESIGN.md R7; violation undermines the specified API or protocol boundary. [Evidence](properties/failed-persistence-releases-no-success.md).
- **Open Questions:** None; implementation hook names are pending source creation.

## restart-replays-only-committed-prefix

- **Priority / type:** P0 / Always
- **Property:** Restart rebuilds the KV map from exactly the durable committed prefix and rejects invalid storage.
- **Invariant and rationale:** `Always` with unique message `restart-replays-only-committed-prefix`. Safety must hold at every evaluated event; also ensure the assertion is encountered.
- **Oracle / fault scenario:** Persist committed put plus uncommitted delete; inspect replayed map immediately after load before timers/messages can commit more entries. The delete may legitimately commit later through a new leader current-term no-op. Separately test truncated file, impossible commit, mismatched identity.
- **Instrumentation:** Storage load validation and replay; direct local tests primary, termination coverage additionally needed.
- **Why it matters / evidence:** DESIGN.md R5,R7; violation undermines the specified API or protocol boundary. [Evidence](properties/restart-replays-only-committed-prefix.md).
- **Open Questions:** Hosted termination fault availability is unconfirmed; local kill/restart tests are required regardless.

## input-and-storage-ownership

- **Priority / type:** P1 / Always
- **Property:** Malformed/nonmember input cannot mutate protocol state; a second process cannot own the same data directory.
- **Invariant and rationale:** `Always` with unique message `input-and-storage-ownership`. Safety must hold at every evaluated event; also ensure the assertion is encountered.
- **Oracle / fault scenario:** Snapshot durable state then malformed/unknown-sender RPC; launch competing owner; restart with changed membership.
- **Instrumentation:** HTTP validation, config validation and directory lock; primarily ordinary API/process tests.
- **Why it matters / evidence:** DESIGN.md R9; violation undermines the specified API or protocol boundary. [Evidence](properties/input-and-storage-ownership.md).
- **Open Questions:** None; implementation hook names are pending source creation.

## bounded-admission

- **Priority / type:** P1 / Always
- **Property:** Oversized or excess admitted client work cannot bypass configured size/count limits.
- **Invariant and rationale:** `Always` with unique message `bounded-admission`. Safety must hold at every evaluated event; also ensure the assertion is encountered.
- **Oracle / fault scenario:** Send oversized body/key/value and flood requests during unavailable quorum; inspect pending count and rejection path before append.
- **Instrumentation:** Admission counters and request body parser; log growth is explicitly excluded.
- **Why it matters / evidence:** DESIGN.md R9; violation undermines the specified API or protocol boundary. [Evidence](properties/bounded-admission.md).
- **Open Questions:** None; implementation hook names are pending source creation.

## stable-majority-progress

- **Priority / type:** P0 / Sometimes
- **Property:** After recovery a stable reachable majority elects and completes a fresh command.
- **Invariant and rationale:** `Sometimes` with unique message `stable-majority-progress`. Recovery should produce this meaningful state at least once; finite failures alone do not prove impossible progress.
- **Oracle / fault scenario:** Heal faults, allow timers/network/storage to run, retry bounded fresh put/read until successful; local test deadline is a harness bound, not a protocol timing guarantee.
- **Instrumentation:** Workload recovery phase; evaluate condition false then true, not Sometimes(true).
- **Why it matters / evidence:** DESIGN.md R8; violation undermines the specified API or protocol boundary. [Evidence](properties/stable-majority-progress.md).
- **Open Questions:** None; implementation hook names are pending source creation.

## repaired-replica-catches-up

- **Priority / type:** P1 / Sometimes
- **Property:** A restored lagging replica eventually applies an acknowledged target prefix during recovery.
- **Invariant and rationale:** `Sometimes` with unique message `repaired-replica-catches-up`. Recovery should produce this meaningful state at least once; finite failures alone do not prove impossible progress.
- **Oracle / fault scenario:** Record a target committed index and command digest, restore partitioned replica, stop new workload and poll until it contains/applies that target.
- **Instrumentation:** Targeted application event; do not require identical latest maps while writes continue.
- **Why it matters / evidence:** DESIGN.md R3,R8; violation undermines the specified API or protocol boundary. [Evidence](properties/repaired-replica-catches-up.md).
- **Open Questions:** None; implementation hook names are pending source creation.

## conflicting-suffix-repair-reached

- **Priority / type:** P1 / Reachable
- **Property:** Exercise actual conflicting uncommitted suffix replacement.
- **Invariant and rationale:** `Reachable` with unique message `conflicting-suffix-repair-reached`. The branch itself is the desired coverage event.
- **Oracle / fault scenario:** Minority leader appends uncommitted entry, majority commits competing entry, heal; record truncate-and-replace branch.
- **Instrumentation:** SUT conflict replacement event; this is pure branch reachability, not a correctness assertion.
- **Why it matters / evidence:** DESIGN.md R3; violation undermines the specified API or protocol boundary. [Evidence](properties/conflicting-suffix-repair-reached.md).
- **Open Questions:** None; implementation hook names are pending source creation.


## Assumptions
Static trusted 1/3/5-node membership; stable durable storage survives process death; no Byzantine peers, snapshots, migration, or exactly-once clients. Claims here are proposed requirements, not verified implementation facts.

## Open Questions
No external-reference answer was received after the optional response window. The provided design and original Raft paper are the stated assistant-assumed scope; no user confirmation is inferred. Hosted Antithesis access and enabled termination faults are unconfirmed; this pass neither provisions nor executes a hosted run.
