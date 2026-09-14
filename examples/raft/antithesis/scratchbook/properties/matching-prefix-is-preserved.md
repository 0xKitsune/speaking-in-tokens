# matching-prefix-is-preserved

## Evidence trail
Read DESIGN.md R3, PROMPT.md and the three UML sources before implementation. This is a claimed guarantee to test, not an observed defect. No source paths/functions exist yet. The original Raft paper is protocol context; this specific oracle is derived from the local design.

## Property and test
Matching entries are retained until an actual uncommitted conflict; committed entries never change.

Deliver shorter delayed matching append after longer append, then a true conflicting suffix; compare committed entries against immutable history.

## Instrumentation status
Missing/proposed: Append conflict/truncation branch; capture old committed prefix. No existing SDK assertion is duplicated. Keep assertion message `matching-prefix-is-preserved` unique. Priority P0; Always: Safety must hold at every evaluated event; also ensure the assertion is encountered.

## Open Questions
None; implementation hook names are pending source creation.
