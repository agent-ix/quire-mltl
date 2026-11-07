---
id: SR-008
title: "IR-1377 acceptance evidence review"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-mltl; spec/requirements/FR-001-admit-temporal-assessment-requests-and-results.md, tests/request_input_type_identity.rs, Cargo.lock"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-mltl/FR-001
    type: reviews
---

# IR-1377 acceptance evidence review

## Summary

Ticket: IR-1377. Reviewed frozen PR #17 head b94b18e against origin/main. Examined FR-001-AC-10: A consumer can construct `RequestInput` and `ObservationInputs` directly from the public `tl-mltl`, `tl-syntax`, and `quire-observation` branch-main cohort types; each field has one compile-time type identity with its owner crate, with no conversion or duplicate rev-pinned identity.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-001-AC-10 has a correctly tagged compile test, but the transitive rev-pinned tl-syntax dependency prevents its compile oracle from succeeding | Cargo.lock:252 |

## Verdict

**FAIL** for FR-001-AC-10 at this head. The test has a valid Trace tag and its typed helper signatures and struct literals make a wrong public field identity a compile failure. Compilation remains blocked by the upstream tl-mltl edge reported by the coder. Plan completion: not assessed. Other pre-existing criteria are outside this diff-scoped review.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-002 | medium | FR-001-AC-10 compile test cannot fail on the old rev-pinned behaviour: it only names the crate's own resolved deps, so it also compiles on origin/main with rev pins | tests/request_input_type_identity.rs:9-56 |

## New findings (disposition pass 2)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | low | Outside-consumer check runs --offline with no copy of quire-mltl's Cargo.lock and keeps its own lock under ./target across runs, so the git SHAs it builds against come from cache/leftover state, not quire-mltl's lock | tests/request_input_type_identity.rs:89-108 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | At 62ae516 Cargo.lock has a single tl-syntax package (branch=main d97d3d7) shared by quire-mltl and tl-mltl e032a3e; the compile oracle now builds and request_input_accepts_cohort_owner_types passes under --locked. |
| FND-002 | fixed | 6f26192 generates an outside crate with its own branch=main tl-syntax/tl-mltl/quire-observation deps and a path dep on quire-mltl, then cargo-checks it. Mutation at 6f26192: tl-mltl changed to rev=e032a3e (same SHA, different source) keeps quire-mltl's lib compiling, but the consumer fails with 2x E0308 (ValidatedTrace, ValidatedPositionHistory) and the test goes red; the replaced compile-only test would have passed. Unmutated: passes. |
| FND-003 | fixed | 1165733 copies the root Cargo.lock over the consumer lock before every check and places the consumer under $CARGO_TARGET_DIR/outside-consumer (fallback CARGO_MANIFEST_DIR/target). Verified with CARGO_TARGET_DIR set elsewhere: test passes, nothing written to ./target, consumer lock pins tl-syntax d97d3d7, tl-mltl e032a3e, quire-observation 4d5198c, identical to the root lock. |
