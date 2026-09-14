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

# Fresh design-stage self-review

Result: no remaining blocker for this bounded pre-implementation research handoff after the two refinements below. This is not implementation acceptance or a hosted Antithesis result.

## Scope and checks

Read the antithesis-research skill and property-catalog conventions, TEST_PLAN.md, DESIGN.md, all three design/*.puml sources, all 19 property evidence files, and all top-level scratchbook/evaluation artifacts. Reviewed against the skill self-review criteria as a targeted greenfield design-stage pass. Production source was absent at the research baseline; concurrent implementation is outside this review. Baseline instrumentation claims are explicitly temporal and must be rescanned during source-stage work.

The supplied parent context says the scope question was asked and no answer arrived after the optional response window. The artifacts accurately record the original Raft paper and local design as assistant-assumed scope. Silence is not user confirmation. The provenance here is copied from synthesis; this review does not independently certify prior documentation retrieval.

Structural checks found 19 catalog slugs and 19 matching evidence files, no unmatched relationship references, and provenance frontmatter in every top-level artifact. The analysis covers architecture, state, serialization, persistence, concurrency and failure boundaries. Each property names priority, assertion type and rationale, an oracle/fault trace and proposed instrumentation. Open questions mirror their evidence files; there are no unsupported partial/needs-human-input tags or purported confirmed defects. Relationship clusters preserve complementary internal and external checks.

## Findings and disposition

1. **Correctness of replay oracle — resolved during review.** `properties/restart-replays-only-committed-prefix.md:9` originally instructed a quorum read after restart to check that an uncommitted delete had not replayed. The delete can legitimately commit later through the elected leader's current-term no-op, so that read cannot isolate startup replay. The researcher updated both evidence and catalog to inspect the reconstructed map immediately after load, before timers/messages, and explicitly permit later lawful commit. Re-read and verified this correction.

2. **R5 coverage gap — resolved during review.** `properties/same-index-same-applied-command.md:7` originally checked only cross-node command agreement. That would not detect duplicate application, skipped indexes or application beyond commit. TEST_PLAN already required exact application order; the researcher added per-process-incarnation consecutive application and `applied <= commit <= log length` to the catalog and evidence. Re-read and verified this correction. Restart replay remains permitted across distinct incarnations.

## Handoff quality and limits

The protocol scenarios are useful and concrete: durable votes, distinct configured quorums, request-bounded follower commit, current-term leader commit, delayed replies, overwritten waiters, unknown client outcomes and injected persistence failure. The external linearizability oracle uses invocation/response precedence and optional timeout completions rather than treating SUT indexes as its proof. Three service containers plus one workload are a justified minimal topology; five-node histories remain deterministic tests initially.

Implementation hook names, hosted SDK/build integration, container definitions and enabled termination faults remain explicit future work. Their absence is appropriate for this design-stage task. Setup/workload agents can use the proposed interfaces and scenarios, but must resolve exact APIs and runtime configuration against the implementation before claiming executable readiness. The selected lenses were evaluated sequentially; this fresh review does not claim a full ensemble or whole-SUT implementation review. No Rust was edited and no protocol tests were run by this reviewer.
