---
id: FR-003
title: "Derive a TL-owned Contract-IR outcome from a validated temporal result"
type: FR
relationships:
  - target: ix://agent-ix/quire-mltl/MRS-001
    type: implements
  - target: ix://agent-ix/quire-mltl/FR-001
    type: depends_on
---
# FR-003: Derive a TL-owned Contract-IR outcome from a validated temporal result

## Description

When a caller selects a constructor-private `ValidatedTemporalResult` for
Contract-IR handoff, quire-mltl SHALL derive one immutable canonical
`tl-mltl.contract-ir-result-map/v1` document containing either a valid
Boolean `value` or a typed `nonValue`, re-derived entirely from that result
with no caller-supplied result field, label parser, callback, or trust flag.
This requirement ports `tl-mltl`'s `mapping::contract_ir` module unchanged in
behavior; it owns no Contract-IR vocabulary, parser, evaluator, or Boolean
coercion of its own.

## Inputs

- A constructor-private `ValidatedTemporalResult` admitted under
  [FR-001](./FR-001-admit-temporal-assessment-requests-and-results.md).
- A `MappingSelection`, constructible only via `MappingSelection::for_result`,
  which binds the exact selected result's own content identity and byte
  digest.

## Outputs

- A canonical `tl-mltl.contract-ir-result-map/v1` document whose identity is
  a domain-separated lowercase SHA-256 digest over its own canonical bytes,
  distinct from its source result's identity, and which embeds the source
  result's complete wire content (request/formula/input/correspondence
  identities, assessment execution, all four progress/closure owner
  references, completeness/availability reference, settlement, decision
  support, and correction relation) for lossless joining.
- A `MappedOutcome::Value { value }` when, and only when, the source result is
  a completed, final, Satisfied or Violated Boolean with a `Complete`
  completeness state and an `Available` availability state.
- A `MappedOutcome::NonValue { reason }` for every other execution, truth,
  completeness, or availability combination, with `reason` one of `Pending`,
  `Unavailable`, `Incomplete`, `Unsupported`, `Failed`, `Refused`,
  `Contradicted`, or `ResourceIncomplete`.

## Behavior

- `map` refuses with a typed `ExpectedMismatch` when the supplied
  `MappingSelection`'s bound identity or byte digest does not exactly match
  the supplied result; callers cannot select a result by any means other than
  `MappingSelection::for_result`.
- The outcome is derived from the source result's own state alone: a
  `ResourceIncomplete`, `Unsupported`, `Failed`, or `Refused` execution maps
  directly to the matching `NonValueKind`. A `Completed` execution with a
  `Contradicted` or `Incomplete` completeness state maps to `Contradicted` or
  `Incomplete` respectively. A `Completed` execution with `Complete`
  completeness and a not-yet-available/producer-unavailable/contract-unavailable
  availability state maps to `Unavailable`. A `Completed`, `Complete`,
  `Available` result that is not `final` maps to `Pending` when its truth is
  `Pending`, otherwise `Unavailable`. Only a `Completed`, `Complete`,
  `Available`, final `Satisfied`/`Violated` result maps to `Value { true }` or
  `Value { false }`.
- `read` re-derives the complete expected mapping — including its outcome —
  entirely from the exact validated source result and selection, and refuses
  any decoded byte that disagrees with that re-derivation; the mapping is
  never accepted on the strength of its own claimed outcome.
- The mapping's content identity is computed the same domain-separated way as
  every other owner contract in this crate (a SHA-256 over the contract label
  and the document's own canonical bytes with `identity` omitted), keeping it
  distinct from and non-substitutable with the source result's identity.

## Acceptance Criteria

| ID | Criteria | Verification |
|---|---|---|
| FR-003-AC-1 | `map` derives `Value { .. }` only from a completed, final, Satisfied/Violated result with `Complete` completeness and `Available` availability; every other execution/completeness/availability/truth combination maps to the typed `NonValue` reason the source state requires. | Test |
| FR-003-AC-2 | `MappingSelection::for_result` binds only to the exact supplied result's own identity and byte digest; `map` and `read` refuse with `ExpectedMismatch` when a supplied selection does not match that exact result. | Test |
| FR-003-AC-3 | Every mapped document embeds the complete source result wire — assessment execution, owner references, settlement, decision-support identities, and correction relation — so a reader losslessly joins the mapping back to its source without a second fetch. | Test |
| FR-003-AC-4 | The mapping's content identity is a domain-separated SHA-256 distinct from its source result's identity; `read` independently re-derives the full expected mapping, including its outcome, from the exact validated source result and refuses any byte that disagrees. | Test |
| FR-003-AC-5 | `map`/`read` accept no caller-supplied result field, Contract-IR label parser, callback, or trust flag; the outcome is re-derived entirely from the validated source result. | Test |

## Dependencies

Depends on [FR-001](./FR-001-admit-temporal-assessment-requests-and-results.md)
for the constructor-private `ValidatedTemporalResult` this requirement maps;
it adds no evaluation semantics of its own.
