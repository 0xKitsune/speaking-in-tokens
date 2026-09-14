# same-index-same-applied-command

## Evidence trail
Read DESIGN.md R5, PROMPT.md and the three UML sources before implementation. This is a claimed guarantee to test, not an observed defect. No source paths/functions exist yet. The original Raft paper is protocol context; this specific oracle is derived from the local design.

## Property and test
Any two application events at the same index carry the same command and term; within each process incarnation application indexes advance consecutively and applied <= commit <= log length.

Partition, elect replacement, repair suffix, restart; independent index-to-command ledger rejects disagreement.

## Instrumentation status
Missing/proposed: Apply event with index/term/command; replay across incarnations is allowed if identical. No existing SDK assertion is duplicated. Keep assertion message `same-index-same-applied-command` unique. Priority P0; Always: Safety must hold at every evaluated event; also ensure the assertion is encountered.

## Open Questions
None; implementation hook names are pending source creation.
