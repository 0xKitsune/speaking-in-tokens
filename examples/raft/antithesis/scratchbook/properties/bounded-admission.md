# bounded-admission

## Evidence trail
Read DESIGN.md R9, PROMPT.md and the three UML sources before implementation. This is a claimed guarantee to test, not an observed defect. No source paths/functions exist yet. The original Raft paper is protocol context; this specific oracle is derived from the local design.

## Property and test
Oversized or excess admitted client work cannot bypass configured size/count limits.

Send oversized body/key/value and flood requests during unavailable quorum; inspect pending count and rejection path before append.

## Instrumentation status
Missing/proposed: Admission counters and request body parser; log growth is explicitly excluded. No existing SDK assertion is duplicated. Keep assertion message `bounded-admission` unique. Priority P1; Always: Safety must hold at every evaluated event; also ensure the assertion is encountered.

## Open Questions
None; implementation hook names are pending source creation.
