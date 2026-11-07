---
id: SR-007
title: "IR-1379 code-review review"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-mltl; IR-1379; src/report.rs; tests/tc_084_full_predecessor.rs; spec/requirements/FR-001-admit-temporal-assessment-requests-and-results.md"
review_set: subset
---

## Summary

Ticket: IR-1379; PR: quire-mltl#18; reviewed head f4b58df. The complete-byte behavior and refusal branches are sound on static inspection. One low Rust error-contract convention finding remains; runtime evidence is pending.

## Verdict

**CONDITIONAL** — The complete-byte behavior and refusal branches are sound on static inspection. One low Rust error-contract convention finding remains; runtime evidence is pending. Runtime compilation/tests remain pending and the lead final gate is not claimed.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The new public predecessor error bypasses the required stable Copy code catalog and thiserror envelope: it exposes a hand-written enum Display/Error without as_str/all/from_code or the documented envelope. | src/report.rs:100 |

## Scope

- FR-001-AC-11; role: examined; spec/requirements/FR-001-admit-temporal-assessment-requests-and-results.md; excerpt: A validated superseding or invalidating result publicly exposes all canonical bytes of its admitted direct predecessor; an original exposes none. The retained direct predecessor remains complete and unchanged after its source value is dropped, without recursively retaining earlier lineage. Retention is admitted at the current input-byte ceiling and refuses one byte over before copying, even when the predecessor was admitted under a larger earlier ceiling.
- FR-001-AC-12; role: examined; spec/requirements/FR-001-admit-temporal-assessment-requests-and-results.md; excerpt: Non-evaluating supplied-predecessor validation accepts exactly the retained complete canonical bytes (or absence for an original), distinguishes missing, unexpected, and unequal bytes with typed errors, and rejects same-identity/different-value, truncated, extended, or identity-only offers without changing result bytes, usage, or predecessor bytes.
- SuppliedPredecessorError; role: examined; src/report.rs; excerpt: pub enum SuppliedPredecessorError {
    /// A correction's retained predecessor was not supplied.
    Missing,
    /// An original result was supplied a predecessor despite having none.
    Unexpected,
    /// Supplied bytes differ from the complete retained predecessor bytes.
    Mismatch,
}

## Evidence

- Exact slice equality is the only supplied-byte decision: no hash, parser, evaluator, allocation or caller callback appears on the existing validation path.
- Public tests assert complete predecessor bytes for superseding and invalidating results, ownership after dropping source values, distinct typed absence/mismatch cases, same-identity changed content, truncated/extended/malformed/identity-only input, and the current retention byte ceiling.
- Reader retention checks the effective current max_input_bytes before copying, and stores only the direct predecessor canonical Vec, not its recursive validated object.
- Quire validation and matrix were run. The changed FR reports grammar-clean with zero grammar findings; both new ACs have two real test binders each.
- Focused Cargo work is waiting on the shared lock in /tmp/ir1379-focused.log; no runtime pass is asserted. No aggregate build was run by this reviewer.
- Existing dependency normalization is IR-1377 PR17 ownership, not this feature review finding. No manifest, lock or CI workflow file changes are in this diff.
- Applicable rule: rust-style/references/wire-and-persistence.md, Error envelopes: each surfaced error uses a stable Copy code with as_str/all/from_code and a thiserror envelope. Adding an allocating envelope must also keep the authored validation allocation promise truthful; update that promise if necessary rather than claim an allocating error path is allocation-free.

## Assurance Context

No active AssuranceProfile is declared in this repository. No enforced review workflow was started, and no human acceptance transition is claimed. Applicable direct review methods are code-review with Rust lane, gap-analysis, base, EARS conformance, integrity and failure-domain. Dependency/object/security analyses are not selected: no relationship, domain-object artifact, authentication, or cryptographic authority is added.
