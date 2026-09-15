# Speaking in Tokens

## Overview

**Problem.** Natural-language requirements are fast to write but often leave behavior, boundaries, and failure semantics open to interpretation. [Intent Formalization](https://arxiv.org/abs/2603.17150) calls this the intent gap: an agent can faithfully implement its own coherent interpretation and still build the wrong system. Tests or proofs against that same interpretation can establish internal consistency without establishing that the specification captured what the engineer meant.

**Solution.** Speaking in Tokens externalizes the agent's interpretation before code is written. It produces one `PLAN.md` containing a readable system diagram, precise constraints and invariants, and observable acceptance conditions. Proposed and unresolved choices remain visible so the engineer can correct the artifact and approve a shared contract; implementation, independent testing, and independent review then use that contract as their source of truth.

**Human interface.** The artifact is designed to make the model's understanding and suggested approach as easy as possible to read, intuit, and reason about. Engineers should be able to see the proposed boundary, ownership, state, control and data flow, failure behavior, and open choices in one pass—and correct a box, arrow, or rule without reconstructing the design from a long conversation or generated code.

The interaction starts with a prompt, not an interview: the agent inspects available context, drafts the complete artifact, presents it, and applies corrections until the engineer approves it. The goal is not more notation; it is faster correction of hidden assumptions and a stable basis for verification.

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

## Research

These papers motivate the skill; they are context, not additional workflow requirements.

- [*Intent Formalization: A Grand Challenge for Reliable Coding in the Age of AI Agents*](https://arxiv.org/abs/2603.17150) frames the gap between human intent and the specification an agent actually implements. See also the [Microsoft Research publication record](https://www.microsoft.com/en-us/research/publication/intent-formalization-a-grand-challenge-for-reliable-coding-in-the-age-of-ai-agents/).
- [*Structure-guided function-level code generation with LLMs via UML activity diagrams*](https://doi.org/10.1016/j.neucom.2025.132502) introduces StructGen and evaluates UML activity diagrams as guidance between requirements and code generation.

Intent Formalization is a position paper: it establishes the intent gap as a serious research problem but does not test this interface. StructGen reports improved function-level code generation with UML guidance, but it does not isolate diagrams from the additional planning and repair work, test human review, or establish repository-scale results.
