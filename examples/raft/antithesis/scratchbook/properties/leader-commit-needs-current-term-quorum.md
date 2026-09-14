# leader-commit-needs-current-term-quorum

## Evidence trail
Read DESIGN.md R4, PROMPT.md and the three UML sources before implementation. This is a claimed guarantee to test, not an observed defect. No source paths/functions exist yet. The original Raft paper is protocol context; this specific oracle is derived from the local design.

## Property and test
New leader commit indexes require a durable majority match for a current-term entry.

Reproduce five-node old-term-majority scenario; old-term replication alone does not commit; current-term no-op enables prefix commit.

## Instrumentation status
Missing/proposed: Commit calculation; independently recompute quorum and term from recorded acknowledgments. No existing SDK assertion is duplicated. Keep assertion message `leader-commit-needs-current-term-quorum` unique. Priority P0; Always: Safety must hold at every evaluated event; also ensure the assertion is encountered.

## Open Questions
None; implementation hook names are pending source creation.
