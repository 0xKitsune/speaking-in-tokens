# vote-requires-current-log

## Evidence trail
Read DESIGN.md R1, PROMPT.md and the three UML sources before implementation. This is a claimed guarantee to test, not an observed defect. No source paths/functions exist yet. The original Raft paper is protocol context; this specific oracle is derived from the local design.

## Property and test
Granted candidates have a lexicographically current enough last-log term/index.

Construct longer old-term log and shorter newer-term log; send vote requests in both directions.

## Instrumentation status
Missing/proposed: Vote decision; deterministic tests are primary, network delay adds composition coverage. No existing SDK assertion is duplicated. Keep assertion message `vote-requires-current-log` unique. Priority P0; Always: Safety must hold at every evaluated event; also ensure the assertion is encountered.

## Open Questions
None; implementation hook names are pending source creation.
