---
name: speaking-in-tokens
description: Guide substantial engineering work through context gathering, design discussion, one Mermaid PLAN.md, explicit user greenlight, implementation, independent Antithesis research, fresh-agent tests, and review. Use when the user wants a reviewable design-to-implementation workflow.
---

# Speaking in Tokens

Produce exactly **one planning artifact: `PLAN.md`**. It is the complete contract for the work. Use the form below, with the same numbered headings in the same order every time. Scale the contents to the task; keep the form stable.

Do not generate prompt transcripts, task-specific skills, separate design/test plans, scratchbooks, property files, reports, handoff files, diagram sidecars, or rendering scripts. Keep diagrams inline in fenced `mermaid` blocks so the plan renders in Markdown in the target browser. Human readability is the primary design criterion. Research findings belong in the relevant sections of the plan, not in an appendix of process history. Source and test files are created only when implementation is authorized; they are not planning artifacts.

## Workflow: understand, agree, build, test, review

1. **Gather context.** Read the request, relevant source, existing tests, interfaces and operational constraints. Investigate questions that the repository can answer before asking the user. Identify actors, data, relationships, ownership, failure modes and intended outcomes.
2. **Grill the design with the user.** Follow `grill-me`: ask one focused question at a time, include your recommended answer, and resolve dependent decisions in order. Work through consequential ambiguity, tradeoffs, limits and failure/recovery semantics until there is shared understanding. Do not turn this into a generic questionnaire or re-ask settled decisions. If the skill is unavailable, disclose that and use this stated interview behavior.
3. **Write the single plan.** Fill the six-section form below, using inline Mermaid plus precise English constraints and invariants. The plan is the shared review surface for flows, data relationships, actors and components. Section 5 includes proposed concrete test cases and coverage; mark the post-implementation Antithesis research as pending. Keep the plan up to date as the user resolves questions.
4. **Wait for the user's greenlight.** Present/link the completed PLAN.md and stop before implementation. Set section 6 to `awaiting user greenlight`. A general request to build something is not approval of an unseen plan in this workflow. An explicit greenlight for the current plan is sufficient; do not ask again. Material changes to the agreed contract return to discussion and approval before dependent work.
5. **Implement the approved plan.** An implementation agent writes production code and the agreed test seams, runs formatting, compilation, lints and existing tests, records the brief handoff in section 6, then stops. It must not author or weaken the independent acceptance suite to fit its code.
6. **Run independent Antithesis research against the implementation.** A separate agent reads and runs `antithesis-research`, using the approved plan and actual source. It maps invariants and failure-prone paths, proposes concrete unit, property-based, simulation, process and Antithesis workloads/assertions as appropriate, and updates section 5 with fault schedules, observables, oracles and test hooks. Flag contract gaps in section 6. The single-file requirement overrides the research skill's scratchbook layout: return findings to the orchestrator or edit PLAN.md only. If the skill is missing, report the missing pass; do not relabel ordinary testing as Antithesis research.
7. **Have a fresh agent write and run the tests.** Give another agent the approved plan, completed research and source. It implements the test cases and required Antithesis integration within the agreed scope. Expected behavior comes from the contract; inspect source only to connect fixtures and instrumentation. Return defects for repair; change a test expectation only when demonstrably wrong against the contract. Re-run affected checks after repairs.
8. **Prepare for review.** Inspect the complete diff against the approved plan, invariants, Rust standards and test coverage. Section 6 records checks actually run, failures repaired, remaining gaps, and review readiness. Separate local property tests, fault simulation and hosted Antithesis results. Missing required coverage or execution means blocked/incomplete, not ready. All reporting stays in PLAN.md; no separate report or PR is required unless requested.

## Test contract for Rust

Section 5 must specify executable cases, not just a list of tools. For each applicable layer, identify covered C/I IDs, setup, actions or generated inputs, observable assertions and replay/reproduction method. Explain any inapplicable layer briefly.

- **Unit and regression tests:** use Rust `#[test]`, private `#[cfg(test)] mod tests` where appropriate, and integration tests under `tests/`. Use the runtime's test attribute (for example `#[tokio::test]`) for async tests. Assert success, boundary inputs and failure behavior; retain a regression for repaired defects.
- **Property-based tests:** use `proptest!` or an equivalent justified framework for generated inputs and operation sequences. Define generators, preconditions, independent oracle, shrinking strategy and replayable failures. Specify the actual universal property. A handful of randomized examples is not a substitute for these requirements.
- **Stateful and fault tests:** exercise ordering, duplication, partitions, cancellation, restart, persistence failures and resource bounds when relevant. State the fault model, forbidden traces and liveness assumptions. Preserve seeds/traces and minimize failing histories.
- **Antithesis:** research concrete safety, liveness and reachability assertions; propose SDK instrumentation, workload, topology, setup and a runnable local smoke check where applicable. Distinguish proposed hooks from installed hooks, and local execution from a hosted campaign. Include the required integration and execution scope in the plan before greenlight; use current `antithesis-documentation` guidance for integration. A hosted run requires configured access and authorization and cannot be inferred from a research pass. If agreed hosted validation is unavailable, report that specific review blocker.

Tests must be independent enough to detect a wrong implementation; do not copy its decision logic into the oracle. Planned cases are not passing results. The fresh test agent owns test implementation, including property tests and agreed Antithesis hooks/workloads.

## Diagrams humans can review

Use the full range of [Mermaid diagram types](https://mermaid.js.org/intro/), choosing whichever communicates the contract most clearly. Do not default everything to a flowchart or require every type in every plan.

| What needs explaining | Useful diagram type |
| --- | --- |
| Message order, request/response, retries, concurrency, persistence before acknowledgment | `sequenceDiagram`; use `alt`, `opt`, `loop`, `par` and notes where they express real behavior |
| Lifecycle, allowed transitions, failure/recovery, guards and effects | `stateDiagram-v2`; label transitions `event [guard] / effect` |
| Components, ownership boundaries, dependencies, decision paths | `flowchart` with subgraphs; architecture, block or C4 views when supported by the target renderer |
| Types, interfaces, composition and cardinality | `classDiagram`; show meaningful relationships, not every implementation field |
| Persistent entities, keys and relationships | `erDiagram` |
| Requirements and their verification relationships | `requirementDiagram` when a visual trace is clearer than the existing ID tables |
| Ordering across phases, time windows or operational workflows | Timeline, Gantt, journey or other supported types appropriate to the question |

Keep each diagram focused on one review question. Use short, descriptive labels and readable layout at normal browser width. Split a dense diagram into complementary inline views. Prefer a few useful diagrams over an exhaustive gallery. Do not rely on color alone, or hide required behavior in Mermaid comments that disappear when rendered.

Put relevant constraint/invariant IDs beside transitions, branches and boundaries. Show guards, effects, ordering, limits, failures and forbidden outcomes where they matter. Keep exact predicates, units, cardinalities and recovery assumptions in the numbered English tables when they would overwhelm the picture; diagrams and tables must agree. Explain whether arrows mean messages, dependencies or state transitions. Diagram syntax does not enforce the labeled conditions: section 5 provides their checkable tests.

Use syntax supported by the target Markdown renderer; GitHub supports fenced Mermaid but may lag upstream releases. Prefer supported equivalents over requiring plugins or custom JavaScript. Render and inspect the changed diagrams locally or in the authorized repository browser when available, checking labels, clipping and readability as well as syntax. Temporary validation files stay outside the repository. Do not introduce diagram sidecars; report any rendering check you could not perform.

## Rust implementation standard

For Rust tasks, use the guidance below when filling the plan and implementing it. It adapts [Apollo's rust-best-practices skill](https://github.com/apollographql/skills/blob/main/skills/rust-best-practices/SKILL.md), with the official [Rust Style Guide](https://doc.rust-lang.org/style-guide/), [API Guidelines](https://rust-lang.github.io/api-guidelines/), and [Cargo layout](https://doc.rust-lang.org/cargo/guide/project-layout.html) as references. These are conventions plus engineering judgment, not a universal architecture. For new Rust projects use the latest stable edition and a current stable toolchain, verified at planning time. Rust currently names its latest stable edition **2024**, not 2026; editions and compiler releases are different. Never default a new project to 2021 without a concrete compatibility reason. For existing repositories, identify compatibility constraints and include any edition/toolchain migration in the approved scope.

- **Architecture and files (section 2):** identify modules, their responsibilities, dependency direction, state owners and I/O boundaries. Use standard Cargo layout: `src/lib.rs` for reusable logic, `src/main.rs` for executable wiring, cohesive domain modules when needed, integration tests under `tests/`, private unit tests beside their code. Keep the smallest layout that makes those boundaries clear; do not create a file per type or layers named manager/service/factory without a concrete need. Existing layouts need no cosmetic reorganization.
- **Types and ownership (sections 2–3):** keep internals private and expose the smallest useful API. Prefer enums/newtypes when they prevent invalid states or mixed identifiers. Use concrete types first; introduce traits for actual substitution, including fault injection. Borrow when ownership is unnecessary; explain substantial cloning or shared mutation. For async systems, define task ownership, cancellation, shutdown, bounded queues and lock scope. Avoid blocking runtime workers and holding locks across network waits.
- **Errors and safety (sections 3–4):** specify which errors callers must distinguish and how they map to API outcomes. Use `Result` and preserve causes; typed errors fit library boundaries, contextual reporting fits applications. Do not require an error crate without need. External input, disk and network failures must not rely on `unwrap`/`expect`. Any deliberate invariant panic needs justification. Prefer safe Rust; any necessary unsafe block needs its safety obligations documented and checked.
- **Reviewability (sections 5–6):** document public behavior, errors and important invariants with rustdoc; comments explain non-obvious reasons. Tests cover behavior and failure modes, with enough assertions to establish the property. Prefer direct code over premature generic frameworks; optimize after measurement.
- **Tooling (section 6):** record the repository's edition, toolchain/MSRV policy, supported feature combinations and exact validation commands. Use stable rustfmt defaults unless the repository config says otherwise. Fix Clippy findings; narrowly justified lint exceptions only, using `#[expect]` when supported. Use formatting for the selected edition; do not enable every pedantic/nursery lint or add dependencies merely to satisfy an external style preference.

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
<Components, module/file responsibilities, dependency direction, ownership, data flow, state, persistence, and concurrency. Include the smallest useful Mermaid view(s) inline. Use event [guard] / effect for transitions.>

```mermaid
<choose the diagram type and write task-specific Mermaid>
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
Antithesis research: <pending until implementation / completed; scope and material limits>.
<Before greenlight: proposed tests and agreed Antithesis integration/execution scope. After implementation: research refines these cases against actual source without weakening the approved contract.>

| ID | Covers | Setup and action / fault schedule | Expected result | Test method |
| --- | --- | --- | --- | --- |
| T1 | C1, I1 | <Concrete scenario> | <Observable pass/fail outcome> | <Unit / simulation / process / integration / hosted> |

<Test seams and independent oracle needed to implement these cases. Define liveness recovery conditions and timeout semantics. Proposed tests are not passing results.>

## 6. Implementation handoff
Status: <awaiting user greenlight / approved / implementing / testing / ready for review / blocked>.
Approval: <pending / user-approved scope and any subsequent agreed changes>.

<Short implementation sequence, required test hooks, commands/checks, and completion criteria. Include references needed to implement the contract. Brief verification status may be added after work; no separate report.>
````

Use an outer fence longer than the embedded Mermaid fence when presenting the entire form as a code block. The form above describes the structure; do not leave angle-bracket placeholders in an actual plan. If a category is irrelevant, write one short reason under its heading instead of manufacturing requirements.

Mermaid and English define the intended behavior; they do not prove the implementation correct. Tests should check observable outcomes, including forbidden traces, rather than duplicate implementation logic. Distinguish local tests, simulated faults and hosted Antithesis runs in the brief verification status.

Example: [Raft plan](examples/raft/PLAN.md).
