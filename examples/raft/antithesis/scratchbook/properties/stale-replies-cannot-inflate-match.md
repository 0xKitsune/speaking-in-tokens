# stale-replies-cannot-inflate-match

## Evidence trail
Read DESIGN.md R2,R4, PROMPT.md and the three UML sources before implementation. This is a claimed guarantee to test, not an observed defect. No source paths/functions exist yet. The original Raft paper is protocol context; this specific oracle is derived from the local design.

## Property and test
Acknowledged match indexes do not exceed the sent request prefix/entries and obsolete responses cannot advance leadership state.

Success on heartbeat with long follower tail; delayed failure after newer success; delayed old-term success after reelection.

## Instrumentation status
Missing/proposed: Correlated outbound request context and reply processing. No existing SDK assertion is duplicated. Keep assertion message `stale-replies-cannot-inflate-match` unique. Priority P0; Always: Safety must hold at every evaluated event; also ensure the assertion is encountered.

## Open Questions
None; implementation hook names are pending source creation.
