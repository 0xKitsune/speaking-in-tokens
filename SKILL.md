---
name: speaking-in-tokens
description: Turn an engineering prompt into a constraint-first PLAN.md with a readable Mermaid system snapshot, then guide approval, implementation, independent Antithesis research, fresh-agent tests, and review. Use when engineers need to verify and correct a proposed system quickly before implementation.
---

# Speaking in Tokens

Produce exactly **one planning artifact: `PLAN.md`**. It is the complete contract for the work. Use the form below, with the same numbered headings in the same order every time. Scale the contents to the task; keep the form stable.

Optimize the plan for correction speed. An engineer should be able to scan the primary diagram and constraint tables and answer: Is this the right system boundary? Who owns each decision and state transition? Which interactions are allowed? Which limits and failure behaviors are binding? Which choices are still proposed or unresolved?

Constraints and invariants are the contract. Mermaid is their visual index, not a substitute for precise English. Give every consequential rule a stable constraint ID (`C1`, `C2`) or invariant ID (`I1`, `I2`), attach the relevant IDs to the diagram, and cover them from section 5. Do not present an inference as an agreed requirement.

Do not generate prompt transcripts, task-specific skills, separate design/test plans, scratchbooks, property files, reports, handoff files, diagram sidecars, or rendering scripts. Keep diagrams inline in fenced `mermaid` blocks so the plan renders in Markdown in the target browser. Human readability is the primary design criterion. Research findings belong in the relevant sections of the plan, not in an appendix of process history. Source and test files are created only when implementation is authorized; they are not planning artifacts.

## Workflow: draft, agree, build, test, review

1. **Gather context without interviewing.** Read the prompt, relevant source, existing tests, interfaces and operational constraints. Investigate questions that the repository can answer. Identify actors, system and trust boundaries, data, relationships, state owners, allowed dependencies, failure modes and intended outcomes. Do not ask planning questions before producing the first artifact.
2. **Draft the artifact.** Create the complete six-section `PLAN.md` immediately. Make reasonable choices when the prompt and repository support them. Mark sourced requirements as `confirmed`, agent-selected defaults as `proposed`, and decisions that cannot safely be inferred as `unresolved`. Put alternatives and their consequences in the review focus instead of starting a question-by-question exchange. Lead section 2 with one readable Mermaid system snapshot. Section 5 includes concrete proposed test cases and coverage; mark post-implementation Antithesis research as pending.
3. **Present and revise the artifact.** Present/link `PLAN.md` and summarize only the proposed or unresolved choices that need attention. Treat the document as the conversation surface: apply the user's corrections directly, keep its diagram, constraints, invariants and tests consistent, and present the revised artifact. Do not switch to an interview unless the user explicitly asks for one.
4. **Wait for the user's greenlight.** Stop before implementation with section 6 set to `awaiting user greenlight`. There must be no `unresolved` constraint or invariant at this gate. A general request to build something is not approval of an unseen plan in this workflow. An explicit greenlight for the current plan confirms its `proposed` rows; update their status and do not ask again. Material changes to the agreed contract return to artifact revision and approval before dependent work.
5. **Implement the approved plan.** An implementation agent writes production code and the agreed test seams, runs formatting, compilation, lints and existing tests, records the brief handoff in section 6, then stops. It must not author or weaken the independent acceptance suite to fit its code.
6. **Run independent Antithesis research against the implementation.** A separate agent reads and runs `antithesis-research`, using the approved plan and actual source. It maps invariants and failure-prone paths, proposes concrete unit, property-based, simulation, process and Antithesis workloads/assertions as appropriate, and updates section 5 with fault schedules, observables, oracles and test hooks. Flag contract gaps in section 6. The single-file requirement overrides the research skill's scratchbook layout: return findings to the orchestrator or edit PLAN.md only. If the skill is missing, report the missing pass; do not relabel ordinary testing as Antithesis research.
7. **Have a fresh agent write and run the tests.** Give another agent the approved plan, completed research and source. It implements the test cases and required Antithesis integration within the agreed scope. Expected behavior comes from the contract; inspect source only to connect fixtures and instrumentation. Return defects for repair; change a test expectation only when demonstrably wrong against the contract. Re-run affected checks after repairs.
8. **Pass an independent review gate.** A fresh reviewer that did not implement the change or author its acceptance tests inspects the complete diff, surrounding code, tests and relevant configuration. It checks every approved constraint and invariant ID, preserved behavior, correctness, compatibility and consensus risk, security and authorization boundaries, rollout and operational failure modes, unnecessary complexity, and missing behavioral coverage. Passing tests are evidence, not a substitute for this review. Blocking findings return to implementation, affected checks are rerun, and the reviewer verifies the repair. Section 6 records findings, repairs, checks actually run, remaining gaps and the final readiness decision. Separate local property tests, fault simulation and hosted Antithesis results. Missing required coverage, an unreviewed repair or an unresolved blocking finding means blocked/incomplete, not ready. All reporting stays in PLAN.md; no separate report or PR is required unless requested.

## Test contract for Rust

Section 5 must specify executable cases, not just a list of tools. For each applicable layer, identify covered constraint and invariant IDs, setup, actions or generated inputs, observable assertions and replay/reproduction method. Explain any inapplicable layer briefly.

- **Unit and regression tests:** use Rust `#[test]`, private `#[cfg(test)] mod tests` where appropriate, and integration tests under `tests/`. Use the runtime's test attribute for async tests. Assert success, boundary inputs and failure behavior; retain a regression for repaired defects.
- **Property-based tests:** use `proptest!` or an equivalent justified framework for generated inputs and operation sequences. Define generators, preconditions, independent oracle, shrinking strategy and replayable failures. Specify the actual universal property. A handful of randomized samples is not a substitute for these requirements.
- **Stateful and fault tests:** exercise ordering, duplication, partitions, cancellation, restart, persistence failures and resource bounds when relevant. State the fault model, forbidden traces and liveness assumptions. Preserve seeds/traces and minimize failing histories.
- **Antithesis:** research concrete safety, liveness and reachability assertions; propose SDK instrumentation, workload, topology, setup and a runnable local smoke check where applicable. Distinguish proposed hooks from installed hooks, and local execution from a hosted campaign. Include the required integration and execution scope in the plan before greenlight; use current `antithesis-documentation` guidance for integration. A hosted run requires configured access and authorization and cannot be inferred from a research pass. If agreed hosted validation is unavailable, report that specific review blocker.

Tests must be independent enough to detect a wrong implementation; do not copy its decision logic into the oracle. Planned cases are not passing results. The fresh test agent owns test implementation, including property tests and agreed Antithesis hooks/workloads.

## Constraint-first Mermaid

Section 2 starts with exactly one **primary system snapshot**. Use a Mermaid diagram with UML-style boundaries and relationships. Prefer a left-to-right component view unless the system is fundamentally a lifecycle or data model. The snapshot must show, where relevant:

- human actors and external systems outside the system boundary;
- components inside the boundary, grouped by ownership, trust or deployment boundary;
- the authoritative owner of mutable state and each durable store;
- permitted calls, events or data movement, with arrow meaning stated once;
- the critical success path and any failure or recovery path that changes externally visible behavior;
- constraint and invariant IDs at the box, boundary or arrow they constrain.

The snapshot is a map, not a prose container. Use one reading direction, short role-based labels and no implementation fields or method inventories. Do not encode a binding rule only in a node label or Mermaid comment. Keep predicates, units, cardinalities, time bounds, authorization rules, retry budgets, ordering guarantees and recovery assumptions in sections 3 and 4; the diagram points to those rows by ID.

Add at most one supporting diagram when a critical behavior cannot be understood from the snapshot and tables alone. Use a sequence diagram for ordering and concurrency, a state diagram for lifecycle and recovery, or an ER/class diagram for cardinality and ownership. A supporting diagram answers one named review question. Label state transitions `event [guard] / effect`. Do not add a diagram merely to restate the primary view.

Apply these consistency rules:

- Every ID shown in Mermaid exists exactly once in the constraint or invariant tables.
- Every architecturally consequential boundary or arrow has an applicable ID.
- Every `proposed` or `unresolved` row is visible in the review focus and easy to locate in the diagram when it affects structure or flow.
- Diagram and table conflicts are contract defects; resolve them before greenlight.
- Arrows have one declared meaning per diagram. Bidirectional arrows require independently defined behavior in both directions.
- Safety invariants say what must never happen. Liveness invariants state the progress assumptions and time or retry boundary under which progress is required.

Before presenting the first artifact, inspect it as a skeptical engineer: confirm that the system boundary, state ownership, allowed and forbidden interactions, critical limits, failure behavior and unresolved choices are visible without reconstructing them from prose. Confirm that every test has an observable oracle and that every consequential constraint has planned coverage. This check improves the draft; it does not replace user correction or establish that the proposed design is right.

Render and inspect the changed diagrams locally or in the authorized repository browser when available. Check syntax, reading order, label clipping and legibility at normal browser width. If the snapshot needs extensive narration to decode, simplify it. Temporary validation files stay outside the repository. Do not introduce diagram sidecars; report any rendering check you could not perform.

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

Review focus: <List proposed or unresolved choices that need engineer attention, or `none`.>

## 2. Design
<State what arrows mean. Then give the primary system snapshot: boundary, actors, components, ownership, permitted dependencies, authoritative state, data flow, persistence, concurrency and contract-changing failure/recovery paths. Put applicable constraint and invariant IDs on boxes, boundaries or arrows.>

```mermaid
<primary system snapshot>
```

<Add no more than one focused supporting diagram only when required.>

## 3. Interfaces and constraints
Status is `confirmed`, `proposed` or `unresolved`. State the basis for every confirmed row. Each row contains one checkable rule.

| ID | Boundary | Required behavior | Status and basis |
| --- | --- | --- | --- |
| C1 | <API / trust / ownership / dependency boundary> | <Inputs, outputs, limits, authorization, errors, retries, ordering and compatibility as relevant> | <status; source of confirmation when confirmed> |

## 4. Invariants
| ID | Property | Scope and assumptions | Status and basis |
| --- | --- | --- | --- |
| I1 | <Precisely what must never happen, or must eventually happen> | <Scope, failure model, progress assumptions and bound> | <status; source of confirmation when confirmed> |

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
