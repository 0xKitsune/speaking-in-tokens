# conflicting-suffix-repair-reached

## Evidence trail
Read DESIGN.md R3, PROMPT.md and the three UML sources before implementation. This is a claimed guarantee to test, not an observed defect. No source paths/functions exist yet. The original Raft paper is protocol context; this specific oracle is derived from the local design.

## Property and test
Exercise actual conflicting uncommitted suffix replacement.

Minority leader appends uncommitted entry, majority commits competing entry, heal; record truncate-and-replace branch.

## Instrumentation status
Missing/proposed: SUT conflict replacement event; this is pure branch reachability, not a correctness assertion. No existing SDK assertion is duplicated. Keep assertion message `conflicting-suffix-repair-reached` unique. Priority P1; Reachable: The branch itself is the desired coverage event.

## Open Questions
None; implementation hook names are pending source creation.
