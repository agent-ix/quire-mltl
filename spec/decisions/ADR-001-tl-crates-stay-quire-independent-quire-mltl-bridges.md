---
id: ADR-001
title: "TL-* crates stay independent of Quire; quire-mltl is the one deliberate bridge"
type: ADR
status: accepted
owner: kreneskyp
relationships:
  - target: ix://agent-ix/quire-contract-ir/PGM-01
    type: depends_on
  - target: ix://agent-ix/quire-mltl/MRS-001
    type: relates_to
---
# ADR-001: TL-* crates stay independent of Quire; quire-mltl is the one deliberate bridge

## Status

**Accepted** — architect ruling recorded against Linear epic
[TL-175](https://github.com/agent-ix/tl-mltl/issues/7). TL-176 scaffolded
this repository as the ruling's bridge crate; this document (TL-177) records
the decision itself. TL-178 performs the wholesale port of `tl-mltl`'s
`wire::request`, `wire::observation` (as `dispatch`), `wire::report`, and
`mapping::contract_ir` modules into this crate; TL-180 removes `tl-mltl`'s
`quire-observation` dependency and its `ix://agent-ix/quire-contract-ir/PGM-01`
citation, completing `tl-mltl`'s independence.

## Context

`tl-mltl` is a deterministic reference implementation of bounded
Mission-time Linear Temporal Logic evaluation over finite traces. It had
grown a direct `quire-observation` dependency to carry its
`tl-mltl.temporal-assessment-request/v1` and
`tl-mltl.temporal-assessment-result/v1` wire contracts, which bind
`quire_observation::authority::{clock,progress,closure,completeness,
availability}` owner-assertion views directly into the request/result
documents, plus a QObs C00 compatibility-dispatch shim and a Contract-IR
result mapping. That coupling meant every consumer of `tl-mltl` — including
one that only wants a standalone MLTL evaluator with no Quire involvement at
all — pulled in `quire-observation` and, through it, PGM-01 governance that
has nothing to do with temporal evaluation.

The owner ruled that every TL-* crate (`tl-syntax`, `tl-parse`, `tl-mltl`,
`tl-rewrite`) SHALL stay independent of the agent-ix/Quire ecosystem, and that
only a dedicated integration crate may bridge a TL-* crate to Quire types.
`tl-mltl`'s existing wire/mapping modules are exactly that bridge, misplaced
inside the crate they should not be coupling.

## Decision

Create `quire-mltl` as the one deliberate exception: a dedicated integration
crate that depends on both `tl-mltl` and `quire-observation` (plus
`tl-syntax`) so that neither has to depend on, or know about, the other.

- `tl-mltl`'s `wire::request`, `wire::observation` (renamed `dispatch`),
  `wire::report`, and `mapping::contract_ir` modules move to `quire-mltl`
  unchanged in behavior (TL-178). `tl-mltl` retains only its own
  future/past evaluation semantics, horizon analysis, and the wire contracts
  that carry no `quire-observation` type (`trace`, `command`, legacy
  aliases).
- `tl-mltl` drops its `quire-observation` dependency and its
  `ix://agent-ix/quire-contract-ir/PGM-01` citation (TL-180), becoming an
  ordinary Rust crate usable by, but not part of, the Quire ecosystem.
- `quire-mltl` takes on both the `quire-observation` dependency and the
  `PGM-01` citation directly. It is not an ordinary consumer of Quire types;
  it is the load-bearing seam between a Quire-independent evaluator and
  Quire's own observation authority, and PGM-01 governs it the same way it
  would govern any other Quire-owned program repository. See
  [MRS-001](../spec.md) for the full rationale.
- `quire-mltl` is licensed `AGPL-3.0-or-later`, this org's default for
  `quire-*` program repositories under PGM-01-R04 — not `AGPL-3.0-only`. An
  earlier scaffold draft of this repository declared `-only` as a deliberate
  exception, reasoned from a local `quire-observation` checkout that was
  sitting on a stale branch; `quire-observation`'s actual `origin/main` has
  been `AGPL-3.0-or-later` since its 2026-09-13 relicense, so nothing in this
  crate's dependency graph forces a narrower term. That mistake was caught
  during TL-176 review and corrected in commit `b8b1f8c`
  (`agent-ix/quire-mltl`); this ADR records the corrected, org-default
  decision, not the withdrawn exception.

## Consequences

- `tl-mltl` becomes reusable by any consumer that wants bounded MLTL
  evaluation without accepting a Quire dependency or PGM-01 governance —
  the ruling's stated goal.
- A consumer that specifically needs the QObs-bridged temporal-assessment
  contract, the compatibility dispatch, or the Contract-IR mapping now
  depends on `quire-mltl`, not `tl-mltl`, for that surface.
- `quire-mltl`'s `Cargo.toml` directly pins exact `tl-mltl`,
  `quire-observation`, and `tl-syntax` source revisions; only this crate
  needs to be re-reviewed and repinned when either upstream's Quire-facing
  contract changes, rather than `tl-mltl` itself.
- Only this repository carries the `agent-ix-baseline-producer` dev-dependency
  and the PGM-01 governance obligations (see
  [NFR-001](../requirements/NFR-001-governance-boundary.md)) that follow from
  being the genuinely Quire-governed half of the boundary.

## Alternatives Considered

- **Keep the wire boundary inside `tl-mltl` and let it depend on
  `quire-observation` directly (the prior arrangement).** Rejected: couples
  the reference evaluator to Quire governance and forces every non-Quire
  consumer to accept a dependency and a compatibility posture it does not
  need. This is exactly what the architect ruling prohibits.
- **Merge `quire-observation`'s authority types into `tl-mltl`, or fold
  `tl-mltl` into `quire-observation`.** Rejected: contradicts the existing
  ownership split — `tl-mltl` owns temporal evaluation semantics,
  `quire-observation` owns observation admission, replay, and authority
  derivation. A merge would duplicate governance and blur the PGM-01-R07
  component classification both crates already have.
- **Leave the `quire-observation` dependency optional behind a Cargo feature
  inside `tl-mltl`, default-off.** Rejected: a feature flag is a
  compatibility layer, and this org does not add compatibility layers to
  prerelease software without an asked-for, expiring exception. It also does
  not remove the coupling from `tl-mltl`'s own dependency graph or
  governance posture when the feature is compiled in by any consumer that
  needs it — the ruling requires the dependency to not exist in `tl-mltl` at
  all, not merely to be optional.
