---
id: SR-013
title: "TL-301 code-review review"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-mltl; TL-301; src/report.rs; tests/tc_088_nonevaluating_read.rs; spec/requirements/FR-001-admit-temporal-assessment-requests-and-results.md"
review_set: subset
---

## Summary

Ticket: TL-301; PR: quire-mltl#27; reviewed head 7a2f0a98e8d7c59fd1d75e7b62f48a71232b65c5. Code-review with the Rust lane over `git diff origin/main...HEAD`. `report::read` no longer reaches `build_wire`/`execute`: the only remaining callers of the evaluators are `report::evaluate`. Request, relation, lineage and decision-support binding is sound. Two real regressions came from dropping `tl_mltl::wire::common::read_expected`: its lexical preflight (depth, string-byte and visited-field ceilings) and its observed usage are gone, and the user docs still describe an evaluating read.

## Verdict

**FAIL (changes required)** — the non-evaluating read is correct in intent and the binding is sufficient: a re-identified forged document is refused for a different request (identity and digest), relation, revision or predecessor digest, and a final result must carry exactly the request's available decision-support identities. There is no evaluating twin or shim, and `read_expected` is no longer imported in `report.rs`. However the owner-declared depth/string/visited-field ceilings are no longer enforced by this reader (FR-001-CON-1, FR-001-AC-6), the returned usage is wholly document-asserted, and README/docs still state that `report::read` re-evaluates. `make ci` (fmt-check, clippy -D warnings, all tests, deny, docs) passed once on this head.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | `report::read` dropped the `read_expected` lexical preflight, so `max_depth`, `max_string_bytes` and `max_visited_fields` are no longer enforced on a result read; a document over those owner ceilings is admitted, violating FR-001-CON-1 / FR-001-AC-6. | src/report.rs:497-562 |
| FND-002 | medium | Returned `usage` is taken wholly from the document: lexical fields (`wire_bytes`, `depth`, `string_bytes`, `visited_fields`) are recorded 0 instead of observed, request-derived fields (formula nodes/depth, positions, propositions, history span) are not bound to the request, and `validate_wire`'s `wire.usage != usage_wire(usage)` check is now vacuous because `usage` is derived from `wire.usage`. | src/report.rs:621-638 |
| FND-003 | medium | User docs still describe the evaluating read: README.md:16 and :80, docs/how-it-works.md:13, docs/api-reference.md:30 ("Re-evaluates the request") and docs/request-dispatch-report.md:66 ("`report::read` re-evaluates with the same relation"). | docs/api-reference.md:30 |
| FND-004 | low | Rustdoc on `ValidatedTemporalResult::request` still says "used for re-derivation" and `usage` says "read/evaluation work"; neither holds for a read result now. | src/report.rs:449-458 |

## Scope

- FR-001-AC-13; role: examined; spec/requirements/FR-001-admit-temporal-assessment-requests-and-results.md; excerpt: `report::read` of a produced result is non-evaluating: within the input-byte ceiling it decodes the typed wire, requires canonical bytes, content identity and internal state consistency, and binds the document to the supplied request and relation (request identity and digest, echoed request fields, lineage revision and predecessor digest, and, for a final result, the exact available decision-support identities) without invoking any evaluator. It carries the recorded execution, truth and settlement and never re-derives them, and refuses with a distinct typed `OwnerReadErrorCode`.
- FR-001-CON-1; role: examined; spec/requirements/FR-001-admit-temporal-assessment-requests-and-results.md; excerpt: Every owner-declared byte, depth, string, formula-node, position, proposition, support, and history-span ceiling is enforced before allocation or traversal completes; one-over refuses with a typed resource-incomplete outcome and no partial document.
- FR-001-AC-6; role: examined; spec/requirements/FR-001-admit-temporal-assessment-requests-and-results.md; excerpt: Exceeding any owner-declared resource ceiling on the request or the result refuses with a typed resource-incomplete outcome and no partial document is emitted; the same input one unit under the ceiling is admitted.
- report::read; role: examined; src/report.rs; excerpt: pub fn read(bytes, request, relation, limits) -> Result<ValidatedTemporalResult, OwnerReadError>
- report::bind_request; role: examined; src/report.rs; excerpt: fn bind_request(wire, request, relation, usage) -> Result<(), OwnerReadError>
- report::recorded_usage; role: examined; src/report.rs; excerpt: fn recorded_usage(wire: &UsageWire, observed: OwnerUsage) -> Result<OwnerUsage, OwnerReadError>
- tests/tc_088_nonevaluating_read.rs; role: examined; tests/tc_088_nonevaluating_read.rs; excerpt: TC-088: strict read of a produced result is non-evaluating.

## Evidence

- Non-evaluation: `grep build_wire|execute(` in src/report.rs shows `build_wire` called only from `evaluate` (report.rs:472) and `execute` only from `build_wire` (report.rs:651). `read` no longer imports `read_expected`. No twin or shim function exists.
- Binding: `bind_request` rebuilds a `ResultWire` from the request and `relation_wire(request, relation)` and keeps only identity, contract label, execution, truth, final flag, settlement, decision support and usage from the document, then requires equality. `relation_wire` binds revision (1 or predecessor+1), predecessor identity, predecessor digest (raw SHA-256 of the predecessor bytes) and corrected request identity, and refuses a correction context mismatch. A final result's support set must equal the request's available completeness identities; a non-final result must have empty support (validate_wire `support_matches_finality`).
- FND-001: before this PR, `read_expected` called `tl_mltl::wire::common::read`, whose `preflight` enforced `max_depth`, `max_string_bytes` and `max_visited_fields` (tl-mltl e032a3e src/wire/common.rs:560-640). The new `read` checks only `max_input_bytes`, the request population, support, evaluation steps and recursion depth. A local probe (untracked test, deleted after) read a valid produced result with `max_depth: 1`, `max_string_bytes: 1` and `max_visited_fields: 1` respectively: all three returned Ok(Satisfied) (old path refused ResourceIncomplete). The same probe showed `evaluate` usage wire_bytes 5388, depth 4, string_bytes 75, visited_fields 134, while `read` of the same bytes reports 0 for all four (FND-002). No existing test exercises those ceilings on `report::read`, which is why `make ci` stays green.
- FND-002: `recorded_usage` copies `wire.usage`; `validate_wire(&wire, effective, usage)` then compares `wire.usage` with `usage_wire(usage)`, which is the same data. `build_wire` records `wire_bytes`/`depth`/`string_bytes`/`visited_fields` as 0, so `ValidatedTemporalResult::usage()` after a read now reports 0 for them where it previously reported observed lexical usage (merge_usage). The ceiling still caught by validate_wire (support, steps, recursion depth) is correct.
- Rust lane: fmt and clippy -D warnings clean under `make ci`. Error mapping is typed and bounded; `usize::try_from` used for u64 conversion; no panics, unsafe or unbounded allocation beyond the input-byte ceiling. `Encoding` is reachable from the re-serialization path but is not in AC-13's code list (spec-review covers it).
- Gate: `make ci` on 7a2f0a9 passed: fmt-check, clippy, 54 tests across 7 test binaries, cargo deny, cargo doc -D warnings.

## Disposition round 1 supplement

Reviewed head 1751b3cc00dad2d870477ff6581642aedaf5a949. All findings are fixed on the exercised source; no new findings. Behavior judged: the lexical ceilings (depth, string bytes, visited fields) are now checked after typed decode, on the re-encoded canonical bytes, rather than by a pre-decode scan. This is acceptable: before decode the work is bounded by the effective `max_input_bytes` and serde_json's recursion limit, decode allocation is linear in those bytes, the refusal is still a typed `ResourceIncomplete` naming the field, and no partial document is returned. The visible differences are refusal precedence (a document that is both over a lexical ceiling and otherwise invalid may report the other code first) and that a string over `max_string_bytes` is allocated once before refusal; neither breaks AC-13 or the bounded-work guarantee. `tl_mltl::wire::common::read`/`preflight` are crate-private, so a pre-decode scan would need a tl-mltl API change. `make ci` passed once on this head.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Round 1, 1751b3c: read re-encodes through tl_mltl produce, whose preflight enforces max_depth, max_string_bytes and max_visited_fields on the canonical bytes (equal to the read bytes); tc_088 read_enforces_every_ceiling_and_measures_usage admits each at the measured ceiling and refuses one under with ResourceIncomplete and the named field. |
| FND-002 | fixed | Round 1, 1751b3c: recorded_usage removed; semantic usage is built from the request (formula nodes/depth, positions, propositions, history span) and the support list, validate_wire compares the recorded usage against it, and the returned usage merges the lexical usage measured by produce; tc_088 asserts read usage equals evaluate usage. |
| FND-003 | fixed | Round 1, 1751b3c: README.md, docs/api-reference.md, docs/how-it-works.md and docs/request-dispatch-report.md now say report::read binds the recorded result without running the evaluator. |
| FND-004 | fixed | Round 1, 1751b3c: rustdoc on usage() and request() updated. |
