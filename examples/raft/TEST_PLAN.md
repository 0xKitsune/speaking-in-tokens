# Independent pre-implementation test plan

Prepared by a separate research agent using antithesis-research (2026-09-08 2e4a837), before Rust implementation. Scope: DESIGN.md R1–R9 and UML. No code/tests existed at the baseline. No additional external-reference answer was received after the optional response window; provided design and original Raft paper are assistant-assumed scope, not user confirmation. This is a design-stage Antithesis research pass, not a hosted test run.

## Acceptance and interfaces
Implementer must stop after production code and existing checks. A fresh test agent implements this plan without weakening its expectations to match code. Record each property as exercised/passed, failed, or deferred in TEST_REPORT.md. Missing hooks or ambiguous expectations are gaps to report, not silently skip. Required seams: deterministic inputs/outputs and readable core state; injected failing persistence; proposal identity and captured results; real node CLI/HTTP and durable paths. Test code can add observation hooks without changing protocol semantics.

## Deterministic Rust tests (P0)
1. Elections: unique configured votes, same-term revote after restart, lexicographic log freshness, single-node immediate election/commit, higher-term stepdown and stale response rejection (R1,R2).
2. Append reconciliation: mismatch rejection, actual uncommitted conflict replacement, matching shorter request does not truncate, committed prefix protected, empty/short append cannot commit unrelated suffix (R3).
3. Quorums: three and five-node majority; old-term-majority must not directly commit; current-term no-op commits inherited prefix. Heartbeat match is only request prefix; delayed failure cannot erase newer success (R4).
4. Apply/results: exact index order, replay only committed prefix immediately after load before protocol messages/timers (retained suffix may later commit lawfully), get/delete result captured at own application position; overwritten proposal never receives replacement's success (R5,R6).
5. Persistence: save failure before outgoing votes, append success, election effects, and client success fences node. Restart maintains term/vote; truncated/invalid/config-mismatched durable state rejected; second directory owner refused (R7,R9).
6. Admission: body/key/value and pending limits, invalid/nonmember RPC no mutation; stale traffic does not reset timer (R8,R9).

## Deterministic event scheduler
Drive 3/5 independent cores through a seeded queue: duplicate/drop/reorder messages, deliver old replies after new terms, isolate subsets, restart from durable copies, and eventually heal. Independent observer stores election events and applied command identity by index, and checks after every event. Keep failing seed/event history. Do not copy core commit/merge logic into the oracle. The scheduler can establish controlled durable-failure windows that real-process SIGKILL cannot reliably hit.

## Real HTTP + disk tests
Start three binary processes on ephemeral local ports with separate temp directories. Poll for election with bounded deadline, submit unique-key put/get/delete, then kill leader after acknowledged put. Retry via surviving nodes, read acknowledged value, restart old leader and verify catch-up. Restart entire cluster retaining paths and re-read. Track and clean only child PIDs. Kill without graceful shutdown. This proves selected process-crash paths, not power-loss durability.

For partition tests use test transport routing/proxy or injectable link controls; if absent, mark real-network partitions deferred and still run deterministic partitions. Submit a new read to an isolated former leader after a newer majority write: no successful stale response. Pending pre-isolation operations may validly finish.

## Independent API oracle
Use unique-key sequential workloads first: record acknowledged value, no overwrites before checked read. For concurrent same-key operations, keep invocation/response history and run a small linearizability search over the sequential KV specification respecting real-time precedence. A timed-out put/delete may have taken effect; include optional completions. Returned log indexes are useful diagnostic corroboration, never the sole ordering oracle. A success result from another proposal is always failure.

## Antithesis handoff
Full property catalog, evidence, topology and evaluation live in antithesis/scratchbook/. Future hosted run uses three node containers plus one Rust workload; immutable internal events supplement external histories. Fault termination availability, SDK/coverage integration and image setup must be confirmed separately. No hosted result is claimed. A finite passing run provides evidence, not a proof or production certification.
