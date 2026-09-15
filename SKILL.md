---
name: speaking-in-tokens
description: Analyze an engineering change and produce one code-grounded, constraint-first spt_<task>.md artifact for human review. Use when a proposed system needs readable diagrams, precise bounds and invariants, an implementation map, and concrete test coverage before code is written.
---

# Speaking in Tokens

Produce exactly one planning artifact. Name it `spt_<descriptive_task_slug>.md`: use the `spt_` prefix and a short lowercase snake-case description of the engineering task, such as `spt_automated_failover.md`. Never use a generic name such as `PLAN.md`.

The artifact is the proposed system, written so an experienced engineer can understand its behavior, boundaries, failure semantics, and implementation consequences without reading the source first.

This is a design-planning skill, not an experiment log or an approval workflow. Do not create alternate versions, companion reports, prompt transcripts, review-focus sections, status dashboards, approval gates, or implementation handoffs. Do not write production code or tests unless the user separately asks for implementation.

## Build the plan from the code

1. Read the request, complete relevant call paths, surrounding code, tests, configuration, deployment assumptions, and linked repositories in scope. Follow behavior across abstraction boundaries instead of inferring it from names.
2. Establish the actual state owners, authority source, concurrency model, shutdown behavior, durable stores, trust boundaries, retry and timeout behavior, and existing recovery paths. Record only facts that constrain the proposed design.
3. Resolve questions from the repository when possible. Make a defensible design choice when the evidence supports one; ask the user only when a missing decision would materially change the system and cannot be inferred safely.
4. Use `antithesis-research` automatically, without asking permission, to identify failure-prone boundaries, safety and liveness properties, observability needs, and useful fault schedules. This is research for the plan: recommend what tests should be written, but do not write the tests. Fold the results into `Test coverage`; do not create an Antithesis report or campaign section.
5. Write or replace the task's existing `spt_*.md` artifact. If an earlier artifact has a generic or obsolete name, rename and revise it in place rather than leaving two versions.
6. Render and inspect every Mermaid diagram. Correct syntax, direction, clipping, excessive width or height, and labels that require surrounding prose to decode. Temporary render files stay outside the repository.

## Artifact order

Use this reading order, adapting subsection names to the system:

1. Title.
2. A primary Mermaid system diagram immediately after the title, before prose.
3. Motivation.
4. Proposed behavior, developed from lifecycle and critical control flow into detailed mechanics.
5. Implementation plan with ordered steps and stable constraint IDs.
6. Safety and liveness invariants.
7. Implementation map with concrete files, symbols, ownership, and dependency changes.
8. Complete system view: the most granular diagram, after the reader has the vocabulary needed to interpret it.
9. Test coverage, after the complete system view.

Do not add a standalone “how the system works today” or repository-tour section. Existing behavior belongs only where it explains a design seam, a compatibility requirement, or an invariant that the change must preserve. Do not repeat the prompt, meeting notes, process history, owners, schedules, experiments, or next steps.

Keep the document concise without deleting code-level detail that changes correctness. Prefer dense, exact descriptions over summaries that force the reader to rediscover constraints in the source.

## Reader-anticipation sequencing

<strong><span style="text-decoration-line: underline; text-decoration-style: double;">Good technical writing models the reader's own next thought and resolves it before they have to ask. Use that thought to order the material, then hide the scaffolding behind ordinary engineering headings that a human would naturally expect in a design document.</span></strong>

The heading list is the technical spine of the artifact. A reader who scans only the headings should understand the system's reasoning path, the engineering content of every section, and why each section follows the previous one.

- Make every title maximally informative per word. Titles are labels, not prose: remove ornamental adjectives, scene-setting, and performative phrasing. Prefer “Handoff Sequence,” “Leader Change Submission,” “Block Production Handover,” “Success Criteria,” and “E2E Failover Tests” over “Graceful Handoff Design,” literal questions, vague labels such as “Lifecycle,” or rubric-shaped phrases such as “Real-Process Handoff Coverage.”
- Use `E2E`, not “end-to-end.” Match the compact vocabulary engineers use with each other.
- Do not force every constraint into the heading. Use the heading to orient the reader, then state immutability, evidence, deadlines, and other precise rules in the first paragraph or a focused subheading.
- Reuse the nouns and concepts established immediately above so the reader never has to translate between two vocabularies.
- Introduce a mechanism only after explaining the problem that makes it necessary. Introduce edge cases immediately after the normal path raises them.
- Put bounds, failure behavior, and recovery directly after the operation whose failure the reader will now be wondering about.
- Do not combine unrelated concepts in an `X and Y` heading merely because they share code. Split them or name the single engineering concept that connects them.
- Reorder the material when the natural question sequence differs from the repository layout or implementation order.

Before presenting the plan, read only its title and headings in order. Rewrite any heading that sounds like a grading rubric, generated taxonomy, or compressed summary rather than something an experienced engineer would naturally say. Artifact headings should not be written as questions.

## Diagram progression

All diagrams are engineering drawings written in plain English. A box should say what a component or state does, owns, or guarantees. Put code symbols and file paths in the implementation map, not in the opening diagram.

### Primary diagram

The first diagram is a real system view, not a decorative overview and not a prose-heavy decision tree. It should be slightly quieter than the complete system view while showing enough detail to stand on its own:

- external trigger or actor;
- system and process boundaries;
- the components on the critical success path;
- authoritative and durable state owners;
- permitted control and data flow;
- the externally meaningful recovery path;
- exact limits, ordering rules, and guards when they define safe behavior.

Use one reading direction and short English labels. Keep related components inside named subgraphs. Label arrows with the event, data, or permission they carry. State numeric bounds and ownership directly instead of hiding them behind unexplained IDs.

### Supporting diagrams

Add lifecycle, sequence, or focused flow diagrams only when they reveal ordering, concurrency, or failure behavior that the primary diagram cannot show cleanly. Each diagram must add detail. Do not restate the same flow at a different level or substitute a flowchart for precise contract text.

### Complete system view

End the design material with the most granular architecture diagram. It may include concrete internal components, durable stores, concurrency and shutdown ownership, control and data planes, observability, protocol boundaries, and constraint or invariant IDs. It must remain readable at normal browser width; split genuinely independent flows into earlier supporting diagrams rather than making this final view a wall of text.

## Required content

### Motivation

Use one short paragraph. State the operational or product failure, why the existing system cannot provide the desired outcome, what success changes, and the hard boundary. Do not preview mechanics that the following sections explain. Describe triggers by behavior—such as recoverable process exit or supervised panic—rather than treating one deployment mechanism such as Kubernetes SIGTERM as the entire problem.

### Proposed behavior

Describe the new control flow in execution order. Include, where relevant:

- entry conditions and every trigger class;
- state ownership and exactly-once behavior;
- authority and authorization checks;
- deterministic selection and retry boundaries;
- the exact point after which a choice becomes immutable;
- ordering between durable state changes, receipts, finality, and local effects;
- what continues running during coordination;
- success evidence and how it is observed;
- deadlines, clock semantics, cancellation, shutdown, and supervisor interaction;
- failure outcomes and the recovery path for failures that cannot run in-process;
- compatibility, rollout, security, and unchanged protocol behavior.

Never turn unresolved engineering questions from the prompt into a copied question list. Answer them from source evidence and invariants, or identify the single decision that genuinely requires user input.

### Implementation plan

Use the heading `Implementation Plan`. Present the implementation as ordered, action-oriented steps and give each consequential step a stable constraint ID (`C1`, `C2`, …). Each statement must be independently checkable.

- An implementation step names the concrete change and defines its input, boundary, owner, order, limit, failure response, or compatibility rule.

### Invariants

Give invariants stable IDs (`I1`, `I2`, …). Each invariant must be independently checkable.

- A safety invariant says what must never happen and under which failure model.
- A liveness invariant states the assumptions and numeric bound under which progress is required.

Do not use status columns such as confirmed, proposed, or unresolved. The document is the plan being presented for human review. Do not encode a binding rule only in Mermaid.

### Implementation map

Map behavior to the actual repository. Name the files, existing symbols, new responsibilities, and dependency direction. Explain why each seam changes and what remains untouched. Include required configuration and deployment changes. Avoid exhaustive file lists, speculative abstractions, and declarations with no control-flow consequence.

## Test coverage

Place all test material after the complete system view under one `Test coverage` section. Describe tests to write, not tests claimed to pass.

Start with concrete test groups; do not add a preamble that explains how to read the tests. Do not create a standalone test-hooks or observability section. Put a required test-only hook or failpoint inside the specific test that needs it, and keep it unavailable through production-facing interfaces.

For each applicable test, state:

- the constraints and invariants covered;
- setup and exact action or fault schedule;
- the external or independent oracle;
- the literal pass condition;
- timeout and healing assumptions;
- how a failure is reproduced with a seed or minimized trace when relevant.

Use standard testing categories: unit/model tests, E2E tests, chaos tests, and regression tests. Chaos coverage should include ordering, duplication, partitions, pauses, process exits, restarts, persistence or provider ambiguity, and bounded-resource behavior when applicable. Separate safety properties, which hold continuously, from recovery checks, which require stated healing and scheduling assumptions.

Translate fault research into a short `Chaos Tests` section written in ordinary system language. Explain in one sentence why these cases are not covered by unit tests or a normal E2E run, then use a `Scenario | Failure introduced | Expected result` table. Do not invent a testing category, expose assertion-framework taxonomy, use SDK terms or code-style property names, describe a branded campaign, or add a tool-specific topology, hosted-run plan, or product-documentation discussion. The runner is an implementation choice, not part of the design. Do not copy implementation decision logic into the test oracle.

End the artifact with one small `Metrics` table after all test sections. Include only the metric name, type, bounded labels, and operational purpose. Do not precede it with observability prose, and never use secrets, endpoints, hashes, identities, transaction bodies, or raw errors as metric labels.

## Final checks

Before presenting the artifact, verify:

- the opening diagram is first, uses English boxes, exposes real boundaries and limits, and is less detailed than the complete view without becoming vague;
- the diagrams become progressively more detailed and do not repeat one another;
- the declarative headings alone form a clear technical progression, with each section resolving the uncertainty created by the preceding material;
- the motivation explains why the change matters;
- every failure path ends in an explicit outcome;
- constraints, invariants, implementation seams, and tests agree;
- every consequential contract rule has observable test coverage;
- the complete system view appears before test coverage;
- tests begin with concrete cases and end with one small metrics table;
- no current-system survey, review focus, approval state, experiment material, duplicate artifact, or Antithesis campaign section remains;
- claims about code and existing tests are supported by inspection, and planned checks are not presented as completed.

Present the finished artifact with a direct link. If the user asks to view it in a browser, open that same `spt_*.md` file; do not generate an HTML or second Markdown copy.
