---
id: SR-003
title: "base checklist review of TL-177 quire-mltl spec artifacts"
type: SpecReview
analysis: base
scope: "spec/spec.md, spec/requirements/FR-001, FR-002, FR-003, NFR-001, spec/decisions/ADR-001"
review_set: subset
---

## Summary

Ran the base requirements checklist (ID formats, duplicates, cross-references,
test-coverage rules) against the six artifacts newly authored for TL-177.
`quire validate --scope /home/peter/dev/quire-mltl/. "spec/**/*.md"` passes
with no errors and no warnings scoped to these files (two EARS warnings and
one vague-verb warning were found and fixed during authoring). The one
structural gap — no `TC-*` test cases exist yet — is expected and intentional:
this is spec-first authoring (TL-177) ahead of the implementation port
(TL-178), which has not landed any code or tests yet.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | ID formats are correct and sequential: MRS-001, FR-001..FR-003, NFR-001, ADR-001; AC/CON ids follow `{PARENT}-AC-N`/`{PARENT}-CON-N`; no duplicates. | MRS-001, FR-001, FR-002, FR-003, NFR-001, ADR-001 |
| FND-002 | low | Every FR/NFR/ADR relationship target is a real artifact id, checked against the pinned upstream revisions: `tl-mltl` FR-018 (repo HEAD), `quire-observation` FR-004/FR-009/FR-010/FR-011 (commit `2bdeb833a330bfa777c19eb4c28c423f856f3ba6`, this crate's `Cargo.toml` pin — the working checkout's default branch is a stale feature branch and does not itself carry these ids), `tl-syntax` FR-014, `quire-contract-ir` PGM-01. | FR-001, FR-002, FR-003, NFR-001, ADR-001 |
| FND-003 | medium | No `TC-*` test cases exist yet, so every AC's Verification column reads plain `Test`/`Inspection` with no `TC-NNN` reference. This is the expected state for spec-first authoring ahead of TL-178 (the implementation port has not landed); it is recorded here rather than backfilled with invented test-case ids, per the instruction not to invent traceability that doesn't exist. TL-178 must add `tc_NNN`-tagged tests and, once `spec/test-cases/` exists, wire each AC to a real `TC-*` id. | FR-001, FR-002, FR-003, NFR-001 |
| FND-004 | low | No FR cites a `US-*` user story; this matches the established `tl-mltl`/`tl-syntax` convention in this same crate family, where FRs `implements` the MRS directly rather than an intermediate user story (unlike `quire-observation`, which does use `US-001`). Not a defect — an intentional cross-repo consistency choice, not an omission. | FR-001, FR-002, FR-003 |
| FND-005 | low | `quire validate --scope . "spec/**/*.md"` exits 0 with no errors or warnings attributable to these six files. The remaining `DuplicateArchetype`/`DuplicateInverseEdge` warnings are module-registration artifacts unrelated to file content and pre-date this change. | (all) |
