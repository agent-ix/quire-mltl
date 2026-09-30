---
id: SR-004
title: "spec review of quire-mltl#8 (drop dangling PGM-01 citations)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-mltl@a7172647afab6b916c1366d948106f8e671de11d; LICENSE-DECISION.md, spec/spec.md, spec/requirements/NFR-001-governance-boundary.md, spec/requirements/FR-005-census-native-correspondence-classes.md, spec/decisions/ADR-001-tl-crates-stay-quire-independent-quire-mltl-bridges.md, spec/reviews/base.md, spec/reviews/dependency.md, spec/reviews/scope-boundary.md"
review_set: subset
---

## Summary

Ticket: none found (branch `chore/drop-pgm01-citations` and PR title carry no
ticket id). Reviewed the PR #8 diff against origin/main. No PGM-01 reference of
any form remains anywhere in the repository (`grep -rni pgm` returns nothing).
MRS-001's new claim that `tl-mltl` is fully independent of Quire was checked
against `tl-mltl` origin/main (its `Cargo.toml` no longer names any quire crate;
its FR-019 is `status: superseded`). `quire validate --scope . "spec/**/*.md"`
output is byte-identical between origin/main and the PR head (exit 0, only
pre-existing module-registration warnings). NFR-001-AC-3 keeps its testable core.

Examined: MRS-001 (Purpose, Requirements Architecture, References), NFR-001
(Statement, Scope, Rationale, AC-1..AC-3, Dependencies), FR-005 (population
section, AC-7), ADR-001 (Status, Context, Decision, Consequences,
Alternatives), LICENSE-DECISION.md, and the three edited review records.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | NFR-001-AC-3 ends "that authority remains with the named human release authority", but after the PGM-01-R06/R09 cite was dropped nothing names that authority. The clause cannot be checked by inspection. The testable core (emitted documents assert no approval, consumer validation, or release decision) is intact. Fix: end the AC at "...constitutes a release decision." | spec/requirements/NFR-001-governance-boundary.md:50 |
| FND-002 | low | FR-005 has the same dangling "named human release authority" clause. Fix: "...records no approval or release decision, as NFR-001 requires." | spec/requirements/FR-005-census-native-correspondence-classes.md:117-119 |
| FND-003 | medium | The PR makes MRS-001 say `tl-mltl` "is fully independent of the Quire ecosystem". The References entry in the same file still says tl-mltl FR-018/FR-019 describe the boundary "as it exists today in tl-mltl, before the port", and that TL-180 "retires FR-019". tl-mltl FR-019 is already `status: superseded`, so the file now contradicts itself. The references also carry ticket-status snapshots, which are tracking ceremony: "This ticket (TL-177) does not itself edit either file", "#63 and #64, have both closed; implementation is #70 and #71", "pending merge as of 2026-09-21". Fix: reduce each reference to a plain link that states ownership, and delete the status snapshots. | spec/spec.md:155-181 |
| FND-004 | medium | Tracking ceremony: a commit-SHA provenance pin. "That mistake was caught during TL-176 review and corrected in commit `b8b1f8c` (`agent-ix/quire-mltl`)". Fix: delete the sentence and keep only the decision (AGPL-3.0-or-later, not -only, and why). | spec/decisions/ADR-001-tl-crates-stay-quire-independent-quire-mltl-bridges.md:73-76 |
| FND-005 | medium | Tracking ceremony: contribution provenance. "Tracked under agent-ix/quire-mltl TL-176, part of epic TL-175. Corrected during TL-176's review — caught by an Opus review agent that fetched ... origin/main instead of trusting a local checkout". Fix: delete the paragraph. | LICENSE-DECISION.md:28-31 |
| FND-006 | low | The governing standard was deleted but "Quire-governed" is still used as a classification in NFR-001 Rationale and ADR-001 Consequences. The PR renamed the MRS-001 heading to "Quire-owned", so the terms now disagree. Fix: use "Quire-owned" in both places. | spec/requirements/NFR-001-governance-boundary.md:26-27; spec/decisions/ADR-001-tl-crates-stay-quire-independent-quire-mltl-bridges.md:98 |
| FND-007 | low | NFR-001 Scope still lists "release claims quire-mltl makes". AC-3 says the crate makes none, and no release-decision authority is defined any longer. This is leftover release-decision ceremony. Fix: drop "and release claims quire-mltl makes". | spec/requirements/NFR-001-governance-boundary.md:18-21 |
| FND-008 | low | Rewrite leftovers. "updates its own `FR-026`, and `FR-028`" has a stray comma, which is left over from the deleted list item. Lines 63, 65 and 111 are left unwrapped. Fix: "updates its own `FR-026` and `FR-028`", then rewrap. | spec/decisions/ADR-001-tl-crates-stay-quire-independent-quire-mltl-bridges.md:63,65,89-90,111 |

## Verdict

Changes requested. Criterion (1) passes: no live PGM-01 reference remains.
Criterion (4) passes: validator output is identical to origin/main. On
criterion (2), the rewritten prose is accurate. NFR-001-AC-3 stays testable,
but its trailing clause now points at nothing (FND-001/002). The PR also adds
one internal contradiction to spec.md (FND-003). On criterion (3), the owner
rule has these fixed in this PR: the commit-SHA pin (FND-004), the
contribution-provenance paragraph (FND-005), and the ticket-status snapshots
(FND-003).
