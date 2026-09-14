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

# Evaluation synthesis

This is bounded design-stage targeted research. Discovery lenses were considered sequentially; a fresh agent review follows separately because the implementation does not exist and agent slots are limited.

- **Gaps filled:** per-proposal response identity; resource admission; corrupted/identity-mismatched startup.
- **Refinements applied:** distinguish asynchronous status from immutable events; bound follower commit by request; local deterministic tests for parser/comparator logic; qualify termination availability and quiet-period progress.
- **Bias disclosed:** deterministic-core observability makes internal safety easier to evaluate than production HTTP timing. TEST_PLAN requires real processes, disk, HTTP and an external oracle to counter this; hosted exploration remains deferred.
- **Unresolved:** user-supplied additional references (none received), implementation hooks, hosted access/fault availability. None is represented as an existing passing test.

Fresh review completed; see fresh-review.md. Applied its replay-oracle correction: inspect immediately after load, since later quorum activity may legitimately commit a retained suffix. Also made per-incarnation consecutive application and index bounds explicit in the safety catalog. No production Rust or tests were written by this research pass.

## Assumptions
Static trusted 1/3/5-node membership; stable durable storage survives process death; no Byzantine peers, snapshots, migration, or exactly-once clients. Claims here are proposed requirements, not verified implementation facts.

## Open Questions
No external-reference answer was received after the optional response window. The provided design and original Raft paper are the stated assistant-assumed scope; no user confirmation is inferred. Hosted Antithesis access and enabled termination faults are unconfirmed; this pass neither provisions nor executes a hosted run.
