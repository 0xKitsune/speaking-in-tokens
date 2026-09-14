# restart-replays-only-committed-prefix

## Evidence trail
Read DESIGN.md R5,R7, PROMPT.md and the three UML sources before implementation. This is a claimed guarantee to test, not an observed defect. No source paths/functions exist yet. The original Raft paper is protocol context; this specific oracle is derived from the local design.

## Property and test
Restart rebuilds the KV map from exactly the durable committed prefix and rejects invalid storage.

Persist committed put plus uncommitted delete; inspect replayed map immediately after load before timers/messages can commit more entries. The delete may legitimately commit later through a new leader current-term no-op. Separately test truncated file, impossible commit, mismatched identity.

## Instrumentation status
Missing/proposed: Storage load validation and replay; direct local tests primary, termination coverage additionally needed. No existing SDK assertion is duplicated. Keep assertion message `restart-replays-only-committed-prefix` unique. Priority P0; Always: Safety must hold at every evaluated event; also ensure the assertion is encountered.

## Open Questions
Hosted termination fault availability is unconfirmed; local kill/restart tests are required regardless.
