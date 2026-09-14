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

# Minimal topology proposal

| Container | Source / role | Connections / durable state |
|---|---|---|
| node-a, node-b, node-c | New Rust multi-stage image, same executable, three service containers | Each talks HTTP to other two; separate persistent `/data` per node |
| workload | New Rust test-driver image, one client container | Talks to all nodes; owns client operation history and independent checker |

Three replicas suffice for partitions/failover. Five-node Figure-8-style histories belong in deterministic tests initially, with an optional five-service run later. No external dependency service or shared database. Container definitions are future setup work, not delivered/validated by this design research.

## Runtime contract
Node CLI must configure id, complete membership/addresses, bind address and durable path. Resolve service names after restarts; never assume fixed IPs. Each node exclusively owns its directory. Real HTTP serialization, disk sync and process startup are exercised. Client observes `/kv` responses; `/status` only locates candidates and reports progress, never proves linearizability.

## Workload and SDK
Future Rust workload embeds Antithesis SDK assertion cataloging. Startup waits for reachable node processes, emits setup_complete and remains alive; do not emit readiness from a first_ command. Template `/opt/antithesis/test/v1/raft/` can contain parallel_driver_commands, serial_driver_recovery, eventually_progress and finally_history (executables). Helper files use helper_ prefix. Per-client append-only histories avoid concurrent file writers. Store invocation, response/timeout, unique operation id, command and result; model unknown outcomes as optional operations, not definite failures.

## Faults and recovery
Network partitions/delay/asymmetry stress transport. Kill/restart requires termination faults enabled explicitly or an agreed custom mechanism; tenant availability is not known. Disk I/O error injection is a separate SUT test seam/custom fault, not assumed provided by ordinary process kills. Keep durable directories across restart. Recovery phases cease writes, restore majority, and test a target prefix; use eventual terminal checks or a stop-faults window for mid-run recovery. Do not conclude safety from unavailable reads.

Local Rust process tests provide HTTP/disk smoke coverage. Hosted Antithesis execution, coverage instrumentation, image publishing, and tenant configuration are deferred; no such run has occurred.

## Assumptions
Static trusted 1/3/5-node membership; stable durable storage survives process death; no Byzantine peers, snapshots, migration, or exactly-once clients. Claims here are proposed requirements, not verified implementation facts.

## Open Questions
No external-reference answer was received after the optional response window. The provided design and original Raft paper are the stated assistant-assumed scope; no user confirmation is inferred. Hosted Antithesis access and enabled termination faults are unconfirmed; this pass neither provisions nor executes a hosted run.
