# failed-persistence-releases-no-success

## Evidence trail
Read DESIGN.md R7, PROMPT.md and the three UML sources before implementation. This is a claimed guarantee to test, not an observed defect. No source paths/functions exist yet. The original Raft paper is protocol context; this specific oracle is derived from the local design.

## Property and test
A failed durable save prevents dependent RPC/client successes and future consensus activity until restart.

Inject errors before write, after temp write, rename and directory sync; record emitted effects and fenced state; restart from whichever complete generation exists.

## Instrumentation status
Missing/proposed: Injectable storage and owner effect release boundary; filesystem corruption is separate from process crash. No existing SDK assertion is duplicated. Keep assertion message `failed-persistence-releases-no-success` unique. Priority P0; Always: Safety must hold at every evaluated event; also ensure the assertion is encountered.

## Open Questions
None; implementation hook names are pending source creation.
