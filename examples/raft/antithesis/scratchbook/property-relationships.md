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

# Property relationships

- Election/term cluster: `one-vote-per-term`, `vote-requires-current-log`, `terms-never-regress` support `one-elected-leader-per-term`. End-to-end leader observations do not replace durable vote checks.
- Replication cluster: `matching-prefix-is-preserved`, `follower-commit-is-request-bounded`, `stale-replies-cannot-inflate-match`, `leader-commit-needs-current-term-quorum` support `same-index-same-applied-command`. `conflicting-suffix-repair-reached` guards against unexercised recovery.
- Client cluster: `client-results-match-proposals`, `isolated-leader-cannot-complete-new-commands`, `acknowledged-operations-survive`. Correct log agreement alone does not imply correct HTTP responses.
- Persistence cluster: `failed-persistence-releases-no-success`, `restart-replays-only-committed-prefix`, `input-and-storage-ownership` support durable election and client properties.
- Recovery/resource cluster: `bounded-admission`, `stable-majority-progress`, `repaired-replica-catches-up`. Overload must not permanently prevent recovery; eventual progress does not excuse safety failures.

These are hypothesized dependencies for triage, not formal proofs or permission to remove overlapping checks.

## Assumptions
Static trusted 1/3/5-node membership; stable durable storage survives process death; no Byzantine peers, snapshots, migration, or exactly-once clients. Claims here are proposed requirements, not verified implementation facts.

## Open Questions
No external-reference answer was received after the optional response window. The provided design and original Raft paper are the stated assistant-assumed scope; no user confirmation is inferred. Hosted Antithesis access and enabled termination faults are unconfirmed; this pass neither provisions nor executes a hosted run.
