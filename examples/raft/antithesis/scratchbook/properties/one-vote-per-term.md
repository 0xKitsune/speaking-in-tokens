# one-vote-per-term

## Evidence trail
Read DESIGN.md R1, PROMPT.md and the three UML sources before implementation. This is a claimed guarantee to test, not an observed defect. No source paths/functions exist yet. The original Raft paper is protocol context; this specific oracle is derived from the local design.

## Property and test
A node grants at most one distinct candidate a vote per term, including across restarts.

Record granted votes by (node,term) outside SUT; grant A, crash after reply, restart, ask for B in same term.

## Instrumentation status
Missing/proposed: Vote grant and durable-save boundary; term/self-vote persistence before outgoing election requests. No existing SDK assertion is duplicated. Keep assertion message `one-vote-per-term` unique. Priority P0; Always: Safety must hold at every evaluated event; also ensure the assertion is encountered.

## Open Questions
Hosted termination fault availability is unconfirmed; local kill/restart tests are required regardless.
