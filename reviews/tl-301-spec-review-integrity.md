---
id: SR-015
title: "TL-301 spec-review-integrity review"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-mltl; TL-301; spec/requirements/FR-001-admit-temporal-assessment-requests-and-results.md; spec/spec.md"
review_set: subset
---

## Summary

Ticket: TL-301; PR: quire-mltl#27; reviewed head 7a2f0a98e8d7c59fd1d75e7b62f48a71232b65c5. Integrity review of the one added criterion, FR-001-AC-13, against the rest of FR-001 and spec/spec.md. AC-13 describes the implementation accurately, but the PR left an older constraint and the system overview stating the opposite trust model.

## Verdict

**FAIL (changes required)** — AC-13 matches the code except that it bounds the read only by the input-byte ceiling, which narrows FR-001-CON-1 for this reader without saying so. FR-001-CON-2 and spec/spec.md still promise that every result field, including truth, is re-derived from the evaluator's own outcome and that no branch accepts an asserted fact it did not validate; AC-13 now states the reader carries recorded truth. The spec contradicts itself until CON-2 and the overview are reworded to the new trust model (recorded truth bound by content identity and request/relation binding, not by re-evaluation).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-001-CON-2 ("every field is re-derived only from the admitted request and the evaluator's own outcome") contradicts FR-001-AC-13 ("carries the recorded execution, truth and settlement and never re-derives them"); spec/spec.md:117-119 ("independently re-derivable by a strict reader; no branch accepts a caller-asserted fact it did not itself validate") is equally stale. | spec/requirements/FR-001-admit-temporal-assessment-requests-and-results.md:173 |
| FND-002 | medium | AC-13 bounds the read only "within the input-byte ceiling", silently exempting the result reader from the depth, string-byte and visited-field ceilings FR-001-CON-1 and FR-001-AC-6 require of every reader; it should require all owner ceilings (and the code should enforce them, SR-013 FND-001). | spec/requirements/FR-001-admit-temporal-assessment-requests-and-results.md:191 |

## Scope

- FR-001-AC-13; role: examined; spec/requirements/FR-001-admit-temporal-assessment-requests-and-results.md; excerpt: `report::read` of a produced result is non-evaluating: within the input-byte ceiling it decodes the typed wire, requires canonical bytes, content identity and internal state consistency, and binds the document to the supplied request and relation (request identity and digest, echoed request fields, lineage revision and predecessor digest, and, for a final result, the exact available decision-support identities) without invoking any evaluator. It carries the recorded execution, truth and settlement and never re-derives them, and refuses with a distinct typed `OwnerReadErrorCode`.
- FR-001-CON-2; role: examined; spec/requirements/FR-001-admit-temporal-assessment-requests-and-results.md; excerpt: The mapping and result modules accept no caller-supplied truth, execution, or settlement value; every field is re-derived only from the admitted request and the evaluator's own outcome.
- FR-001-CON-1; role: examined; spec/requirements/FR-001-admit-temporal-assessment-requests-and-results.md; excerpt: Every owner-declared byte, depth, string, formula-node, position, proposition, support, and history-span ceiling is enforced before allocation or traversal completes; one-over refuses with a typed resource-incomplete outcome and no partial document.
- FR-001-AC-6; role: context_only; spec/requirements/FR-001-admit-temporal-assessment-requests-and-results.md; excerpt: Exceeding any owner-declared resource ceiling on the request or the result refuses with a typed resource-incomplete outcome and no partial document is emitted; the same input one unit under the ceiling is admitted.
- spec/spec.md overview; role: examined; spec/spec.md; excerpt: Every emitted document is bounded, canonical, and independently re-derivable by a strict reader; no branch accepts a caller-asserted fact it did not itself validate against the supplied owner views.

## Evidence

- AC-13 vs code: decode with unknown fields refused (ResultWire is `deny_unknown_fields`), canonical re-serialization equality, content identity recomputation and state-combination checks (validate_wire), request/relation binding (bind_request + relation_wire) and the final-result support set all match src/report.rs. The read can also return `Encoding` (re-serialization failure), which AC-13's code list omits; that path is effectively unreachable for a just-decoded value, so it is not raised as a finding.
- FR-001 body (line ~162) "Every reader independently re-derives the applicable identity" still holds: identity is still recomputed.
- FR-005-AC-4's replay re-derivation is unaffected: the census evaluates through `report::evaluate` before reading.
- Sub-analyses not selected: EARS (no new requirement statement, only an AC row), object/dependency (no new domain object or relationship edge), security (no authentication or cryptographic authority added; the content identity was never an authenticity mechanism).
