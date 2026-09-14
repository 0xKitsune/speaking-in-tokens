# input-and-storage-ownership

## Evidence trail
Read DESIGN.md R9, PROMPT.md and the three UML sources before implementation. This is a claimed guarantee to test, not an observed defect. No source paths/functions exist yet. The original Raft paper is protocol context; this specific oracle is derived from the local design.

## Property and test
Malformed/nonmember input cannot mutate protocol state; a second process cannot own the same data directory.

Snapshot durable state then malformed/unknown-sender RPC; launch competing owner; restart with changed membership.

## Instrumentation status
Missing/proposed: HTTP validation, config validation and directory lock; primarily ordinary API/process tests. No existing SDK assertion is duplicated. Keep assertion message `input-and-storage-ownership` unique. Priority P1; Always: Safety must hold at every evaluated event; also ensure the assertion is encountered.

## Open Questions
None; implementation hook names are pending source creation.
