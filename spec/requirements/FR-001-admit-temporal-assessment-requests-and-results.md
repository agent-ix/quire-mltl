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
evaluation semantic they already carry.

## Inputs

- A `tl-syntax` formula document (formula-v1 for the future lane, formula-v2
  for the past lane) and a matching proposition map.
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
- A typed `OwnerReadError` (contract mismatch, expected mismatch, identity
  mismatch, invalid combination, resource-incomplete, or encoding) for any
  invalid input, with no partial request or result document emitted.

## Behavior

- A request selects exactly one lane: formula-v1 plus a trace under
  `mltl.closed-trace/v1` or `mltl.online-prefix/v1`, or formula-v2 plus an
  origin-complete history under `mltl.origin-complete-history/v1`. A future
  formula containing a past-operator node, a past formula containing a future
  node, a closed-trace profile paired with an open trace, or a past anchor
  past the history's through-position is refused before evaluation begins.
- The request retains the formula, proposition map, trace/history, clock,
  evaluator, and all four progress/closure owner assertions as exact
  `ArtifactReference`s (contract, schema digest, content identity, revision,
  byte digest), and retains the completeness and availability assertions with
  their full owner fact population. Every one of these facts is echoed
  verbatim from the supplied owner view; none is derived from another.
- The clock's subject/authority SHALL agree with the decision-scope
  progress/closure subject/authority, and the completeness and availability
  assertions SHALL agree with that same subject/authority and population
  identity; the surrounding-execution progress/closure pair SHALL likewise
  agree with each other and the clock. quire-mltl SHALL require the clock's
  selected range length to equal the admitted trace/history position count
  exactly, and SHALL refuse a timestamped-event clock family as unsupported.
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
| FR-001-AC-2 | A future request accepts only formula-v1/non-past-operator graphs under a trace profile consistent with the trace's closure, and a past request accepts only formula-v2/non-future-node graphs under an origin-complete history with an anchor at or before its through-position; every other combination is refused before evaluation. | Test |
| FR-001-AC-3 | The four progress/closure axes and the completeness/availability assertions are echoed verbatim from the supplied owner views with no derivation between them, and a request whose clock/scope/authority identities disagree across those views is refused. | Test |
| FR-001-AC-4 | `evaluate` yields exactly one execution and one truth state per admitted request, sets `final_result` only for a completed Boolean, selects the settlement basis required by lane and closure, and downgrades an over-limit decision-support population to `ResourceIncomplete`/`Unavailable` with an empty support list instead of a partial one. | Test |
| FR-001-AC-5 | A correction is accepted only when the predecessor's subject, correspondence, formula, proposition map, clock, evaluator, anchor, and lane exactly match; it receives a strictly greater revision and its own new identity, and the predecessor's bytes are never mutated. | Test |
| FR-001-AC-6 | Exceeding any owner-declared resource ceiling on the request or the result refuses with a typed resource-incomplete outcome and no partial document is emitted; the same input one unit under the ceiling is admitted. | Test |

## Dependencies

`tl-mltl` FR-018 supplies the future/past evaluation semantics this
requirement's `evaluate` step delegates to and the prior form of this wire
boundary the port preserves behaviorally. `quire-observation` FR-004 supplies
the `authority::{clock,progress,closure,completeness,availability}` owner
artifacts this requirement reads and echoes. `tl-syntax` FR-014 supplies the
strict formula and proposition-map documents this requirement admits.
