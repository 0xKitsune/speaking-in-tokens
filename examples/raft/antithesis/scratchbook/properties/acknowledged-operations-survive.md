# acknowledged-operations-survive

## Evidence trail
Read DESIGN.md R6,R7, PROMPT.md and the three UML sources before implementation. This is a claimed guarantee to test, not an observed defect. No source paths/functions exist yet. The original Raft paper is protocol context; this specific oracle is derived from the local design.

## Property and test
Successful client operations remain in the committed history after crash/replacement.

Unique key put, receive success, kill leader, elect successor, quorum read; repeat full cluster restart retaining directories.

## Instrumentation status
Missing/proposed: Workload success history and independent KV oracle; later overwrites are accounted for, never assert a superseded value remains current. No existing SDK assertion is duplicated. Keep assertion message `acknowledged-operations-survive` unique. Priority P0; Always: Safety must hold at every evaluated event; also ensure the assertion is encountered.

## Open Questions
Hosted termination fault availability is unconfirmed; local kill/restart tests are required regardless.
