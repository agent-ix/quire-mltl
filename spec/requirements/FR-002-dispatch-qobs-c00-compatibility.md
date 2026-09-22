---
id: FR-002
title: "Dispatch QObs C00 compatibility contracts without semantic substitution"
type: FR
relationships:
  - target: ix://agent-ix/quire-mltl/MRS-001
    type: implements
  - target: ix://agent-ix/quire-mltl/FR-001
    type: depends_on
  - target: ix://agent-ix/quire-observation/FR-010
    type: references
  - target: ix://agent-ix/quire-observation/FR-011
    type: references
---
# FR-002: Dispatch QObs C00 compatibility contracts without semantic substitution

## Description

When a caller selects a QObs C00 contract at the `dispatch` boundary
(ported from `tl-mltl`'s `wire::observation`, renamed because this crate is
the compat-dispatch shim rather than the observation owner itself),
quire-mltl SHALL report whether that exact contract is consumed by the
temporal request boundary without accepting, inspecting, or mutating any
artifact of an unsupported contract. When the caller dispatches a temporal
handoff, quire-mltl SHALL delegate production entirely to [FR-001](./FR-001-admit-temporal-assessment-requests-and-results.md)'s
request derivation with no duplicated evaluation or observation-authority
logic.

## Inputs

- A selected `Contract`: `TemporalAssessment`, `RepairPlan`, or
  `ClosedPopulationQuery`.
- For the temporal path, the [FR-001](./FR-001-admit-temporal-assessment-requests-and-results.md)
  `RequestInput` and caller-lowered `OwnerLimits`.

## Outputs

- For `TemporalAssessment`, a `Compatibility::Supported` disposition naming
  the exact wire-contract label and the exact compiled `quire-observation`
  revision this crate's `Cargo.toml` pins, plus (on dispatch) the
  byte-identical `TemporalRequestDocument` a direct `request::derive` call
  would produce for the same inputs and limits.
- For `RepairPlan` or `ClosedPopulationQuery`, a `Compatibility::Unsupported`
  disposition naming the exact foreign QObs contract label
  (`quire.observation.repair-plan/v1` or
  `quire.observation.closed-population-query/v1`) and the same exact compiled
  `quire-observation` revision, with no Boolean, aggregate, repaired result,
  or partial temporal document ever exposed.

## Behavior

- `compatibility(contract)` is a pure, total function of the contract
  selector alone: it never accepts, reads, or inspects a repair-plan or
  closed-population-query artifact to decide support, only the closed
  `Contract` enum value.
- `TemporalAssessment` is the only contract for which `compatibility` returns
  `Supported`; `RepairPlan` and `ClosedPopulationQuery` always return
  `Unsupported`.
- `consume_temporal` delegates directly to [FR-001](./FR-001-admit-temporal-assessment-requests-and-results.md)'s
  `request::derive` under the supplied `OwnerLimits`, duplicating no
  temporal-evaluation or observation-authority logic of its own; its output
  is byte-identical to a direct `request::derive` call for the same inputs
  and limits.
- The repair and closed-population-query branches identify their exact QObs
  contract as unsupported and never accept, inspect, project, evaluate, or
  relabel a foreign artifact; no supported or unsupported disposition may
  substitute a compatible range, ambient checkout, ingestion order, or a
  TL-owned reconstruction of QObs-owned semantics.
- Every supported and unsupported disposition names the same exact compiled
  `quire-observation` revision exported by this crate; the crate's Cargo
  resolution and this exported revision constant name the same commit.

## Constraints

| ID | Constraint | Type | Validation |
|---|---|---|---|
| FR-002-CON-1 | The `quire-observation` Cargo dependency pin and the revision constant exported by `dispatch` name the same exact commit. | Provenance | Test |
| FR-002-CON-2 | Unsupported repair-plan and closed-population-query dispatch never expose a Boolean, aggregate, repaired result, or partial temporal document. | Integrity | Test |

## Acceptance Criteria

| ID | Criteria | Verification |
|---|---|---|
| FR-002-AC-1 | Given valid temporal owner views, `consume_temporal` produces bytes and resource usage identical to a direct FR-001 request-derivation call for the same inputs and limits, under both exact and one-over limits. | Test |
| FR-002-AC-2 | Given the `RepairPlan` or `ClosedPopulationQuery` selector, `compatibility` returns the corresponding typed `Unsupported` disposition and exact compiled `quire-observation` revision, with no value-bearing or artifact input/output at any point. | Test |
| FR-002-AC-3 | Cargo resolution, the exported revision constant, and every supported/unsupported disposition all name the same exact `quire-observation` commit. | Test |

## Dependencies

Depends on [FR-001](./FR-001-admit-temporal-assessment-requests-and-results.md)
for the one supported temporal-assessment dispatch branch. References
`quire-observation` FR-010 (repair-plan/affected-region repair) and FR-011
(closed-population-query evaluation), whose artifacts this requirement
explicitly declines to consume; `quire-observation` retains ownership of both.
