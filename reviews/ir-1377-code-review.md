---
id: SR-007
title: "IR-1377 code and Rust review"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-mltl; Cargo.toml, Cargo.lock, tests/request_input_type_identity.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-mltl/FR-001
    type: reviews
---

# IR-1377 code and Rust review

## Summary

Ticket: IR-1377. Reviewed frozen PR #17 head b94b18e against origin/main. Examined FR-001-AC-10: A consumer can construct `RequestInput` and `ObservationInputs` directly from the public `tl-mltl`, `tl-syntax`, and `quire-observation` branch-main cohort types; each field has one compile-time type identity with its owner crate, with no conversion or duplicate rev-pinned identity.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | tl-mltl still brings a rev-pinned tl-syntax identity into the branch-main cohort, so the crate fails to compile and RequestInput does not have one usable TL type identity | Cargo.lock:252 |

## Verdict

**FAIL** at this head. The direct dependencies use branch=main and the compile-only consumer test covers all RequestInput and ObservationInputs owner fields, future trace, and past history. Cargo.lock records tl-mltl using rev 6aa9b11 and this crate using branch-main d97d3d7. The coder reported cargo check --tests E0308 in src/report.rs from these distinct tl_syntax::Formula identities. Upstream tl-mltl must normalize its internal edge; this review did not rerun the blocked compile. Rust review is folded into this code-review artifact.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-002 | low | New test adds a deprecated `/// Trace: FR-001-AC-10` doc line instead of the ix-trace-rs `#[trace]` attribute | tests/request_input_type_identity.rs:9 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | 62ae516 refreshes Cargo.lock to tl-mltl branch=main e032a3e, whose tl-syntax edge is branch=main; the rev=6aa9b11 tl-syntax package is gone and one tl-syntax (branch=main d97d3d7) remains. `cargo test --locked --test request_input_type_identity`, tc_084/tc_086 and `cargo clippy --locked --all-targets -D warnings` pass at 62ae516. The new InfiniteTraceV1 arm in src/report.rs:510-513 is a correct exhaustive-match refusal; tl-syntax d97d3d7 already rejects that profile at FormulaDocument validation (InfiniteProfileRequiresUnboundedEdition), so the arm is defensive only. |
