# Speaking in Tokens

Speaking in Tokens is a Codex skill for turning substantial engineering work into a reviewable design contract before implementation.

The skill produces one planning artifact, `PLAN.md`, using the contract defined in [SKILL.md](SKILL.md). Its primary review surface is a concise Mermaid system snapshot connected to explicit constraints and invariants. The goal is to let engineers quickly see what system is being proposed, which boundaries and behaviors are binding, and what needs correction.

The interaction starts with a prompt, not an interview. The agent inspects available context, drafts the complete artifact, and presents it. The engineer then corrects the artifact until it is ready for approval.

## What the skill enforces

- One `PLAN.md`, with a stable six-section structure.
- One primary Mermaid system snapshot showing actors, boundaries, ownership, authoritative state, permitted interactions, and contract-changing failure paths.
- Stable constraint IDs (`C1`, `C2`) and invariant IDs (`I1`, `I2`) shared by the diagram, English contract, and tests.
- Explicit `confirmed`, `proposed`, and `unresolved` statuses so inferred decisions do not look agreed.
- No implementation until the user approves a plan with no unresolved constraints.
- Implementation, independent Antithesis research, independently authored tests, repair, and a fresh independent review against the approved contract.

## Workflow

1. Inspect the prompt, source, tests, interfaces, and operational environment without conducting a requirements interview.
2. Draft the complete constraint-first `PLAN.md`, marking inferred choices and unresolved decisions.
3. Present the artifact and apply corrections directly to it.
4. Wait for explicit user greenlight.
5. Implement the approved plan.
6. Run independent Antithesis research against the implementation.
7. Have a fresh agent write and run the acceptance tests.
8. Repair failures, then have a fresh reviewer inspect the complete diff, tests, configuration, risks, and coverage against the contract.

The complete instructions and required plan form are in [SKILL.md](SKILL.md).

## Rationale

**Problem.** Natural-language requirements are fast to write but often leave behavior, boundaries, and failure semantics open to interpretation. [Intent Formalization](https://arxiv.org/abs/2603.17150) calls this the intent gap: an agent can faithfully implement its own coherent interpretation and still build the wrong system. Tests or proofs against that same interpretation can establish internal consistency without establishing that the specification captured what the engineer meant.

**Solution.** Before writing code, the agent externalizes its interpretation as one compact artifact: a readable system diagram, precise constraints and invariants, and observable acceptance conditions. Proposed and unresolved choices are visible so the engineer can correct the artifact and approve a shared contract. Implementation, independent testing, and independent review then use that contract as their source of truth rather than inferring intent again from the original prompt or from the produced code.

**Human interface.** The artifact is designed to make the model's understanding and suggested approach as easy as possible to read, intuit, and reason about. Engineers should be able to see the proposed boundary, ownership, state, control and data flow, failure behavior, and open choices in one pass—and correct a box, arrow, or rule without reconstructing the design from a long conversation or generated code.

The goal is not more notation; it is faster correction of hidden assumptions and a stable basis for verification. Mermaid helps only when its boxes and arrows connect to exact, reviewable rules.

## Research

These papers motivate the skill; they are context, not additional workflow requirements.

- [*Intent Formalization: A Grand Challenge for Reliable Coding in the Age of AI Agents*](https://arxiv.org/abs/2603.17150) frames the gap between human intent and the specification an agent actually implements. See also the [Microsoft Research publication record](https://www.microsoft.com/en-us/research/publication/intent-formalization-a-grand-challenge-for-reliable-coding-in-the-age-of-ai-agents/).
- [*Structure-guided function-level code generation with LLMs via UML activity diagrams*](https://doi.org/10.1016/j.neucom.2025.132502) introduces StructGen and evaluates UML activity diagrams as guidance between requirements and code generation.

Intent Formalization is a position paper: it establishes the intent gap as a serious research problem but does not test this interface. StructGen reports improved function-level code generation with UML guidance, but it does not isolate diagrams from the additional planning and repair work, test human review, or establish repository-scale results.

## Evidence boundary

No selected paper establishes that engineers can reliably catch incorrect intent by reviewing the artifact this skill produces. No selected paper establishes that Mermaid is superior to equally precise prose, that the constraint tables and diagram remain consistent, or that this workflow improves autonomous repository-scale implementation.

Those are hypotheses this skill makes testable. The artifact must make proposed choices visible, the engineer must correct it before approval, and implementation and tests must be checked against the approved constraints. The quality of that review must be evaluated through actual use; a coherent-looking diagram is not evidence that the design is correct.
