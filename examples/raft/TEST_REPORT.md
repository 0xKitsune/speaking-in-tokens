# Independent test-agent report

The fresh test agent read the frozen design/test plan and implementation handoff before authoring tests. It did not change production behavior or the expected outcomes in DESIGN.md / TEST_PLAN.md. Tests use public deterministic events, durable snapshots, a failing persistence implementation, real storage directories, and actual binary processes behind test-owned HTTP proxies. No new dependencies were needed.

## Results and first-run record

- Initial protocol run: 8 tests passed, including 48 seeded schedules (24 seeds each for three and five nodes; 700 fault events each, then a fair recovery phase).
- Root's concurrent source review found that deleting an established `state.json` could silently reinitialize storage when no temp file remained. The implementation agent repaired startup to recognize the persistent initialization marker. This was a production repair before the storage suite's first execution; the test agent did **not** independently observe an initial failing test for that defect. Missing-state and marker-only regression cases now pass.
- Initial HTTP run: the history-oracle unit test passed; the process test failed because its fixture assumed `/status` always returned status fields during admission saturation. The API may return 503 when its owner queue is full. The fixture was corrected to inspect status after submissions complete; bounds on appended entries and rejection counts remain unchanged. Panic cleanup killed/waited only the fixture's children; retained failure artifacts were under the OS temp directory.
- After that harness correction, the full process scenario passed, including actual HTTP partitions, an outstanding proposal during leader replacement, admission saturation, kill/restart, and concurrent histories. A subsequent full run with strengthened oracle tests also passed.
- Final protocol suite: 10 tests passed after adding explicit stale AppendReply/new-term correlation and failed leader-no-op persistence tests. Full final suite: 16 tests (10 protocol, 3 storage, 3 HTTP including 2 oracle self-tests).
- Commands run: `cargo test --test protocol -- --nocapture`, `cargo test --test storage --test http -- --nocapture` (initial harness failure), `cargo test --tests -- --nocapture`, `cargo test --tests`, `cargo test --test protocol`, `cargo fmt`, and `cargo clippy --all-targets -- -D warnings`. Final consolidated run: `cargo test --tests` passed all 16 tests; `cargo fmt --check` passed; `cargo clippy --all-targets -- -D warnings` passed.

This is not a successful one-shot implementation claim: startup required a source-review repair. The independent tests found a harness assumption to correct; they have not found another production correctness defect in the exercised schedules.

## Coverage against the frozen plan

| Plan / properties | Independent checks | Result |
|---|---|---|
| Elections — R1, R2 | Unique configured votes; duplicate voter in five-node election; durable same-term vote across restart; lexicographic freshness; higher-term stepdown; stale votes/append replies; single-node election and commit | Passed |
| Append reconciliation — R3 | Wrong previous term rejection; shorter matching requests preserve suffix; commit bounded to request end; conflicting uncommitted suffix replaced; committed prefix cannot be overwritten | Passed |
| Quorums — R4 | Three/five-node majority thresholds; inherited old-term entry acknowledged by majority does not directly commit; current-term no-op commits inherited prefix; empty heartbeat matches only its prefix; delayed failure cannot undo success | Passed |
| Apply/results — R5, R6 | Exact ordered put/get/put/delete values; immediate restart replay excludes retained uncommitted suffix; observer checks consecutive per-incarnation application and cross-node command identity; real pending proposal fails on replacement, never receives replacement result | Passed |
| Persistence — R7 | Failing saves suppress election/vote/append/client effects and fence; failed no-op save prevents leadership effects; failed commit save cannot apply; last successful durable state stays unchanged | Passed at abstraction boundary |
| Real disk / ownership — R7, R9 | Term/vote/log/commit roundtrip; exclusive directory lock; identity/membership mismatch; truncated JSON, invalid metadata/log, missing established state, interrupted initialization, valid-state recovery with stale temp | Passed |
| Input / admission — R8, R9 | Core malformed input has no durable mutation; unconfigured HTTP candidate cannot raise term; stale leader traffic does not reset election timer; bounded replication batch; HTTP key/value/body rejection; 160 isolated client submissions yield at least 32 rejections and at most 128 appended entries | Passed |
| Fault scheduler — R1–R5, R8 | Seeded reorder/duplicate/drop, partitions, old replies, restarts; independent election and applied-entry observers; durable term monotonicity; every schedule heals and all nodes reach a fresh command | Passed, finite schedules |
| Real processes — R6–R8 | Three subprocesses, six directed HTTP proxy links; acknowledged write survives killed leader; old leader restart/catch-up; full-cluster kill/restart; actual partition creates new majority leader; isolated old leader cannot complete new read; pending old proposal returns unknown after heal | Passed |
| Client history oracle — R6 | Nine concurrent same-key put/get/delete operations in three rounds; search checks sequential KV behavior and real-time precedence, independently of returned log indexes; self-tests reject stale reads and applied definitive rejections, allow unknown completion after timeout | Passed |

## Research property catalog mapping

Every catalog property has at least one local check; “passed” means exercised examples/schedules, not exhaustive verification.

| Catalog property | Test evidence |
|---|---|
| one-elected-leader-per-term | Seeded independent election observer; distinct configured vote test |
| one-vote-per-term | Durable vote and restart test |
| vote-requires-current-log | Freshness table cases |
| terms-never-regress | Scheduler per-node monotonic observer; restart and term tests |
| matching-prefix-is-preserved | Append reconciliation cases |
| follower-commit-is-request-bounded | Short/empty append with unrelated suffix |
| leader-commit-needs-current-term-quorum | Three/five-node inherited-prefix tests |
| stale-replies-cannot-inflate-match | Delayed failure and old-term reply tests |
| same-index-same-applied-command | Seeded cross-node observer and ordered application test |
| restart-replays-only-committed-prefix | Immediate reconstruction test before any election |
| acknowledged-operations-survive | Real killed leader and full-cluster restart |
| isolated-leader-cannot-complete-new-commands | Real directed partition, pending read/write, bounded unknown outcomes |
| client-results-match-proposals | Per-position get/delete values, independent concurrent history, old waiter replacement |
| failed-persistence-releases-no-success | Injected failures at election, grant, no-op, append, proposal and commit |
| input-and-storage-ownership | Malformed/nonmember inputs, real lock/identity/corruption cases |
| bounded-admission | Core batch/size checks, HTTP body limit and isolated saturation |
| stable-majority-progress | Every seeded schedule's healed phase and real majority failover |
| conflicting-suffix-repair-reached | Explicit differing-term suffix replacement and partition repair |
| repaired-replica-catches-up | Healed scheduler all-node state check; restarted process committed-index convergence |

## Observer independence and limitations

The scheduler records elected leader identity by term and applied command identity by index; it does not reproduce Raft's merge/quorum algorithm. It additionally checks consecutive application indexes per process incarnation. Failing schedules write their seed and event history to `speaking-raft-failure-{cluster-size}-{seed}.trace` in the OS temp directory. No failing randomized seed occurred in these runs. The HTTP fixture retains its config/log directory on failure and otherwise cleans up.

The client oracle models only the public sequential KV specification. A definitive rejected request has no effect; an unknown-outcome request can be omitted or complete later than its HTTP timeout. Actual concurrent search ran a small healthy history; unknown outcomes were exercised in the process partition scenario and in oracle self-tests, not a large mixed-fault linearizability campaign. The pending-proposal replacement scenario checks that the request is still waiting immediately before links heal, avoiding a test that only verifies its earlier timeout.

Durability injection tests failure **before a save completes**, as represented by the persistence seam. They do not emulate a filesystem that returns an error after partially making a new state durable, physical power loss, arbitrary disk corruption during operation, or every fsync/rename interleaving. Corrupt startup states and process SIGKILL are tested separately. Fault scheduling is finite and deterministic; real process timing is bounded and wall-clock dependent.

Hosted Antithesis was **not configured or run**. These tests are local Rust simulation, local disk tests, and local process/HTTP tests. The independent research scratchbook remains the proposed hosted-testing handoff; no SDK instrumentation, container campaign, or production-certification claim is made.
