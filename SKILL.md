---
name: speaking-in-tokens
description: Produce one implementation-ready PLAN.md containing a UML design, explicit constraints and invariants, and independently researched test cases in a fixed format. Use when turning a substantial engineering request into a plan an agent can implement.
---

# Speaking in Tokens

Produce exactly **one planning artifact: `PLAN.md`**. It is the complete contract for the work. Use the form below, with the same numbered headings in the same order every time. Scale the contents to the task; keep the form stable.

Do not generate prompt transcripts, task-specific skills, separate design/test plans, scratchbooks, property files, reports, handoff files, diagram sidecars, or rendering scripts. Keep UML inline in fenced `plantuml` blocks. Research findings belong in the relevant sections of the plan, not in an appendix of process history. Source and test files are created only when implementation is authorized; they are not planning artifacts.

## How to fill the form

1. Read the user's request and relevant existing code. Turn ambiguous language into concrete behavior. Infer routine choices and label meaningful assumptions. Ask only about unresolved choices that change required behavior; no extra approval is needed when implementation is already authorized.
2. Fill sections 1–4. Use UML for ownership, interactions and state transitions. Include guards and effects, not just boxes and arrows. English explains constraints and invariants. Use stable IDs (`C1`, `I1`, `T1`) to connect them.
3. Before implementation, have a **separate research agent run `antithesis-research`** against the proposed design and available source. It fills/refines section 5 in this same file and flags design gaps in section 6. The user's single-file requirement overrides that dependency's normal scratchbook/file layout: return findings to the orchestrator or edit PLAN.md only. If greenfield, clearly distinguish proposed properties from observed source behavior. If the dependency is unavailable, report the missing pass; never label a generic review as Antithesis research. No hosted execution is implied.
4. Check that every invariant has test coverage, failure scenarios have expected outcomes, and no blocking semantic decision is hidden. Set readiness accurately. Stop after the plan for a planning-only request. If implementation was requested, the implementation agent can now start directly from this file.
5. For the implementation/testing workflow: implementation agent writes production code, runs existing checks, then stops; a fresh test agent implements section 5. Neither may weaken requirements to make code pass. Resolve defects against the plan. Put any necessary contract correction or brief verification result in section 6, not a new document. Do not narrate every tool call or retain discarded approaches.

## Rust implementation standard

For Rust tasks, use the guidance below when filling the plan and implementing it. It adapts [Apollo's rust-best-practices skill](https://github.com/apollographql/skills/blob/main/skills/rust-best-practices/SKILL.md), with the official [Rust Style Guide](https://doc.rust-lang.org/style-guide/), [API Guidelines](https://rust-lang.github.io/api-guidelines/), and [Cargo layout](https://doc.rust-lang.org/cargo/guide/project-layout.html) as references. These are conventions plus engineering judgment, not a universal architecture. Follow existing repository/toolchain requirements first.

- **Architecture and files (section 2):** identify modules, their responsibilities, dependency direction, state owners and I/O boundaries. Use standard Cargo layout: `src/lib.rs` for reusable logic, `src/main.rs` for executable wiring, cohesive domain modules when needed, integration tests under `tests/`, private unit tests beside their code. Keep the smallest layout that makes those boundaries clear; do not create a file per type or layers named manager/service/factory without a concrete need. Existing layouts need no cosmetic reorganization.
- **Types and ownership (sections 2–3):** keep internals private and expose the smallest useful API. Prefer enums/newtypes when they prevent invalid states or mixed identifiers. Use concrete types first; introduce traits for actual substitution, including fault injection. Borrow when ownership is unnecessary; explain substantial cloning or shared mutation. For async systems, define task ownership, cancellation, shutdown, bounded queues and lock scope. Avoid blocking runtime workers and holding locks across network waits.
- **Errors and safety (sections 3–4):** specify which errors callers must distinguish and how they map to API outcomes. Use `Result` and preserve causes; typed errors fit library boundaries, contextual reporting fits applications. Do not require an error crate without need. External input, disk and network failures must not rely on `unwrap`/`expect`. Any deliberate invariant panic needs justification. Prefer safe Rust; any necessary unsafe block needs its safety obligations documented and checked.
- **Reviewability (sections 5–6):** document public behavior, errors and important invariants with rustdoc; comments explain non-obvious reasons. Tests cover behavior and failure modes, with enough assertions to establish the property. Prefer direct code over premature generic frameworks; optimize after measurement.
- **Tooling (section 6):** record the repository's edition, toolchain/MSRV policy, supported feature combinations and exact validation commands. Use stable rustfmt defaults unless the repository config says otherwise. Fix Clippy findings; narrowly justified lint exceptions only, using `#[expect]` when supported. Do not enable every pedantic/nursery lint, update the toolchain, or add dependencies merely to satisfy an external style preference.

Default checks for a Cargo workspace with a committed lockfile:

```sh
cargo fmt --all -- --check
cargo check --workspace --all-targets --locked
cargo test --workspace --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --locked
```

Adapt commands to the actual project. Check supported non-default feature combinations explicitly; `--all-features` does **not** resolve mutually exclusive features. Include doctests where applicable. A new project generates its lockfile before locked checks. Completion includes a human-comprehension review of the module boundaries and public API, not just green lints. Record unrun checks honestly. All these decisions remain inside the same six-section PLAN.md.

## Required form

````markdown
# <Work title>

## 1. Outcome and scope
<What the system/change must do. In scope, excluded scope, and meaningful assumptions. No copied prompt or conversation history.>

## 2. Design
<Components, module/file responsibilities, dependency direction, ownership, data flow, state, persistence, and concurrency. Include the smallest useful UML view(s) inline. Use event [guard] / effect for transitions.>

```plantuml
@startuml
<task-specific UML>
@enduml
```

## 3. Interfaces and constraints
| ID | Interface or constraint | Required behavior |
| --- | --- | --- |
| C1 | <API / boundary> | <Inputs, outputs, limits, errors, retries, ordering, compatibility as relevant> |

## 4. Invariants
| ID | Property | Applies when |
| --- | --- | --- |
| I1 | <Precisely what must always hold, or eventually happen> | <Scope and failure/progress assumptions> |

## 5. Test cases
Research pass: <pending / completed; scope and any material limits in one sentence>.

| ID | Covers | Setup and action / fault schedule | Expected result | Test method |
| --- | --- | --- | --- | --- |
| T1 | C1, I1 | <Concrete scenario> | <Observable pass/fail outcome> | <Unit / simulation / process / integration / hosted> |

<Test seams and independent oracle needed to implement these cases. Define liveness recovery conditions and timeout semantics. Proposed tests are not passing results.>

## 6. Implementation handoff
Readiness: <ready / blocked, with specific unresolved decisions>.

<Short implementation sequence, required test hooks, commands/checks, and completion criteria. Include references needed to implement the contract. Brief verification status may be added after work; no separate report.>
````

Use an outer fence longer than the embedded UML fence when presenting the entire form as a code block. The form above describes the structure; do not leave angle-bracket placeholders in an actual plan. If a category is irrelevant, write one short reason under its heading instead of manufacturing requirements.

UML and English define the intended behavior; they do not prove the implementation correct. Tests should check observable outcomes, including forbidden traces, rather than duplicate implementation logic. Distinguish local tests, simulated faults and hosted Antithesis runs in the brief verification status.

Example: [Raft plan](examples/raft/PLAN.md).
