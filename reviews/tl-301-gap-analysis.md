---
id: SR-014
title: "TL-301 gap-analysis review"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-mltl; TL-301; src/report.rs; tests/tc_088_nonevaluating_read.rs; spec/requirements/FR-001-admit-temporal-assessment-requests-and-results.md"
review_set: subset
---

## Summary

Ticket: TL-301; PR: quire-mltl#27; reviewed head 7a2f0a98e8d7c59fd1d75e7b62f48a71232b65c5. Planless gap analysis of the diff against FR-001-AC-13 using `quire matrix`. Plan completion: not assessed. FR-001-AC-13 is tagged by three tests in tests/tc_088_nonevaluating_read.rs. The truth-carrying test is a genuine non-evaluation witness. The refusal tests do not exercise the binding the ticket most depends on: a re-identified forged final result with altered decision support, and a re-identified document whose request-echo fields were altered.

## Verdict

**CONDITIONAL** — AC-13 is implemented and tagged; `read_returns_recorded_truth_without_running_an_evaluator` proves no evaluator runs (a re-identified opposite-truth document is accepted verbatim). Request-identity substitution is covered elsewhere (tc_086 reads `other_result` against `request` and gets `ExpectedMismatch`). Coverage gaps remain on the decision-support and echoed-field binding and on the `InvalidCombination` refusal AC-13 lists. FR-001-CON-1 enforcement on the result reader is a code defect reported in SR-013 FND-001 and not repeated here.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | No test refuses a re-identified forged document whose binding fields were altered: a final result with added/removed decision-support identities (`decisionSupport`) or a changed echoed request field (for example `subjectIdentity` or `anchor`) with a recomputed identity. The only `ExpectedMismatch` test in tc_088 uses a revision mismatch via `Superseding(&original)`. | tests/tc_088_nonevaluating_read.rs:160-175 |
| FND-002 | low | AC-13 lists `InvalidCombination` as a refusal of `report::read`, but no test reaches it (for example a re-identified `final: true` with `truth: pending`, or a completed result with `settlement: unavailable`). | tests/tc_088_nonevaluating_read.rs:120-158 |

## Scope

- FR-001-AC-13; role: examined; spec/requirements/FR-001-admit-temporal-assessment-requests-and-results.md; excerpt: `report::read` of a produced result is non-evaluating: within the input-byte ceiling it decodes the typed wire, requires canonical bytes, content identity and internal state consistency, and binds the document to the supplied request and relation (request identity and digest, echoed request fields, lineage revision and predecessor digest, and, for a final result, the exact available decision-support identities) without invoking any evaluator. It carries the recorded execution, truth and settlement and never re-derives them, and refuses with a distinct typed `OwnerReadErrorCode` (`ResourceIncomplete`, `InvalidUtf8`, `InvalidJson`, `NonCanonical`, `ContractMismatch`, `IdentityMismatch`, `InvalidCombination`, `ExpectedMismatch`).
- FR-001-CON-1; role: context_only; spec/requirements/FR-001-admit-temporal-assessment-requests-and-results.md; excerpt: Every owner-declared byte, depth, string, formula-node, position, proposition, support, and history-span ceiling is enforced before allocation or traversal completes; one-over refuses with a typed resource-incomplete outcome and no partial document.
- FR-001-CON-2; role: context_only; spec/requirements/FR-001-admit-temporal-assessment-requests-and-results.md; excerpt: The mapping and result modules accept no caller-supplied truth, execution, or settlement value; every field is re-derived only from the admitted request and the evaluator's own outcome.

## Evidence

- `quire matrix`: FR-001-AC-13 tagged by tests/tc_088_nonevaluating_read.rs:105, :122, :164. FR-001-CON-1 and FR-001-CON-2 untagged (pre-existing).
- Bindings: tc_088 `read_returns_recorded_truth_without_running_an_evaluator` correctly witnesses the non-evaluation clause; `read_refuses_with_typed_codes` covers ResourceIncomplete (one byte under input ceiling), InvalidUtf8, InvalidJson (empty object and trailing data), NonCanonical, IdentityMismatch; `read_refuses_a_document_that_does_not_match_the_supplied_relation` covers ExpectedMismatch via revision/relation. ContractMismatch on `report::read` is exercised by tc_086:628-638 (mutated contract label).
- Request-identity substitution: tc_086_native_correspondence.rs:280-287 reads another request's result against `request` and asserts ExpectedMismatch.
- Existing callers of `report::read` (tc_084, tc_084_full_predecessor, tc_086, tc_087) pass unchanged under `make ci` on this head; the FR-005 census still evaluates through `report::evaluate` before reading, so its re-derived outcome is unaffected.
- No production code outside the AC-13 scope was added; `bind_request` and `recorded_usage` are owned by FR-001-AC-13.

## Disposition round 1 supplement

Reviewed head 1751b3cc00dad2d870477ff6581642aedaf5a949. All findings are fixed on the exercised source; no new findings. Behavior judged: the lexical ceilings (depth, string bytes, visited fields) are now checked after typed decode, on the re-encoded canonical bytes, rather than by a pre-decode scan. This is acceptable: before decode the work is bounded by the effective `max_input_bytes` and serde_json's recursion limit, decode allocation is linear in those bytes, the refusal is still a typed `ResourceIncomplete` naming the field, and no partial document is returned. The visible differences are refusal precedence (a document that is both over a lexical ceiling and otherwise invalid may report the other code first) and that a string over `max_string_bytes` is allocated once before refusal; neither breaks AC-13 or the bounded-work guarantee. `tl_mltl::wire::common::read`/`preflight` are crate-private, so a pre-decode scan would need a tl-mltl API change. `make ci` passed once on this head.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Round 1, 1751b3c: read_refuses_forged_support_and_echoed_request_fields refuses re-identified documents with a changed anchor, a changed subjectIdentity and emptied decision support, each with ExpectedMismatch. |
| FND-002 | fixed | Round 1, 1751b3c: read_reaches_invalid_combination refuses a re-identified original at revision 2 with InvalidCombination. |
