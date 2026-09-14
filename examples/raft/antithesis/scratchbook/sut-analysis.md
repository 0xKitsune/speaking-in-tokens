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

# Design-stage SUT analysis

Inputs: PROMPT.md, DESIGN.md R1–R9, design/components.puml, roles.puml, replication.puml. No production source existed when this independent research pass started. The git commit identifies the containing repository baseline; this design is uncommitted. No reported incidents were supplied or invented.

## Architecture and state
HTTP POST /kv accepts put/get/delete. All are replicated commands; GET /status is stale diagnostics. POST /raft/vote and /raft/append are trusted peer RPCs. A serialized node owner drives a deterministic core; transport and timers deliver explicit events outside the owner lock. Durable state contains term, vote, log, committed index, and cluster identity. A temp-write/fsync/rename/directory-fsync transaction precedes corresponding visible effects. The KV map is replayed only through the committed prefix. A process-exclusive directory lock prevents two owners.

## Concurrency and faults
The high-risk boundaries are transition-to-durable-write, durable-write-to-RPC-response, RPC dispatch-to-delayed-response, log apply-to-HTTP-waiter, and crash-to-replay. Peer failure must not block the owner. Election timers must not be reset by stale messages. Client requests can time out with unknown outcome; no retry deduplication is promised.

A locally recorded leader is not proof of global exclusivity: different terms can have leaders during partitions. Audit election events keyed by term instead. Diagnostic snapshots are asynchronous; do not compare whole maps across different applied indexes.

## Required observability seams
Expose deterministic core inputs and read-only term/vote/log/commit state to Rust integration tests. Provide a storage trait or explicit injected persistence backend: tests must fail the next durable save before effects escape. HTTP waiters must bind index + term + original command (or proposal identity), not just index. Capture command results at application time. Local tests may inspect durable files; distributed monitors should consume immutable election/application events keyed by node, term, index, command and process incarnation. These are proposed seams, not existing instrumentation.

## Attention-focus ledger
Architecture, persistence, concurrency, claimed safety, progress, failure modes, and unproven assumptions are covered above. Product impact is lost acknowledged writes or stale successful reads. Dependencies are Rust HTTP/runtime/serialization plus local filesystem; no database/consensus library. Bug history and existing tests: none at design stage. Version compatibility is explicit rejection of changed cluster identity/invalid storage, not rolling upgrades. Wildcard finding: an overwritten index can wake an old client waiter with another command's result even when the replicated log is correct.

## Design refinements for implementation
1. Bind client completion to exact proposal identity and captured result.
2. Make persistence failure injectable; fence client and peer success responses after failure.
3. Preserve readable event/state surfaces for a test agent independent of implementation.
4. Define malformed/nonmember input validation before changing term or resetting timers.

## Assumptions
Static trusted 1/3/5-node membership; stable durable storage survives process death; no Byzantine peers, snapshots, migration, or exactly-once clients. Claims here are proposed requirements, not verified implementation facts.

## Open Questions
No external-reference answer was received after the optional response window. The provided design and original Raft paper are the stated assistant-assumed scope; no user confirmation is inferred. Hosted Antithesis access and enabled termination faults are unconfirmed; this pass neither provisions nor executes a hosted run.
