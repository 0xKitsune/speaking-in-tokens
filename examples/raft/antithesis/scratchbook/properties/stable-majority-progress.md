# stable-majority-progress

## Evidence trail
Read DESIGN.md R8, PROMPT.md and the three UML sources before implementation. This is a claimed guarantee to test, not an observed defect. No source paths/functions exist yet. The original Raft paper is protocol context; this specific oracle is derived from the local design.

## Property and test
After recovery a stable reachable majority elects and completes a fresh command.

Heal faults, allow timers/network/storage to run, retry bounded fresh put/read until successful; local test deadline is a harness bound, not a protocol timing guarantee.

## Instrumentation status
Missing/proposed: Workload recovery phase; evaluate condition false then true, not Sometimes(true). No existing SDK assertion is duplicated. Keep assertion message `stable-majority-progress` unique. Priority P0; Sometimes: Recovery should produce this meaningful state at least once; finite failures alone do not prove impossible progress.

## Open Questions
None; implementation hook names are pending source creation.
