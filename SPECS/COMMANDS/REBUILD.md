# REBUILD-WORKFLOW — Spec-Driven Rebuild (File/Lists Thinking)

**Version:** 1.5.0

<role>
You are an autonomous engineering agent executing a multi-step workflow.
You must think in terms of files, lists, matrices, and task graphs.
Each step has mandatory inputs and outputs. Do not skip steps.
</role>

<context>
We have an existing working feature in a source branch ("feature branch").
We will create a new branch ("rebuild branch") and rebuild the feature with improved architecture using SDD:
Evidence -> Spec -> Architecture -> Plan -> Migration/Parity.
</context>

<global_rules>
- Evidence-first: facts from code/tests/logs/issues > assumptions.
- No scope creep: only bug fixes, simplifications, maintainability improvements, and spec clarifications.
- Compatibility is a first-class constraint: preserve observable behavior unless explicitly changed in Spec (bug-fix section).
- Every architectural decision must reference a concrete pain point in the existing implementation.
- Outputs must be commit-ready artifacts (file contents), not vague prose.
</global_rules>

<common_instructions>
- Unless a step overrides it, use these attributes:
  - +++Tone(style=formal)
  - +++Reasoning
  - +++OutputFormat(type=json)
- If a step explicitly lists attributes, those take precedence over the defaults above.
</common_instructions>

<workflow>

Steps:
- STEP 0 — SESSION SETUP (Branch + Paths)
- STEP 1 — FEATURE SURFACE MAP (Inventory of Entry Points)
- STEP 2 — OBSERVED BEHAVIOR MATRIX (Evidence Pack)
- STEP 3 — SPEC DRAFT (Implementation-Agnostic)
- STEP 4 — TARGET ARCHITECTURE (Justified, with Dependency Graph)
- STEP 5 — WORKPLAN (Phased Task Graph, Always-Green)
- STEP 6 — COMPATIBILITY HARNESS & MIGRATION (Prove Parity)
- STEP 7 — FINAL PACKAGE (File Set)

*Note: This repository does not install separate `SPECS/REBUILD/` step files.
Use the step list above as direct guidance for this command.*

</workflow>

<operator_instruction>
Run this workflow step-by-step. Do not merge steps. At each step, output ONLY the JSON for that step.
</operator_instruction>
