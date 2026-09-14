---
name: speaking-in-tokens
description: Turn engineering intent into reviewable UML designs, explicit constraints, and invariants, then coordinate an independent Antithesis research and test-planning pass before implementation and a fresh test agent afterward. Use for substantial systems designed through this workflow.
---

# Speaking in Tokens

Keep English as the fast entry point. Expose its interpretation as UML with explicit guards, effects, constraints, and invariants. The human review surface is the design, not proof annotations. UML without defined semantics is documentation, not a correctness guarantee.

## Artifacts

Create a task directory containing:

- `PROMPT.md`: original request, scope, supplied references, and explicitly labeled inferred choices.
- `SKILL.md`: task-specific instructions linking back to this skill and the design contract; avoid copying the entire workflow.
- `design/*.puml`: UML source (PlantUML); sequence diagrams for interactions, state diagrams for lifecycle, component diagrams for ownership. Use only the views that clarify the task.
- `DESIGN.md`: APIs, UML links, constraints and stable property IDs, accepted/forbidden traces, failure semantics, assumptions, exclusions, and unresolved decisions.
- `TEST_PLAN.md`: independent research agent's scenarios, expected outcomes, fault schedules, test oracles, and coverage-to-property mapping, written before implementation.
- `HANDOFF.md`: phase boundaries, agent roles, checks actually run, failures, repairs, and remaining coverage limitations.
- Source and tests in the target language's conventional layout.

Render UML locally where available and inspect the result. Never send private design text to a public rendering service without authorization. Diagrams and prose must agree; do not claim executable semantics merely because a diagram renders.

## Pipeline

### 1. Interpret and design

Inspect the relevant source if it exists. Identify ambiguous outcomes using examples, not a long generic interview. Choose routine details from context; record them as assumptions rather than user decisions. Ask only when a missing decision materially changes intended behavior. Existing authorization to implement remains valid: producing a concrete design does not automatically require another approval.

Write UML with `event [guard] / effect` transitions and success/failure ordering. Define persistence boundaries, ownership, externally visible results, concurrency, and retry semantics. Give invariants stable IDs and attach them to diagrams and API behavior. Specify liveness assumptions separately from safety requirements. Use readable English constraints; formal languages are optional, not the default.

### 2. Independent Antithesis research BEFORE implementation

Use a separate agent with the prompt, UML/design, existing source if any, and the available `antithesis-research` skill. It must read that skill and applicable references. This is a required pass, not a synonym for writing ordinary unit tests. Preserve the scratchbook artifacts required by that skill inside the task directory. Link the property catalog into `TEST_PLAN.md`.

For a greenfield task, say explicitly that this is design-stage research: no implementation exists yet, proposed assertions are not installed, source-level defect claims are unavailable. Research the design, protocol, fault model, and intended interfaces. Revisit concrete code paths after implementation. Do not fabricate a code scan or claim an Antithesis execution occurred.

The researcher owns the test plan. It must include observable oracles independent of implementation internals, failure/recovery schedules, reachable interesting states, and test environment boundaries. Investigate ambiguities; surface unresolved semantic choices before dependent implementation. Do not invent an answer to a scope question from another skill; carry forward actual user scope and record inferred references honestly.

### 3. Implementation agent, then STOP

Give a separate implementation agent the agreed design and the frozen test plan. It writes production code, runs compilation and existing checks, and writes its handoff. It must not rewrite expected outcomes to fit its implementation or author the independent acceptance tests. It stops at its handoff; the orchestrator then starts a fresh test agent.

### 4. Fresh test agent

Give the fresh agent the design, original test plan, implementation, and handoff. It implements and runs the planned tests. It may inspect source to connect fixtures and instrumentation, but expected behavior comes from the design. It records missing requirements, uncovered properties, and failures instead of silently weakening checks. Refresh the Antithesis source/instrumentation evidence as appropriate.

### 5. Repair and review

Return failures to implementation with minimal reproductions and property IDs. Keep the independently authored assertions unchanged unless the test itself is demonstrated wrong against the design; document such corrections. Re-run affected checks. Record that repairs occurred rather than labeling the result a successful one-shot.

Report the actual boundary of evidence: compilation, local tests, fault simulation, container smoke tests, and hosted Antithesis runs are distinct. A proposed deployment topology does not mean Antithesis was configured or run. Do not publish externally beyond the user's authorization.

## Example

[Raft service](examples/raft/PROMPT.md): elections, durable replication, linearizable client operations, and recovery. Read its [design](examples/raft/DESIGN.md) and [test plan](examples/raft/TEST_PLAN.md) together. Additional event-bus and file-storage examples are future work, not implemented examples.
