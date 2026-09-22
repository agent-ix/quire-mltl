---
id: MRS-001
title: quire-mltl v0.1 master requirements
type: MasterRequirements
relationships:
  - target: ix://agent-ix/tl-mltl/MRS-001
    type: depends_on
  - target: ix://agent-ix/quire-observation/MRS-001
    type: depends_on
  - target: ix://agent-ix/quire-contract-ir/PGM-01
    type: depends_on
---

# Master Requirements Specification

## Purpose

quire-mltl bridges `tl-mltl`'s TL-owned temporal evaluation types to
`quire-observation`'s owner-assertion views. It is the one deliberate
exception to `tl-mltl`'s independence from the agent-ix/Quire ecosystem: an
architect ruling (Linear epic
[TL-175](https://linear.app/agent-ix/issue/TL-175)) requires every TL-*
crate to stay independent of Quire, with only a dedicated integration crate
permitted to bridge them. `quire-mltl` exists so `tl-mltl` itself never has to
depend on, or know about, `quire-observation`.

This specification owns the wire contracts and mapping logic being ported out
of `tl-mltl` under this same epic (TL-178): the temporal-assessment
request/result contract pair, the QObs C00 compatibility dispatch shim, and
the Contract-IR result mapping. It does not re-derive temporal evaluation
semantics, observation-authority semantics, or formula/trace syntax — those
stay owned by `tl-mltl`, `quire-observation`, and `tl-syntax` respectively,
and `quire-mltl` consumes them as dependencies.

### Why quire-mltl is Quire-governed and tl-mltl is not

`tl-mltl` is having its own `ix://agent-ix/quire-contract-ir/PGM-01` citation
removed (TL-180) as part of becoming fully independent of the Quire ecosystem
— an ordinary Rust crate that happens to be usable by Quire, not a component
of it. `quire-mltl` takes the opposite posture on purpose: it directly
incorporates `quire-observation` (a Quire-governed observation boundary) and
exists specifically to hand Quire-shaped evidence — Contract-IR mappings,
owner-assertion-qualified temporal results — back into the ecosystem. It is
not an ordinary consumer of Quire types; it is the load-bearing seam between a
Quire-independent evaluator and Quire's own observation authority. Under PGM-01-R07's component/artifact
classification criterion, that seam role makes `quire-mltl` itself a
Quire-owned program repository, not merely a consumer of one; PGM-01 at
`ix://agent-ix/quire-contract-ir/PGM-01` therefore governs this crate's
compatibility, provenance, evidence, human authority, and qualification
boundaries directly, the same way it would govern any other Quire-owned
program repository. The two repositories are not inconsistent with each other
— they are the two halves of the same boundary, each stating its own
governance posture correctly.

## Scope

### In Scope

- Owner-strict `tl-mltl.temporal-assessment-request/v1` derivation and reading
  against independently supplied `quire_observation::authority::{clock,
  progress, closure, completeness, availability}` owner views, plus
  `tl-syntax` formula/proposition-map documents and a `tl-mltl` trace or
  origin-complete history.
- Owner-strict `tl-mltl.temporal-assessment-result/v1` evaluation and reading,
  delegating the actual future/past truth evaluation to `tl-mltl`'s existing
  evaluators and preserving every owner assertion, settlement basis, decision
  support and correction relation the request carried.
- QObs C00 compatibility dispatch (the `dispatch` module, renamed from
  `tl-mltl`'s `wire::observation`): delegates the supported temporal handoff
  to request derivation with no duplicated evaluation or observation-authority
  logic, and returns an explicit typed unsupported outcome — naming the exact
  foreign contract and compiled QObs revision — for the QObs-owned
  repair-plan and closed-population-query contracts, without accepting,
  inspecting, or mutating either artifact.
- Deriving a TL-owned `tl-mltl.contract-ir-result-map/v1` value-or-typed-non-value
  outcome from a validated temporal result (`mapping::contract_ir`), re-derived
  entirely from that result with no caller-supplied fields, label parser,
  callback, or trust flag.
- The strict reading, canonical-bytes, content-identity, and bounded-resource
  discipline `tl-mltl`'s existing owner boundary established, applied to the
  four ported contracts.

### Out of Scope

- Temporal evaluation semantics themselves — future/past truth evaluation,
  horizon analysis, decision-scope semantics — remain owned by `tl-mltl` and
  are consumed, not reimplemented.
- Observation admission, replay, revisioned authority derivation, repair
  coordination, and closed-population query evaluation remain owned by
  `quire-observation` and are consumed, not reimplemented or projected.
- Formula and proposition-map parsing and strict syntax validation remain
  owned by `tl-syntax`.
- Contract-IR vocabulary, parsers, evaluators, Boolean coercion helpers,
  callbacks, or trust flags — the mapping accepts no caller-supplied result
  fields and owns no Contract-IR wire type of its own.
- `tl-mltl`'s other owner contracts (`trace`, `command`, and the legacy
  aliases) and CLI surface, which are not part of the TL-178 port and remain
  in `tl-mltl` only.

## System Overview

`quire-mltl` sits directly between `tl-mltl` and `quire-observation`: it
depends on both crates (plus `tl-syntax`) rather than either depending on it
or on each other. A caller independently strict-reads the
`quire_observation::authority` owner views and the `tl-syntax`/`tl-mltl`
formula, proposition-map, trace, or history documents it needs, and supplies
all of them to `quire-mltl`. `quire-mltl` binds them into one immutable
canonical temporal-assessment request, evaluates it through `tl-mltl`'s
existing evaluators, emits one immutable canonical result, and can derive a
separate TL-owned Contract-IR mapping view from that result. Every emitted
document is bounded, canonical, and independently re-derivable by a strict
reader; no branch accepts a caller-asserted fact it did not itself validate
against the supplied owner views.

## Requirements Architecture

FR-001 owns temporal-assessment request derivation, strict reading, and
result evaluation. FR-002 owns the QObs C00 compatibility dispatch boundary
and depends on FR-001 for its one supported branch. FR-003 owns the
Contract-IR result mapping and depends on FR-001 for the validated result it
maps. NFR-001 constrains this crate's governance, provenance, and
qualification boundary under PGM-01.

## References

- [Linear epic TL-175](https://linear.app/agent-ix/issue/TL-175) — TL-*
  crates stay independent of the Quire ecosystem; `quire-mltl` is the one
  dedicated bridge.
- TL-176 scaffolded this repository; TL-177 (this specification) precedes
  TL-178, which performs the wholesale port of `wire::request`,
  `wire::observation` (as `dispatch`), `wire::report`, and
  `mapping::contract_ir` out of `tl-mltl`.
- `tl-mltl`'s own
  [FR-018](https://github.com/agent-ix/tl-mltl/blob/main/spec/requirements/FR-018-publish-temporal-owner-wire.md)
  and
  [FR-019](https://github.com/agent-ix/tl-mltl/blob/main/spec/requirements/FR-019-consume-qobs-c00.md)
  describe this same wire boundary as it exists today in `tl-mltl`, before the
  port. Neither is retained unedited: TL-179 rewrites FR-018 to describe a
  generic TL-owned assertion boundary once `tl-mltl` drops its
  `quire-observation` dependency, and TL-180 retires FR-019
  (`status: superseded`) once that consumption path moves to this crate's
  FR-002. This ticket (TL-177) does not itself edit either file — those edits
  land under TL-179/TL-180 — but the References above point at FR-018/FR-019
  as they exist today, not at artifacts that stay frozen.
- [PGM-01](https://github.com/agent-ix/quire-contract-ir/blob/main/spec/program/PGM-01-governance.md)
  governs this crate directly.
