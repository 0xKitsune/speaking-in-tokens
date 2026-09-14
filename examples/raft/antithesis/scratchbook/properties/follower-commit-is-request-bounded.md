# follower-commit-is-request-bounded

## Evidence trail
Read DESIGN.md R3, PROMPT.md and the three UML sources before implementation. This is a claimed guarantee to test, not an observed defect. No source paths/functions exist yet. The original Raft paper is protocol context; this specific oracle is derived from the local design.

## Property and test
Follower commit cannot advance beyond the prefix matched by the received append request.

Follower has divergent tail; deliver empty or short append with large leaderCommit; assert tail not applied.

## Instrumentation status
Missing/proposed: Append commit update with request prev index plus entries length. No existing SDK assertion is duplicated. Keep assertion message `follower-commit-is-request-bounded` unique. Priority P0; Always: Safety must hold at every evaluated event; also ensure the assertion is encountered.

## Open Questions
None; implementation hook names are pending source creation.
