---
id: NFR-001
title: Retain governance and qualification boundaries
type: NFR
quality_attribute: compliance
---
# NFR-001: Retain governance and qualification boundaries

## Statement

Every wire document quire-mltl exchanges SHALL use an explicit supported
schema identity. Agent-produced requests, results, QObs
compatibility dispositions, and Contract-IR mappings SHALL remain distinct
from human approval and consuming-project validation.

## Scope

All `tl-mltl.temporal-assessment-request/v1`,
`tl-mltl.temporal-assessment-result/v1`, `tl-mltl.contract-ir-result-map/v1`
documents, QObs C00 compatibility dispositions, and release claims
quire-mltl makes are in scope.

## Rationale

quire-mltl is the one deliberate exception to `tl-mltl`'s independence from
the Quire ecosystem (see [MRS-001](../spec.md)); it is genuinely
Quire-governed where `tl-mltl` is not. Unidentified schema drift at this
specific bridge would silently reintroduce the
coupling `tl-mltl`'s own independence exists to avoid, and would let a
Quire-shaped result claim authority it does not have.

## Measurement and Evaluation

| Metric | Target | Threshold | Method |
|---|---|---|---|
| Unversioned exchanged document kinds | 0 | 0 | Test |
| Requirement-tagged tests Cargo does not compile and run | 0 | 0 | Test |

## Verification

Schema-negative tests reject an unknown contract identity. A compiled Rust test census re-derives which requirement-tagged
tests Cargo actually runs.

## Acceptance Criteria

| ID | Criteria | Verification |
|---|---|---|
| NFR-001-AC-1 | Unknown schema/contract identities are rejected by every strict reader. | Test |
| NFR-001-AC-2 | Every requirement-tagged Rust test is a test Cargo actually compiles and runs; no compiled requirement-tagged test is ignored or configured out. | Test |
| NFR-001-AC-3 | No request, result, dispatch disposition, or Contract-IR mapping this crate emits asserts that it has been approved, that a consuming project has validated it, or that it constitutes a release decision — that authority remains with the named human release authority. | Inspection |

## Dependencies

Applies schema-identity compatibility and the human release-decision boundary
to every contract this crate owns.
