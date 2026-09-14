# one-elected-leader-per-term

## Evidence trail
Read DESIGN.md R1, PROMPT.md and the three UML sources before implementation. This is a claimed guarantee to test, not an observed defect. No source paths/functions exist yet. The original Raft paper is protocol context; this specific oracle is derived from the local design.

## Property and test
At most one node records election victory in a given term.

Collect immutable election events; duplicate/reorder vote responses and reject nonmember voters; audit distinct voter majority.

## Instrumentation status
Missing/proposed: Election transition and configured unique vote set, not asynchronous status snapshots. No existing SDK assertion is duplicated. Keep assertion message `one-elected-leader-per-term` unique. Priority P0; Always: Safety must hold at every evaluated event; also ensure the assertion is encountered.

## Open Questions
None; implementation hook names are pending source creation.
