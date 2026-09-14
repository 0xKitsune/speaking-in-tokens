# Raft key-value service

Standalone Rust implementation of Raft elections, replication and durable recovery, with an HTTP JSON key-value API. This is the first [Speaking in Tokens](../../SKILL.md) example. Review [the prompt](PROMPT.md), [UML and contract](DESIGN.md), and [independent test plan](TEST_PLAN.md) before the code.

## Run three nodes

From this directory, build once:

```sh
cargo build
```

In three terminals, run:

```sh
target/debug/speaking-raft config/node1.json
target/debug/speaking-raft config/node2.json
target/debug/speaking-raft config/node3.json
```

Each line starts a separate process. The example uses ports 9101–9103 and stores durable data under `data/node1`–`data/node3`, relative to the working directory. Use the same working directory on restart. Each configuration lists all members, including itself. IDs, cluster identity and membership must match stored state. One process holds an exclusive lock per directory. `LOCK` is also a persistent initialization marker: only the process that creates it may initialize a missing `state.json`. The marker is synced before initial state is written and remains after shutdown. If initial setup is interrupted before state is installed, or established state disappears, startup fails instead of resetting term/vote/log. Do not remove the marker to recover an existing member; restore its valid state or rebuild the whole disposable example cluster with a fresh identity. Racing first starts may conservatively fail and require retry; they cannot both initialize.

Find the leader using each node's local diagnostic endpoint:

```sh
curl -s http://127.0.0.1:9101/status
```

Send commands to whichever node reports `"role":"Leader"`:

```sh
curl -s http://127.0.0.1:9101/kv -H 'content-type: application/json' -d '{"op":"put","key":"example","value":"hello"}'
curl -s http://127.0.0.1:9101/kv -H 'content-type: application/json' -d '{"op":"get","key":"example"}'
curl -s http://127.0.0.1:9101/kv -H 'content-type: application/json' -d '{"op":"delete","key":"example"}'
```

Success is `{"index":2,"term":1,"value":"hello"}` (indexes/terms vary). A missing key returns null. Put returns the assigned value; delete returns the previous value. Every command, including get, occupies a replicated log entry. The returned value is captured when that entry is applied.

Followers return HTTP 503 with a leader ID hint when known; clients decide where to retry. HTTP 504 means **unknown outcome**, not rollback. Stepdown or a storage failure can also produce an unknown-outcome 503. Requests are not deduplicated. `/status` reports stale local state, never a linearizable read.

## Run the independent tests

```sh
cargo test --tests
cargo fmt --check
cargo clippy --all-targets -- -D warnings
```

The suite launches and cleans up its own local child processes and ephemeral ports. It includes deterministic protocol fault schedules, storage failures, and HTTP partition/restart scenarios. [Test report](TEST_REPORT.md) records the 16 passing tests, the startup repair, and limits of this evidence.

The checked-in SVG diagrams render directly in [DESIGN.md](DESIGN.md). To regenerate them locally from PlantUML sources, set `PLANTUML_JAR` to a local jar (and optionally `JAVA_BIN`) and run `../../scripts/render-uml.sh`. The existing diagrams were rendered with PlantUML 1.2026.8 and Java 21 using its bundled Smetana layout, without an external rendering server.

## Implementation boundaries

- `src/lib.rs`: deterministic consensus core and durability-before-effects wrapper.
- `src/storage.rs`: complete state file, file sync, atomic rename, directory sync, startup validation and exclusive directory lock.
- `src/main.rs`: single serialized owner, HTTP handlers, election timers, peer transport, bounded pending client responses.

Public simulation interface: construct `Durable::empty(cluster, id, members)`; provide a `Persistence` implementation; create `Node::new(state, storage)` and call `step(Event)`. Events include explicit election/heartbeat timeouts, proposals, vote/append RPCs and correlated replies. `Effects` carries outbound requests, inbound response, exact proposal identity, applied entry results, election reset and stepdown flags. Deliver outbound requests to another node and return its reply with the original request. To restart, construct a fresh node from the last successfully saved `Durable` copy. A failing `save` fences the node and releases no effects. The core's readable fields support diagnostics; callers must not mutate them in a running service.

Timers use 600–999 ms randomized elections and 100 ms heartbeats. Peer HTTP timeout is 250 ms; at most one RPC per peer is in flight. At most 128 admitted HTTP consensus/client calls and 128 pending client commands, with a bounded 256-item owner mailbox. Client waiters expire after 2.8 seconds; HTTP waits expire after 3 seconds. Timed-out log entries remain eligible to commit. HTTP bodies are limited to 1 MiB, keys to 1 KiB, values to 16 KiB, replication batches to 32 entries.

## Limits and evidence

Static membership of 1, 3 or 5 nodes. No snapshots/compaction, membership changes, pre-vote, check-quorum, lease reads, exactly-once clients, TLS/authentication, Byzantine protection or production performance claim. Disk/log growth is unbounded, and complete-state persistence is intentionally expensive. Peer endpoints assume a trusted local/test network. File durability relies on filesystem fsync/rename behavior; process-kill tests cannot establish power-loss safety.

The independent Antithesis research artifacts describe properties and a future fault-testing environment. They are not evidence of a hosted Antithesis run. Actual validation and repairs belong in [HANDOFF.md](HANDOFF.md) and the fresh test agent's test report.
