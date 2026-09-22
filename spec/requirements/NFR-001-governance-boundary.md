---
id: NFR-001
title: Retain governance and qualification boundaries
type: NFR
quality_attribute: compliance
relationships:
  - target: ix://agent-ix/quire-contract-ir/PGM-01
    type: depends_on
---
# NFR-001: Retain governance and qualification boundaries

## Statement

Every wire document quire-mltl exchanges SHALL use an explicit supported
schema identity, exact source-revision pins for the `tl-mltl`, `tl-syntax`,
and `quire-observation` dependencies it bridges, contribution provenance, and
canonical PGM-01 evidence boundaries. Agent-produced requests, results, QObs
compatibility dispositions, and Contract-IR mappings SHALL remain distinct
from human approval and consuming-project validation.

## Scope

All `tl-mltl.temporal-assessment-request/v1`,
`tl-mltl.temporal-assessment-result/v1`, `tl-mltl.contract-ir-result-map/v1`
documents, QObs C00 compatibility dispositions, cross-repository dependency
pins, and release claims quire-mltl makes are in scope.

## Rationale

quire-mltl is the one deliberate exception to `tl-mltl`'s independence from
the Quire ecosystem (see [MRS-001](../spec.md)); it is genuinely
Quire-governed where `tl-mltl` is not. Unidentified schema, dependency, or
revision drift at this specific bridge would silently reintroduce the
coupling `tl-mltl`'s own independence exists to avoid, and would let a
Quire-shaped result claim authority it does not have.

## Measurement and Evaluation

| Metric | Target | Threshold | Method |
|---|---|---|---|
| Unversioned exchanged document kinds | 0 | 0 | Test |
| Omitted `tl-mltl`/`tl-syntax`/`quire-observation` revision identities in an emitted document | 0 | 0 | Inspection |
| Requirement-tagged tests Cargo does not compile and run | 0 | 0 | Test |

## Verification

Schema-negative tests reject an unknown contract or dependency-revision
identity. A compiled Rust test census re-derives which requirement-tagged
tests Cargo actually runs.

## Acceptance Criteria

| ID | Criteria | Verification |
|---|---|---|
| NFR-001-AC-1 | Unknown schema/contract identities and an omitted `tl-mltl`, `tl-syntax`, or `quire-observation` revision are rejected by every strict reader. | Test |
| NFR-001-AC-2 | Every exchanged record names the exact `tl-mltl`, `tl-syntax`, and `quire-observation` compiled revisions and the exact QObs C00 contract it selected, without recording an automated release or approval decision. | Test |
| NFR-001-AC-3 | Every requirement-tagged Rust test is a test Cargo actually compiles and runs; no compiled requirement-tagged test is ignored or configured out. | Test |
| NFR-001-AC-4 | No request, result, dispatch disposition, or Contract-IR mapping this crate emits asserts that it has been approved, that a consuming project has validated it, or that it constitutes a release decision — that authority remains with the named human release authority under PGM-01-R06/R09. | Inspection |

## Dependencies

Applies `ix://agent-ix/quire-contract-ir/PGM-01` — specifically R01 (schema
compatibility), R02 (crate compatibility and pins), and R06/R09
(contribution provenance and the human release decision) — to every contract
this crate owns.
