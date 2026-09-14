# repaired-replica-catches-up

## Evidence trail
Read DESIGN.md R3,R8, PROMPT.md and the three UML sources before implementation. This is a claimed guarantee to test, not an observed defect. No source paths/functions exist yet. The original Raft paper is protocol context; this specific oracle is derived from the local design.

## Property and test
A restored lagging replica eventually applies an acknowledged target prefix during recovery.

Record a target committed index and command digest, restore partitioned replica, stop new workload and poll until it contains/applies that target.

## Instrumentation status
Missing/proposed: Targeted application event; do not require identical latest maps while writes continue. No existing SDK assertion is duplicated. Keep assertion message `repaired-replica-catches-up` unique. Priority P1; Sometimes: Recovery should produce this meaningful state at least once; finite failures alone do not prove impossible progress.

## Open Questions
None; implementation hook names are pending source creation.
