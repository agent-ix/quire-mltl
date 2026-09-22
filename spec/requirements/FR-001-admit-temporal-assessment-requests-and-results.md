---
id: FR-001
title: "Admit temporal-assessment requests and emit immutable results against independently supplied QObs owner views"
type: FR
relationships:
  - target: ix://agent-ix/quire-mltl/MRS-001
    type: implements
  - target: ix://agent-ix/tl-mltl/FR-018
    type: depends_on
  - target: ix://agent-ix/quire-observation/FR-004
    type: depends_on
  - target: ix://agent-ix/tl-syntax/FR-014
    type: depends_on
---
# FR-001: Admit temporal-assessment requests and emit immutable results against independently supplied QObs owner views

## Description

When a caller supplies an independently strict-read formula, proposition map,
trace or origin-complete history, clock, and the complete set of
`quire_observation::authority` owner views a temporal assessment depends on,
quire-mltl SHALL bind them into one immutable canonical
`tl-mltl.temporal-assessment-request/v1` document, evaluate it through
`tl-mltl`'s existing future or past evaluators under exactly its selected
profile, and emit one immutable canonical `tl-mltl.temporal-assessment-result/v1`
document. This requirement ports `tl-mltl`'s `wire::request` and `wire::report`
modules unchanged in behavior; it does not add, remove, or reinterpret any
evaluation semantic they already carry. The one exception is the
`observationRevision` wire field: `tl-mltl`'s current `QUIRE_OBSERVATION_REVISION`
constant (`924006300f45b38483be1cbdf99b68f899b7d368`, `src/lib.rs`) has drifted
from `tl-mltl`'s own Cargo pin (`2bdeb833a330bfa777c19eb4c28c423f856f3ba6`)
since commit `7638e2c` re-pinned the dependency without updating the constant
or the test that checks it. quire-mltl's port re-derives this constant from
its own Cargo pin (see [FR-002](./FR-002-dispatch-qobs-c00-compatibility.md))
rather than porting the drift forward, so the `observationRevision` value this
crate emits is not byte-for-byte identical to `tl-mltl`'s current (stale)
output for the same inputs.

## Inputs

- A `tl-syntax` formula document (formula-v1 for the future lane, formula-v2
  for the past lane) and a matching proposition map: every `Proposition` node
  id the formula references SHALL be present in the proposition map, or the
  request is refused.
- A `tl-mltl` validated trace (future lane) or validated origin-complete
  position history (past lane).
- An independently strict-read `quire_observation::authority::clock::View`.
- Four independent `quire_observation::authority` progress/closure owner
  views: decision-scope progress, decision-scope closure, surrounding-execution
  progress, surrounding-execution closure.
- An independent `quire_observation::authority::completeness::View` and
  `quire_observation::authority::availability::View`.
- Exact native subject and native/TL correspondence identities, an explicit
  evaluation anchor, and caller-lowered `OwnerLimits`.
- For result evaluation: the admitted `ValidatedTemporalRequest` plus an
  explicit `ResultRelationInput` (`Original`, or the exact predecessor result
  being `Superseding`/`Invalidating`).

## Outputs

- A canonical `tl-mltl.temporal-assessment-request/v1` document whose identity
  is a domain-separated lowercase SHA-256 digest over its own canonical bytes
  with the `identity` field omitted, and a constructor-private
  `ValidatedTemporalRequest` when independently strict-read against the same
  expected inputs.
- A canonical `tl-mltl.temporal-assessment-result/v1` document binding the
  evaluated truth or non-value to the request identity, every owner
  assertion/state it carried, settlement basis, exact decision support, and
  original/superseding/invalidating direct-predecessor relation.
- A typed `OwnerReadError` for any invalid input, with no partial request or
  result document emitted, carrying one of the ten `OwnerReadErrorCode`
  variants: `InvalidUtf8`, `InvalidJson`, `NonCanonical`, `ResourceIncomplete`,
  `ContractMismatch`, `ExpectedMismatch`, `IdentityMismatch`,
  `InvalidCombination`, `Encoding`, or `EvaluationRefused`. `NonCanonical` is
  central to the canonical-bytes discipline MRS-001 requires: a strict reader
  re-serializes the decoded value and byte-compares the result against the
  input bytes, refusing with `NonCanonical` on any disagreement even when the
  decoded value is otherwise semantically valid.

## Behavior

- A request selects exactly one lane: formula-v1 carrying a `tl_syntax::SemanticProfile`
  of `mltl.closed-trace/v1` or `mltl.online-prefix/v1`, paired with a trace
  under the `tl-mltl.trace/v1` artifact contract, or formula-v2 carrying
  `mltl.origin-complete-history/v1`, paired with a history under the
  `tl-mltl.position-history/v1` artifact contract. The semantic profile is a
  property of the formula (`tl_syntax::SemanticProfile`), distinct from the
  trace/history `ArtifactReference.contract` field, which always reads
  `tl-mltl.trace/v1` or `tl-mltl.position-history/v1` regardless of which
  semantic profile the paired formula carries. A future formula containing a
  past-operator node, a past formula containing a future node, a closed-trace
  semantic profile paired with an open trace, or a past anchor past the
  history's through-position is refused before evaluation begins.
- Every `NodeKind::Proposition` id the formula references SHALL be present in
  the supplied proposition map; a formula referencing an id absent from the
  map is refused with `ExpectedMismatch("propositionMap")` before evaluation
  begins.
- The request retains the formula, proposition map, trace/history, and all
  four progress/closure owner assertions as exact `ArtifactReference`s
  (contract, schema digest, content identity, revision, byte digest), and
  retains the completeness and availability assertions with their full owner
  fact population. Of these, only the clock, the four progress/closure axes
  (decision-scope progress/closure, surrounding-execution progress/closure),
  and the completeness/availability assertions are echoed verbatim from the
  independently supplied `quire_observation::authority` owner views, with no
  derivation between them. The formula and proposition-map references are, by
  contrast, derived by quire-mltl itself: their `identity` is each supplied
  document's own re-derived content identity and their `digest` a SHA-256 over
  that document's own canonical bytes — computed from the supplied
  formula/proposition-map documents, not copied from any `quire_observation`
  owner view. The evaluator reference is not an `ArtifactReference` at all: it
  is a distinct `EvaluatorReference` (identity, revision, implementation
  digest) synthesized entirely in-crate from the request's selected lane and
  the compiled source revision, not read or echoed from any supplied input.
- The clock's subject/authority SHALL agree with the decision-scope
  progress/closure subject/authority, and the completeness and availability
  assertions SHALL agree with that same subject/authority and population
  identity; the surrounding-execution progress/closure pair SHALL likewise
  agree with each other and the clock. quire-mltl SHALL require the clock's
  selected range length to equal the admitted trace/history position count
  exactly, and SHALL refuse a timestamped-event clock family as unsupported.
- For the past lane, the clock's selected range family SHALL agree with the
  origin-complete history's own clock binding: an `EventPosition` clock range
  requires an `EventPosition` history binding, and a `FixedSample` clock range
  requires a `FixedSample` history binding whose unit is `nanoseconds`, whose
  epoch and period denominators are both `1`, and whose epoch/period
  numerators exactly equal the clock's `epochNanos`/`periodNanos`; every other
  pairing (including a history binding absent for a past request) is refused
  with `ExpectedMismatch("clockBinding")`.
- `evaluate` executes exactly the evaluator selected by the request's lane
  (`tl-mltl`'s future evaluator for the future lane, its past evaluator for
  the past lane) and yields exactly one `AssessmentExecution`
  (`Completed`, `ResourceIncomplete`, `Unsupported`, `Failed`, or `Refused`)
  and exactly one `TemporalTruth` (`Satisfied`, `Violated`, `Pending`, or
  `Unavailable`). `final_result` is true only for a `Completed` execution with
  a `Satisfied` or `Violated` truth. The settlement basis is `ClosedScope` for
  a completed past result or a completed closed-future Boolean,
  `DecisiveWitness`/`DecisiveCounterexample` for a completed open-future
  Boolean, `Unsettled` for a completed pending prefix, and `Unavailable` for
  every non-completed execution.
- Decision support is the sorted, de-duplicated set of observation identities
  behind every `Available` completeness fact, populated only on a final
  result. When that population would exceed the caller's support ceiling, the
  entire result is downgraded to `ResourceIncomplete`/`Unavailable` with an
  empty support list rather than emitting a partial support population.
- A correction (`Superseding` or `Invalidating`) is accepted only when the
  predecessor's subject, correspondence, formula, proposition map, clock,
  evaluator, anchor, and lane exactly match the current request. Its revision
  is the predecessor's revision plus one, it records the predecessor's exact
  identity and byte digest, and the predecessor's own bytes are never
  mutated. An `Original` result always carries revision one and no
  predecessor reference.
- Every reader independently re-derives the applicable identity from the
  decoded document's own canonical bytes and rejects a document whose
  recomputed identity disagrees, whose contract label is wrong, whose
  dependency revisions (`tl-syntax`, `quire-observation`) or limits disagree
  with the compiled crate, or whose execution/truth/settlement/relation
  combination is not one of the valid combinations above.

## Constraints

| ID | Constraint | Type | Validation |
|---|---|---|---|
| FR-001-CON-1 | Every owner-declared byte, depth, string, formula-node, position, proposition, support, and history-span ceiling is enforced before allocation or traversal completes; one-over refuses with a typed resource-incomplete outcome and no partial document. | Resource | Test |
| FR-001-CON-2 | The mapping and result modules accept no caller-supplied truth, execution, or settlement value; every field is re-derived only from the admitted request and the evaluator's own outcome. | Integrity | Test |

## Acceptance Criteria

| ID | Criteria | Verification |
|---|---|---|
| FR-001-AC-1 | Given valid independently-strict-read formula/proposition-map/trace-or-history/clock and all owner assertion views, `derive` produces one canonical request document with a domain-separated SHA-256 identity over its own canonical bytes, and `read` strict-reads it back to a constructor-private view only when every supplied owner input matches exactly. | Test |
| FR-001-AC-2 | A future request accepts only formula-v1/non-past-operator graphs under a semantic profile consistent with the trace's closure, and a past request accepts only formula-v2/non-future-node graphs under an origin-complete history with an anchor at or before its through-position; every other combination is refused before evaluation. | Test |
| FR-001-AC-3 | The clock, the four progress/closure axes, and the completeness/availability assertions are echoed verbatim from the supplied `quire_observation` owner views with no derivation between them, and a request whose clock/scope/authority identities disagree across those views is refused; the formula and proposition-map `ArtifactReference`s are independently derived from the supplied documents' own content identity and canonical bytes, and the evaluator reference is a distinct `EvaluatorReference` synthesized in-crate, neither read from a supplied view. | Test |
| FR-001-AC-4 | `evaluate` yields exactly one execution and one truth state per admitted request, sets `final_result` only for a completed Boolean, selects the settlement basis required by lane and closure, and downgrades an over-limit decision-support population to `ResourceIncomplete`/`Unavailable` with an empty support list instead of a partial one. | Test |
| FR-001-AC-5 | A correction is accepted only when the predecessor's subject, correspondence, formula, proposition map, clock, evaluator, anchor, and lane exactly match; it receives a strictly greater revision and its own new identity, and the predecessor's bytes are never mutated. | Test |
| FR-001-AC-6 | Exceeding any owner-declared resource ceiling on the request or the result refuses with a typed resource-incomplete outcome and no partial document is emitted; the same input one unit under the ceiling is admitted. | Test |
| FR-001-AC-7 | A structurally valid request or result document whose bytes do not equal the canonical re-serialization of their own decoded value is refused with `OwnerReadErrorCode::NonCanonical`, even when every field value is otherwise valid. | Test |
| FR-001-AC-8 | A formula referencing a `Proposition` node id absent from the supplied proposition map is refused with `ExpectedMismatch("propositionMap")` before evaluation begins; the same formula against a proposition map that includes every referenced id is admitted. | Test |
| FR-001-AC-9 | For the past lane, a clock/history-binding pairing other than `EventPosition`/`EventPosition` or an exactly-agreeing `FixedSample`/`FixedSample` pair (nanosecond unit, denominator 1 on both epoch and period, numerators equal to the clock's `epochNanos`/`periodNanos`) is refused with `ExpectedMismatch("clockBinding")`. | Test |

## Dependencies

`tl-mltl` FR-018 supplies the future/past evaluation semantics this
requirement's `evaluate` step delegates to and the prior form of this wire
boundary the port preserves behaviorally. This `depends_on` edge points at
FR-018 as it stands at authoring time, not a frozen artifact: TL-179 rewrites
FR-018 into a generic owner boundary once `tl-mltl` drops its
`quire-observation` dependency, and this requirement's own evaluation-delegation
behavior is unaffected by that rewrite. `quire-observation` FR-004 supplies
the `authority::{clock,progress,closure,completeness,availability}` owner
artifacts this requirement reads and echoes. `tl-syntax` FR-014 supplies the
strict formula and proposition-map documents this requirement admits.
