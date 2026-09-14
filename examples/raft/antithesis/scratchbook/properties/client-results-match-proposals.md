# client-results-match-proposals

## Evidence trail
Read DESIGN.md R6, PROMPT.md and the three UML sources before implementation. This is a claimed guarantee to test, not an observed defect. No source paths/functions exist yet. The original Raft paper is protocol context; this specific oracle is derived from the local design.

## Property and test
A successful response belongs to the original proposal and returns its result at that log position.

Old leader proposal times out/steps down; another command replaces index; old waiter must not succeed for replacement. Batch put/get/delete/put before handlers resume to test result capture.

## Instrumentation status
Missing/proposed: Waiter registration/completion and apply results keyed by original proposal identity. No existing SDK assertion is duplicated. Keep assertion message `client-results-match-proposals` unique. Priority P0; Always: Safety must hold at every evaluated event; also ensure the assertion is encountered.

## Open Questions
None; implementation hook names are pending source creation.
