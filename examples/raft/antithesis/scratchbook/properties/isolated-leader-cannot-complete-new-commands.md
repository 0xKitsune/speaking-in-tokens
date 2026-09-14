# isolated-leader-cannot-complete-new-commands

## Evidence trail
Read DESIGN.md R6, PROMPT.md and the three UML sources before implementation. This is a claimed guarantee to test, not an observed defect. No source paths/functions exist yet. The original Raft paper is protocol context; this specific oracle is derived from the local design.

## Property and test
A freshly submitted operation on an isolated leader cannot succeed without durable quorum replication.

Commit new value through majority, then submit new get/put on isolated former leader; require unavailable/unknown outcome, never stale success.

## Instrumentation status
Missing/proposed: Workload partition control and request start boundaries; operations already committed before isolation may complete. No existing SDK assertion is duplicated. Keep assertion message `isolated-leader-cannot-complete-new-commands` unique. Priority P0; Always: Safety must hold at every evaluated event; also ensure the assertion is encountered.

## Open Questions
None; implementation hook names are pending source creation.
