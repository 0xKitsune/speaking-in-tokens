# Workflow record

## Design phase

Primary agent wrote PROMPT.md, task SKILL.md, DESIGN.md and three UML sources before production code. Scope is a static-membership Raft KV service with HTTP JSON, durable state and log-based reads. This phase completed before implementation. No compilation or tests had run at that point.

No additional reference answer arrived after the optional response window. Research proceeded with the provided design and assistant-proposed original Raft paper, explicitly labeled as an assumption.

## Independent research phase

A separate research agent used antithesis-research and wrote TEST_PLAN.md plus a 19-property scratchbook before production code. A fresh reviewer checked the artifacts. Findings added exact client proposal identity and a persistence-failure seam; review corrected the restart oracle to inspect uncommitted replay before new elections can legitimately commit retained entries. The researcher authored neither production code nor tests.

## Design validation

The shared and example SKILL.md files passed skill-creator quick_validate.py (using an isolated uv environment with PyYAML). All three UML sources rendered locally with PlantUML; their PNG previews were inspected and the role layout revised for readability. Committed SVGs are generated views; scripts/render-uml.sh rerenders them with a local Java runtime and PlantUML jar. No private text was sent to a rendering server.
