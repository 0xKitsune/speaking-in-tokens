---
name: raft-example
description: Implement and test the standalone Rust Raft key-value example using its UML design and independent research plan.
---

Follow [the shared workflow](../../SKILL.md). Read [PROMPT.md](PROMPT.md), [DESIGN.md](DESIGN.md), and all linked UML views. The research phase writes [TEST_PLAN.md](TEST_PLAN.md) before implementation. Treat the design's property IDs as the behavior contract. Keep implementation and independent test authorship separate. Use conventional Cargo source and integration-test files rather than forcing a network service into a single Rust file.

This example implements its own Raft core. HTTP/serialization/runtime crates are allowed; Raft libraries are not. Keep consensus transitions deterministic and separable from transport, time, and disk so faults can be injected. A single node owner serializes transitions. Preserve externally checkable events or status for the independent test harness.
