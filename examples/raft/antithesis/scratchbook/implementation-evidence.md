---
sut_path: /Users/daniel/kit/speaking-in-tokens/examples/raft
commit: 937e42c6bc2f77d4b52cba3698a4e906161cf638
updated: 2026-09-14
stage: post-implementation-source-review
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

# Implementation evidence refresh

Independent source-only pass after the implementation agent stopped and while the fresh test agent worked. Reviewed all of src/lib.rs, src/main.rs, src/storage.rs and Cargo.toml. Original design-stage files are preserved as historical evidence. Source paths and line numbers below refer to this inspection; repairs can shift them. No tests, builds, hosted runs, or mutations to production code were performed by this pass.

## Actual SDK scan
Searched src/ and Cargo.toml for `antithesis`, `assert_always`, `assert_sometimes`, `assert_reachable`, and `assert_unreachable`. **No matches.** There is no Antithesis SDK dependency or assertion instrumentation. Public deterministic state and effects are implemented test seams, not SDK integration. Existing-assertions.md continues to document the original preimplementation scan; this file is the source-stage update.

## Property-to-source map

| Properties | Current implementation evidence |
|---|---|
| one-vote-per-term; vote-requires-current-log; one-elected-leader-per-term | lib.rs:295 ElectionTimeout persists increment/self-vote through Node::step; :352 Vote validates configured sender/freshness and existing vote; :380 VoteReply counts BTreeSet voters and current term |
| terms-never-regress | lib.rs follow helper; :380/:471 higher-term replies step down; durable state validation rejects entry terms above current term |
| matching-prefix-is-preserved; follower-commit-is-request-bounded | lib.rs:405 Append validates input before mutation, rejects committed/same-term contradictory entries, checks prefix, truncates actual conflict, and bounds commit by request end |
| leader-commit-needs-current-term-quorum | lib.rs:263 advance_commit counts self plus durable matched peer indexes and requires current term; become_leader appends no-op |
| stale-replies-cannot-inflate-match | lib.rs:471 requires matched == request end, same leadership term, monotonic match, and failure prefix equal current next index |
| same-index-same-applied-command | lib.rs:273 apply emits Applied(index,term,command,value), increments in order. Cross-node equality is not checked internally; independent scheduler must check it |
| acknowledged-operations-survive; isolated-leader-cannot-complete-new-commands | lib.rs:333 all commands including reads enter log; main.rs:227 successful response comes only from Applied effect. External failover/read oracle remains necessary |
| client-results-match-proposals | main.rs:209 drains pending requests on stepdown; :227 matches index, term and full command and sends captured value |
| failed-persistence-releases-no-success | lib.rs:536 Node::step clones core, executes transition, saves changed Durable, fences on error and releases neither state nor effects; main.rs:193 drains pending on fence |
| restart-replays-only-committed-prefix | lib.rs:175 Core::new validates and applies only durable commit; storage.rs:14 validates loaded state and identity; no timers run in Core::new |
| input-and-storage-ownership | lib.rs:352/:405 validation; storage.rs:14 exclusive fs2 lock and identity check; main.rs Config denies unknown fields |
| bounded-admission | main.rs:54 shared request semaphore; :169 pending <=128 admission check; :326 128 slots; body limit 1MiB; lib.rs MAX_KEY/MAX_VALUE/MAX_BATCH |
| stable-majority-progress; repaired-replica-catches-up | main.rs:126 owner loop includes randomized elections and 100ms heartbeats, reqwest 250ms timeout, one in-flight RPC per peer; lib.rs retries lower prefix and bounded batches |
| conflicting-suffix-repair-reached | lib.rs Append truncation branch is available to deterministic tests but emits no dedicated hosted reachability assertion |

## Boundary findings

**Durability:** save writes state.tmp, syncs it, renames to state.json and syncs the directory (storage.rs:57). Node::step saves before publishing core/effects. If save fails after rename, the on-disk generation may already be newer while in-memory core remains old; fencing prevents additional protocol operations. Restart may legitimately retain that new generation. Pending request errors correctly say unknown outcome. The injectable Persistence trait exists; the actual FileStorage does not have named subphase fault hooks, so a mocked save error alone does not test failures at write/rename/directory-sync boundaries.

**Transport:** outbound tasks run outside the owner and deliver replies with original requests. One in-flight RPC per peer bounds task count; reply delivery may drop on a full channel and later heartbeats/elections retry. Shared request slots mean full client admission can temporarily reject peer RPCs too; this is a recovery/load test target, not demonstrated safety failure. The main event loop uses synchronous whole-state serialization/fsync and clones the complete core per event: accepted simplicity with potentially poor growth behavior, no throughput claim.

**Proposal completion:** waiting identity includes index/term/command and results are captured in Applied, avoiding the two design-stage risks. Pending requests are drained on stepdown before later application effects. A previously committed operation may fail to return to its client after leadership loss; unknown outcome semantics permit this. A fresh request on an isolated leader cannot produce an Applied result unless the core counts a quorum.

**Storage initialization finding and repair:** The first implementation treated missing state.json plus missing state.tmp as fresh even when LOCK/directory already existed. Root treated this as a gap against the fail-closed startup contract. The implementation agent subsequently changed LOCK into a persistent initialization marker, created atomically and synced before initial state creation. Missing state after prior use now returns an error; interrupted initial setup also fails closed. The fresh test agent was asked to add regressions. This addendum is from the orchestrator after the original source-only review; see TEST_REPORT.md for actual validation. Deletion of every marker and state file cannot be distinguished from a new directory and is outside the fault model.

## Missing hosted integration and observability

- No Rust Antithesis SDK dependency/cataloging, assertion calls, or coverage instrumentation.
- No immutable election/application event export for a cross-process monitor; /status is local and insufficient to prove safety.
- No confirmed runnable workload test template, container topology execution, fault-availability confirmation, or hosted result in this source-only review.
- No named real-storage subphase failures or network partition administration hooks in production executable. Deterministic core schedules and process kill/restart cover different boundaries.
- No external linearizability checker in the production source, appropriately: fresh acceptance tests/workload must own that oracle.

## Assumptions and remaining work

Fixed trusted membership and retained durable storage; no Byzantine protection or snapshots. The fresh test agent owns actual coverage outcomes. Map tests to the frozen property plan and disclose any missed HTTP partitions, storage subphase faults, linearizability histories, or hosted properties. Source inspection alone does not establish the invariants.
