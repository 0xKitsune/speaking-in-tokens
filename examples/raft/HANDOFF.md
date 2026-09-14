# Workflow record

## Design phase

Primary agent wrote PROMPT.md, task SKILL.md, DESIGN.md and three UML sources before production code. Scope is a static-membership Raft KV service with HTTP JSON, durable state and log-based reads. This phase completed before implementation. No compilation or tests had run at that point.

No additional reference answer arrived after the optional response window. Research proceeded with the provided design and assistant-proposed original Raft paper, explicitly labeled as an assumption.

## Independent research phase

A separate research agent used antithesis-research and wrote TEST_PLAN.md plus a 19-property scratchbook before production code. A fresh reviewer checked the artifacts. Findings added exact client proposal identity and a persistence-failure seam; review corrected the restart oracle to inspect uncommitted replay before new elections can legitimately commit retained entries. The researcher authored neither production code nor tests.

## Design validation

The shared and example SKILL.md files passed skill-creator quick_validate.py (using an isolated uv environment with PyYAML). All three UML sources rendered locally with PlantUML; their PNG previews were inspected and the role layout revised for readability. Committed SVGs are generated views; scripts/render-uml.sh rerenders them with a local Java runtime and PlantUML jar. No private text was sent to a rendering server.

## Implementation agent handoff

Production implementation complete; implementation agent stopped before independent test authorship. Added Cargo package, deterministic core, persistence abstraction and file storage, real HTTP owner/transport, three sample configs and usage README. No existing tests were present; no acceptance tests were authored by this agent and no contract/test-plan expectations were edited.

Checks actually run: `cargo fmt`, `cargo check`, `cargo clippy --all-targets -- -D warnings` (all passed). This establishes formatting/compilation/lints only, not consensus correctness. Fresh test agent must exercise the frozen plan next.

Test seam: `Node<P: Persistence>::step(Event) -> Result<Effects, String>`, with `Persistence::save(&Durable) -> io::Result<()>`. Use a last-successful-state copy for restart and deliberate save errors for fencing. `Node::core` exposes durable state, role, leader, applied index, KV state, and next/matched maps; `Effects` exposes messages, reply, exact proposal identity and captured per-entry results. `Core::new` immediately replays committed state. `Event::{VoteReply,AppendReply}` require peer ID plus the original request and response. Election and heartbeat are explicit events, without internal wall time. `FileStorage::open(path, expected)` enforces locking, validation and identity. Binary accepts one JSON config path; schema/examples and HTTP API are documented in README.

Runtime disk errors fence the owner, suppress effects and fail pending responses with unknown outcome. Outstanding requests expire; replication tasks are capped at one per peer. HTTP startup/config/process behavior and fault schedules remain for independent verification. Deliberate limitations are in DESIGN.md and README; in particular no hosted Antithesis execution or power-loss claim.

### Post-handoff storage repair

Root source review identified that a missing `state.json` in previously opened storage could silently initialize fresh Raft state. That could forget a durable vote/term and violate election safety. The implementation now uses the persistent `LOCK` file as an initialization marker, atomically created with `create_new`, locked, and fsynced (including its directory) before initial state creation. Only its creator may initialize absent state. Existing marker plus missing state fails startup; interrupted first initialization also fails conservatively. The lock is never deleted on close. Public API unchanged; independent regression tests remain owned by the test agent. This is a repair, not an untouched one-shot success or a power-loss certification.

## Orchestrator API smoke

Built the binary and ran an independent temporary Python HTTP driver against three real child processes on ephemeral local ports and separate temporary directories. Passed: election, acknowledged put, SIGKILL of leader, replacement-leader read of acknowledged value, old-node restart, delete/get, full-cluster restart and read, invalid oversized-key rejection. Every child was killed and waited on; temporary state was removed. This was run before the missing-state initialization repair and supplements rather than replaces the independently authored Rust acceptance suite.

## Fresh independent test phase

A fresh test agent read the frozen plan and authored tests/protocol.rs, tests/storage.rs, and tests/http.rs after the implementation handoff. It did not modify production code or the design/test-plan expectations. Final result: 16 passing tests, formatting check passed, strict Clippy passed. See [TEST_REPORT.md](TEST_REPORT.md) for complete property mapping and first-run history.

Coverage includes 48 seeded three/five-node fault schedules (700 scheduled events per seed plus recovery), real HTTP partitions through six directed test proxies, pending proposal replacement, killed leader/full-cluster restart, a small independent concurrent-history checker, durable failure injection, storage corruption and ownership checks, and admission limits.

The HTTP fixture initially assumed status fields remained available during overload; that harness assumption was corrected. Review also strengthened the test oracle's handling of late completion after an unknown-outcome timeout and added oracle self-tests. Neither adjustment changed production requirements.

## Outcome of the skill exercise

The UML/design and independent research plan preceded implementation (commit 937e42c). Independent research clarified proposal identity and restart test semantics. The implementation compiled, then source review found one startup failure-mode gap and the implementation agent repaired it. Independent tests passed against the final implementation. This is a completed workflow exercise with a repair, not proof that UML produced a correct one-shot.

No hosted Antithesis run, SDK integration, power-loss guarantee, snapshots, or dynamic membership is claimed. The two other proposed examples (event bus and resumable storage) have not been built.
