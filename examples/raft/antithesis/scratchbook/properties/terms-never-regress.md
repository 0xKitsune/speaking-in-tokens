# terms-never-regress

## Evidence trail
Read DESIGN.md R2, PROMPT.md and the three UML sources before implementation. This is a claimed guarantee to test, not an observed defect. No source paths/functions exist yet. The original Raft paper is protocol context; this specific oracle is derived from the local design.

## Property and test
Each node term never decreases and higher-term replies fence old leadership before further effects.

Replay old and higher-term vote/append responses after a new election; compare durable/restarted term history.

## Instrumentation status
Missing/proposed: RPC/reply ingestion and persistent term transition. No existing SDK assertion is duplicated. Keep assertion message `terms-never-regress` unique. Priority P0; Always: Safety must hold at every evaluated event; also ensure the assertion is encountered.

## Open Questions
None; implementation hook names are pending source creation.
