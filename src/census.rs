//! FR-005: the closed, reviewed native-correspondence class registry.
//!
//! This module owns only the registry's *classification data* — which
//! classes exist, how each one is classified (`applicable`, `excluded`, or
//! `blocked`), and the stable digest over the ordered class set. It holds no
//! fixture bytes and calls none of this crate's `derive`/`evaluate`/`read`/
//! `map` paths: fixtures require `agent-ix-baseline-producer` and
//! `quire_observation` admission machinery that only this crate's
//! `[dev-dependencies]` provide, so building and replaying them is the
//! integration test suite's job (`tests/tc_087_native_correspondence_census.rs`),
//! not this library's. See FR-005 and
//! `spec/decisions/ADR-002-quire-mltl-owns-the-native-correspondence-dimension.md`.

use tl_mltl::wire::common::raw_sha256;

/// One `quire-contract-ir` contract this crate's supplier obligation is
/// measured against. Pinned by identity and by the digest of *this crate's
/// own* wire schema for the document that contract reads — the exact "bytes
/// this crate actually exchanged" ADR-002 describes — never by a digest
/// copied out of `quire-contract-ir` itself, which would put another
/// repository's content in this one (FR-005-AC-6).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CounterpartContract {
    /// Exact `quire-contract-ir` contract label, e.g. `"FR-026"`.
    pub label: &'static str,
    /// This crate's own wire contract label for the exchanged document
    /// (`contract_ir::CONTRACT`, `report::CONTRACT`, or `request::CONTRACT`).
    pub exchanged_contract: &'static str,
    /// This crate's own checked-in schema digest for `exchanged_contract`
    /// — a real, locally computed SHA-256 (see `contract_ir::SCHEMA_SHA256`
    /// / `report::SCHEMA_SHA256`), not a value read from `quire-contract-ir`.
    pub exchanged_schema_sha256: &'static str,
    /// The upstream ticket/PR revision, carried as provenance only — never
    /// substituted for the digest above (FR-005's own text: "A ticket state
    /// is not a substitute for the contract revision").
    pub provenance: &'static str,
}

/// `quire-contract-ir` FR-026's accepted native/TL temporal-correspondence
/// contract, at the revision this repository's own spec (ADR-002) records:
/// specified by `quire-contract-ir` #64/PR #68, implemented by #71/PR #78
/// (merged to `quire-contract-ir` `main` as `ad5a418`). FR-026 "joins a
/// `TlMappedResultView`" — the exact document this crate's `contract_ir`
/// module supplies — so the exchanged bytes pinned here are that mapping's
/// own checked-in schema digest.
pub const QUIRE_CONTRACT_IR_FR_026: CounterpartContract = CounterpartContract {
    label: "quire-contract-ir.FR-026",
    exchanged_contract: crate::contract_ir::CONTRACT,
    exchanged_schema_sha256: crate::contract_ir::SCHEMA_SHA256,
    provenance: "quire-contract-ir #71 / PR #78 (merged ad5a418)",
};

/// Reason code shared by every class this crate's own strict request
/// admission (`request::build_wire`'s profile/node-kind/clock-scope checks)
/// already makes unreachable before `report::evaluate` can run. See each
/// citing entry's `reason` text for the exact guard.
const UNREACHABLE_THROUGH_ADMITTED_REQUEST: &str = "unreachable-through-admitted-request";

/// Reason code for the one class `tl_mltl::PositionHistoryDocument::new`
/// itself refuses to construct, one layer below this crate's own boundary.
const UNREACHABLE_THROUGH_TL_MLTL_CONSTRUCTOR: &str = "unreachable-through-tl-mltl-constructor";

/// Why a class is `excluded` from the applicable population.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ExcludedClass {
    /// Stable reason code. Never "not implemented" (FR-005-AC-2).
    pub reason_code: &'static str,
    /// The concrete evidence backing the reason code.
    pub reason: &'static str,
}

/// A named unresolved dependency and its exact admission condition.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BlockedClass {
    /// The unresolved dependency (ticket, PR, or upstream contract).
    pub dependency: &'static str,
    /// The exact condition that would admit this class.
    pub admission_condition: &'static str,
}

/// One applicable class's binding to its canonical fixture.
///
/// This struct carries no bytes: `fixture_id` is the exact name of the
/// `tests/tc_087_native_correspondence_census.rs` function that builds the
/// one canonical fixture for this class and independently records its
/// expected outcome, so `FIXTURE_IDS_COVERED` in that file can assert every
/// applicable class here is actually replayed, and every replayed fixture
/// names a class here (FR-005-AC-2's "exactly one canonical fixture").
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ApplicableClass {
    /// Name of the test-module function producing this class's fixture.
    pub fixture_id: &'static str,
    /// The `quire-contract-ir` counterpart contract(s) this class's mapped
    /// outcome is measured against, if the class reaches a mapped outcome.
    pub counterpart: Option<CounterpartContract>,
}

/// The three ways a registry entry may be classified. Lifecycle state is
/// deliberately outside `ClassKey` (FR-005-AC-3): reclassifying an entry
/// changes this field, never the entry's token.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ClassState {
    /// Carried losslessly, with one canonical fixture and expectation.
    Applicable(ApplicableClass),
    /// Invalid by construction, redundant under another class, or out of
    /// this crate's scope — never "not implemented".
    Excluded(ExcludedClass),
    /// Named unresolved dependency; visible, carries no outcome, excluded
    /// from the applicable denominator.
    Blocked(BlockedClass),
}

/// One row of the reviewed ordered class registry.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RegistryEntry {
    /// Stable UTF-8-sortable class token. Never reused across two shapes,
    /// never re-keyed when `state` changes (FR-005-AC-3).
    pub token: &'static str,
    /// One line naming which catalog dimension(s) this entry's fixture
    /// isolates (the registry is not the catalog's Cartesian product: one
    /// fixture may exercise several dimensions at once).
    pub dimensions: &'static str,
    /// Current classification.
    pub state: ClassState,
}

/// The closed, reviewed native-correspondence class registry, ordered by the
/// UTF-8 bytes of each entry's `token` (`tc_087` pins this order's digest).
///
/// Every class in FR-005's closed catalog table appears in at least one
/// entry below; entries are one-axis-at-a-time isolations of a common
/// baseline (future, closed-trace, event-position clock, full proposition
/// coverage, complete/available, `Completed`+`Satisfied`, original relation,
/// value-true outcome), not the catalog's full Cartesian product, per
/// FR-005's own "reviewed ordered enumeration... not its full Cartesian
/// product."
pub const REGISTRY: &[RegistryEntry] = &[
    RegistryEntry {
        token: "availability/available",
        dimensions: "availability=available (part of the baseline fixture)",
        state: ClassState::Applicable(ApplicableClass {
            fixture_id: "tc_087_baseline_future_closed_trace_satisfied",
            counterpart: Some(QUIRE_CONTRACT_IR_FR_026),
        }),
    },
    RegistryEntry {
        token: "availability/contract-unavailable",
        dimensions: "availability=contract-unavailable -> NonValueKind::Unavailable",
        state: ClassState::Applicable(ApplicableClass {
            fixture_id: "tc_087_availability_contract_unavailable",
            counterpart: Some(QUIRE_CONTRACT_IR_FR_026),
        }),
    },
    RegistryEntry {
        token: "availability/not-yet-available",
        dimensions: "availability=not-yet-observed -> NonValueKind::Unavailable",
        state: ClassState::Applicable(ApplicableClass {
            fixture_id: "tc_087_availability_not_yet",
            counterpart: Some(QUIRE_CONTRACT_IR_FR_026),
        }),
    },
    RegistryEntry {
        token: "availability/producer-unavailable",
        dimensions: "availability=producer-unavailable -> NonValueKind::Unavailable",
        state: ClassState::Applicable(ApplicableClass {
            fixture_id: "tc_087_availability_producer_unavailable",
            counterpart: Some(QUIRE_CONTRACT_IR_FR_026),
        }),
    },
    RegistryEntry {
        token: "clock-family/event-position",
        dimensions: "clock family=event-position (part of the baseline fixture)",
        state: ClassState::Applicable(ApplicableClass {
            fixture_id: "tc_087_baseline_future_closed_trace_satisfied",
            counterpart: Some(QUIRE_CONTRACT_IR_FR_026),
        }),
    },
    RegistryEntry {
        token: "clock-family/fixed-sample",
        dimensions: "clock family=fixed-sample, past lane",
        state: ClassState::Applicable(ApplicableClass {
            fixture_id: "tc_087_past_fixed_sample_agreeing_binding",
            counterpart: Some(QUIRE_CONTRACT_IR_FR_026),
        }),
    },
    RegistryEntry {
        token: "clock-family/timestamped-event",
        dimensions: "clock family=timestamped-event -> read-time ExpectedMismatch(clockFamily)",
        state: ClassState::Applicable(ApplicableClass {
            fixture_id: "tc_087_clock_family_timestamped_event_refused",
            counterpart: None,
        }),
    },
    RegistryEntry {
        token: "clock-history-binding/absent-for-past-request",
        dimensions: "clock/history binding: past history with no clock binding",
        state: ClassState::Excluded(ExcludedClass {
            reason_code: UNREACHABLE_THROUGH_TL_MLTL_CONSTRUCTOR,
            reason: "`tl_mltl::PositionHistoryDocument::new`'s own \
                `validate_without_digest` (tl-mltl src/past/mod.rs) requires \
                `self.clock.as_ref().ok_or(HistoryError::MissingClock)?` before any \
                document is constructed; a `PositionHistoryDocument` with `clock: None` \
                cannot exist as a value at all, so quire-mltl's own past-lane clock-binding \
                check (request.rs::validate_clock_scope) can never observe an admitted \
                history with an absent binding to refuse -- this class is invalid by \
                construction one layer below this crate's own boundary, proven directly by \
                `tc_087_absent_clock_binding_is_refused_by_tl_mltl_itself`",
        }),
    },
    RegistryEntry {
        token: "clock-history-binding/agreeing-event-position",
        dimensions: "clock/history binding: agreeing EventPosition pair (part of the past baseline)",
        state: ClassState::Applicable(ApplicableClass {
            fixture_id: "tc_087_past_baseline_satisfied",
            counterpart: Some(QUIRE_CONTRACT_IR_FR_026),
        }),
    },
    RegistryEntry {
        token: "clock-history-binding/agreeing-fixed-sample",
        dimensions: "clock/history binding: agreeing FixedSample pair",
        state: ClassState::Applicable(ApplicableClass {
            fixture_id: "tc_087_past_fixed_sample_agreeing_binding",
            counterpart: Some(QUIRE_CONTRACT_IR_FR_026),
        }),
    },
    RegistryEntry {
        token: "clock-history-binding/disagreeing",
        dimensions:
            "clock/history binding: EventPosition clock over a FixedSample-bound history -> ExpectedMismatch(clockBinding)",
        state: ClassState::Applicable(ApplicableClass {
            fixture_id: "tc_087_clock_history_binding_disagreeing",
            counterpart: None,
        }),
    },
    RegistryEntry {
        token: "completeness/complete",
        dimensions: "completeness=complete (part of the baseline fixture)",
        state: ClassState::Applicable(ApplicableClass {
            fixture_id: "tc_087_baseline_future_closed_trace_satisfied",
            counterpart: Some(QUIRE_CONTRACT_IR_FR_026),
        }),
    },
    RegistryEntry {
        token: "completeness/contradicted",
        dimensions: "completeness=contradicted -> NonValueKind::Contradicted",
        state: ClassState::Applicable(ApplicableClass {
            fixture_id: "tc_087_completeness_contradicted",
            counterpart: Some(QUIRE_CONTRACT_IR_FR_026),
        }),
    },
    RegistryEntry {
        token: "completeness/incomplete",
        dimensions: "completeness=incomplete -> NonValueKind::Incomplete",
        state: ClassState::Applicable(ApplicableClass {
            fixture_id: "tc_087_completeness_incomplete",
            counterpart: Some(QUIRE_CONTRACT_IR_FR_026),
        }),
    },
    RegistryEntry {
        token: "execution-truth/completed-pending",
        dimensions: "execution=Completed, truth=Pending (open-prefix future lane)",
        state: ClassState::Applicable(ApplicableClass {
            fixture_id: "tc_087_execution_truth_pending",
            counterpart: Some(QUIRE_CONTRACT_IR_FR_026),
        }),
    },
    RegistryEntry {
        token: "execution-truth/completed-satisfied",
        dimensions: "execution=Completed, truth=Satisfied (part of the baseline fixture)",
        state: ClassState::Applicable(ApplicableClass {
            fixture_id: "tc_087_baseline_future_closed_trace_satisfied",
            counterpart: Some(QUIRE_CONTRACT_IR_FR_026),
        }),
    },
    RegistryEntry {
        token: "execution-truth/completed-violated",
        dimensions: "execution=Completed, truth=Violated",
        state: ClassState::Applicable(ApplicableClass {
            fixture_id: "tc_087_execution_truth_violated",
            counterpart: Some(QUIRE_CONTRACT_IR_FR_026),
        }),
    },
    RegistryEntry {
        token: "execution-truth/failed",
        dimensions: "execution=Failed",
        state: ClassState::Excluded(ExcludedClass {
            reason_code: UNREACHABLE_THROUGH_ADMITTED_REQUEST,
            reason: "every EvaluationError/PastEvaluationError arm report.rs::classify_future/\
                classify_past maps to AssessmentExecution::Failed (TimeOverflow, Horizon, \
                InvalidNodeReference, PositionArithmeticOverflow, HistoryPositionAbsent, Result) \
                requires a malformed formula/trace/history combination that request::build_wire's \
                own strict admission (node-reference bounds via formula_graph_depth, profile/node-kind \
                matching, clock-scope agreement) already refuses at read time before evaluation can run; \
                a request this crate ever hands to report::evaluate cannot reach these arms",
        }),
    },
    RegistryEntry {
        token: "execution-truth/refused",
        dimensions: "execution=Refused",
        state: ClassState::Excluded(ExcludedClass {
            reason_code: UNREACHABLE_THROUGH_ADMITTED_REQUEST,
            reason: "AssessmentExecution::Refused requires EvaluationError::TraceNotStrictlyOrdered \
                (tl-mltl future/evaluate.rs::validate_trace) or a lane/input mismatch \
                (report.rs::execute's None arms); trace::validate already refuses an out-of-order \
                trace at admission (the identical `pair[0] >= pair[1]` check, tl-mltl wire/trace.rs), \
                and request::build_wire's lane_matches_input invariant guarantees the lane always \
                matches the admitted input, so neither precondition survives request admission",
        }),
    },
    RegistryEntry {
        token: "execution-truth/resource-incomplete",
        dimensions: "execution=ResourceIncomplete (also the resource-boundary one-over-ceiling class)",
        state: ClassState::Applicable(ApplicableClass {
            fixture_id: "tc_087_resource_boundary_one_over_ceiling",
            counterpart: Some(QUIRE_CONTRACT_IR_FR_026),
        }),
    },
    RegistryEntry {
        token: "execution-truth/unsupported",
        dimensions: "execution=Unsupported",
        state: ClassState::Excluded(ExcludedClass {
            reason_code: UNREACHABLE_THROUGH_ADMITTED_REQUEST,
            reason: "AssessmentExecution::Unsupported requires EvaluationError::UnsupportedProfile/\
                UnsupportedPastNode or PastEvaluationError::OwnerStatePreserved{Unsupported}/\
                FutureNodeUnsupported; request::build_wire already matches the evaluator dispatch to \
                the formula's own profile exactly and refuses any past-lane formula containing a \
                Future-family node before evaluation, and this crate never constructs the \
                PositionHistorySource::NonValue producer-side placeholder OwnerStatePreserved reads, \
                so no admitted request can reach either arm",
        }),
    },
    RegistryEntry {
        token: "lane/future-closed-trace",
        dimensions: "lane=future, semantic profile=ClosedTraceV1 (part of the baseline fixture)",
        state: ClassState::Applicable(ApplicableClass {
            fixture_id: "tc_087_baseline_future_closed_trace_satisfied",
            counterpart: Some(QUIRE_CONTRACT_IR_FR_026),
        }),
    },
    RegistryEntry {
        token: "lane/future-online-prefix",
        dimensions: "lane=future, semantic profile=OnlinePrefixV1",
        state: ClassState::Applicable(ApplicableClass {
            fixture_id: "tc_087_execution_truth_pending",
            counterpart: Some(QUIRE_CONTRACT_IR_FR_026),
        }),
    },
    RegistryEntry {
        token: "lane/past-origin-complete-history",
        dimensions: "lane=past, semantic profile=OriginCompleteHistoryV1",
        state: ClassState::Applicable(ApplicableClass {
            fixture_id: "tc_087_past_baseline_satisfied",
            counterpart: Some(QUIRE_CONTRACT_IR_FR_026),
        }),
    },
    RegistryEntry {
        token: "mapped-outcome/non-value-contradicted",
        dimensions: "mapped outcome=NonValue(Contradicted)",
        state: ClassState::Applicable(ApplicableClass {
            fixture_id: "tc_087_completeness_contradicted",
            counterpart: Some(QUIRE_CONTRACT_IR_FR_026),
        }),
    },
    RegistryEntry {
        token: "mapped-outcome/non-value-failed",
        dimensions: "mapped outcome=NonValue(Failed)",
        state: ClassState::Excluded(ExcludedClass {
            reason_code: UNREACHABLE_THROUGH_ADMITTED_REQUEST,
            reason: "NonValueKind::Failed is contract_ir::outcome's image of \
                AssessmentExecution::Failed only; see execution-truth/failed for why that execution \
                state is unreachable through an admitted request",
        }),
    },
    RegistryEntry {
        token: "mapped-outcome/non-value-incomplete",
        dimensions: "mapped outcome=NonValue(Incomplete)",
        state: ClassState::Applicable(ApplicableClass {
            fixture_id: "tc_087_completeness_incomplete",
            counterpart: Some(QUIRE_CONTRACT_IR_FR_026),
        }),
    },
    RegistryEntry {
        token: "mapped-outcome/non-value-pending",
        dimensions: "mapped outcome=NonValue(Pending)",
        state: ClassState::Applicable(ApplicableClass {
            fixture_id: "tc_087_execution_truth_pending",
            counterpart: Some(QUIRE_CONTRACT_IR_FR_026),
        }),
    },
    RegistryEntry {
        token: "mapped-outcome/non-value-refused",
        dimensions: "mapped outcome=NonValue(Refused)",
        state: ClassState::Excluded(ExcludedClass {
            reason_code: UNREACHABLE_THROUGH_ADMITTED_REQUEST,
            reason: "NonValueKind::Refused is contract_ir::outcome's image of \
                AssessmentExecution::Refused only; see execution-truth/refused for why that execution \
                state is unreachable through an admitted request",
        }),
    },
    RegistryEntry {
        token: "mapped-outcome/non-value-resource-incomplete",
        dimensions: "mapped outcome=NonValue(ResourceIncomplete)",
        state: ClassState::Applicable(ApplicableClass {
            fixture_id: "tc_087_resource_boundary_one_over_ceiling",
            counterpart: Some(QUIRE_CONTRACT_IR_FR_026),
        }),
    },
    RegistryEntry {
        token: "mapped-outcome/non-value-unavailable",
        dimensions: "mapped outcome=NonValue(Unavailable)",
        state: ClassState::Applicable(ApplicableClass {
            fixture_id: "tc_087_availability_not_yet",
            counterpart: Some(QUIRE_CONTRACT_IR_FR_026),
        }),
    },
    RegistryEntry {
        token: "mapped-outcome/non-value-unsupported",
        dimensions: "mapped outcome=NonValue(Unsupported)",
        state: ClassState::Excluded(ExcludedClass {
            reason_code: UNREACHABLE_THROUGH_ADMITTED_REQUEST,
            reason: "NonValueKind::Unsupported is contract_ir::outcome's image of \
                AssessmentExecution::Unsupported only; see execution-truth/unsupported for why that \
                execution state is unreachable through an admitted request",
        }),
    },
    RegistryEntry {
        token: "mapped-outcome/value-false",
        dimensions: "mapped outcome=Value(false)",
        state: ClassState::Applicable(ApplicableClass {
            fixture_id: "tc_087_execution_truth_violated",
            counterpart: Some(QUIRE_CONTRACT_IR_FR_026),
        }),
    },
    RegistryEntry {
        token: "mapped-outcome/value-true",
        dimensions: "mapped outcome=Value(true) (part of the baseline fixture)",
        state: ClassState::Applicable(ApplicableClass {
            fixture_id: "tc_087_baseline_future_closed_trace_satisfied",
            counterpart: Some(QUIRE_CONTRACT_IR_FR_026),
        }),
    },
    RegistryEntry {
        token: "proposition-coverage/all-present",
        dimensions: "every referenced proposition present (part of the baseline fixture)",
        state: ClassState::Applicable(ApplicableClass {
            fixture_id: "tc_087_baseline_future_closed_trace_satisfied",
            counterpart: Some(QUIRE_CONTRACT_IR_FR_026),
        }),
    },
    RegistryEntry {
        token: "proposition-coverage/referenced-absent",
        dimensions: "a referenced proposition absent from the map -> ExpectedMismatch(propositionMap)",
        state: ClassState::Applicable(ApplicableClass {
            fixture_id: "tc_087_proposition_coverage_absent",
            counterpart: None,
        }),
    },
    RegistryEntry {
        token: "resource-boundary/at-ceiling",
        dimensions: "resource boundary: exactly at a declared ceiling (max_formula_nodes=2 admits)",
        state: ClassState::Applicable(ApplicableClass {
            fixture_id: "tc_087_resource_boundary_at_ceiling",
            counterpart: Some(QUIRE_CONTRACT_IR_FR_026),
        }),
    },
    RegistryEntry {
        token: "resource-boundary/one-over-ceiling",
        dimensions: "resource boundary: one over a declared ceiling -> ResourceIncomplete",
        state: ClassState::Applicable(ApplicableClass {
            fixture_id: "tc_087_resource_boundary_one_over_ceiling",
            counterpart: Some(QUIRE_CONTRACT_IR_FR_026),
        }),
    },
    RegistryEntry {
        token: "result-relation/invalidating",
        dimensions: "result relation=invalidating",
        state: ClassState::Applicable(ApplicableClass {
            fixture_id: "tc_087_result_relation_invalidating",
            counterpart: Some(QUIRE_CONTRACT_IR_FR_026),
        }),
    },
    RegistryEntry {
        token: "result-relation/original",
        dimensions: "result relation=original (part of the baseline fixture)",
        state: ClassState::Applicable(ApplicableClass {
            fixture_id: "tc_087_baseline_future_closed_trace_satisfied",
            counterpart: Some(QUIRE_CONTRACT_IR_FR_026),
        }),
    },
    RegistryEntry {
        token: "result-relation/superseding",
        dimensions: "result relation=superseding",
        state: ClassState::Applicable(ApplicableClass {
            fixture_id: "tc_087_result_relation_superseding",
            counterpart: Some(QUIRE_CONTRACT_IR_FR_026),
        }),
    },
];

/// Population, exclusions, and blocked set — never a bare ratio
/// (FR-005-AC-7).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CensusReport {
    /// Total reviewed registry entries.
    pub total: usize,
    /// Applicable class tokens, sorted.
    pub applicable: Vec<&'static str>,
    /// Excluded class tokens paired with their reason code, sorted by token.
    pub excluded: Vec<(&'static str, &'static str)>,
    /// Blocked class tokens paired with their unresolved dependency, sorted
    /// by token.
    pub blocked: Vec<(&'static str, &'static str)>,
    /// The exact `tl-mltl`/`tl-syntax`/`quire-observation` revisions this
    /// census was measured at (this crate's own `Cargo.toml` pins).
    pub measured_revisions: MeasuredRevisions,
}

/// The exact dependency revisions a census figure was measured at.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MeasuredRevisions {
    /// `tl-mltl` git revision this crate's `Cargo.toml` pins.
    pub tl_mltl: &'static str,
    /// `tl-syntax` git revision this crate's `Cargo.toml` pins.
    pub tl_syntax: &'static str,
    /// `quire-observation` git revision this crate's `Cargo.toml` pins.
    pub quire_observation: &'static str,
}

/// The exact revisions this crate's `Cargo.toml`/`Cargo.lock` pin. Shared by
/// every census figure so no reported population is measured at an
/// ambiguous or drifted dependency set.
pub const MEASURED_REVISIONS: MeasuredRevisions = MeasuredRevisions {
    tl_mltl: tl_mltl::TL_MLTL_SOURCE_REVISION,
    tl_syntax: tl_mltl::TL_SYNTAX_REVISION,
    quire_observation: crate::QUIRE_OBSERVATION_REVISION,
};

/// Computes the population/exclusions/blocked-set census over [`REGISTRY`].
#[must_use]
pub fn census() -> CensusReport {
    let mut applicable = Vec::new();
    let mut excluded = Vec::new();
    let mut blocked = Vec::new();
    for entry in REGISTRY {
        match entry.state {
            ClassState::Applicable(_) => applicable.push(entry.token),
            ClassState::Excluded(class) => excluded.push((entry.token, class.reason_code)),
            ClassState::Blocked(class) => blocked.push((entry.token, class.dependency)),
        }
    }
    CensusReport {
        total: REGISTRY.len(),
        applicable,
        excluded,
        blocked,
        measured_revisions: MEASURED_REVISIONS,
    }
}

/// The ordered-set digest FR-005-AC-3 pins: a SHA-256 over the registry's
/// tokens in their declared order, newline-joined. Any insertion, deletion,
/// reordering, or token rename changes this digest, which is exactly the
/// "successor registry identity" FR-005 requires for such a change.
#[must_use]
pub fn ordered_set_digest() -> String {
    let joined = REGISTRY
        .iter()
        .map(|entry| entry.token)
        .collect::<Vec<_>>()
        .join("\n");
    raw_sha256(joined.as_bytes())
}

/// Returns every applicable entry's `fixture_id`s, sorted and deduplicated.
/// One fixture may serve several classes, so this is not `REGISTRY.len()`.
#[must_use]
pub fn applicable_fixture_ids() -> Vec<&'static str> {
    let mut ids: Vec<&'static str> = REGISTRY
        .iter()
        .filter_map(|entry| match entry.state {
            ClassState::Applicable(class) => Some(class.fixture_id),
            ClassState::Excluded(_) | ClassState::Blocked(_) => None,
        })
        .collect();
    ids.sort_unstable();
    ids.dedup();
    ids
}

#[cfg(test)]
mod tests {
    use super::{census, ordered_set_digest, ClassState, REGISTRY};

    // Trace: TC-087, FR-005-AC-1
    #[test]
    fn registry_is_sorted_by_token_and_every_token_is_unique() {
        let tokens: Vec<&str> = REGISTRY.iter().map(|entry| entry.token).collect();
        let mut sorted = tokens.clone();
        sorted.sort_unstable();
        assert_eq!(tokens, sorted, "REGISTRY must be declared in token order");
        let mut deduped = tokens.clone();
        deduped.dedup();
        assert_eq!(
            tokens.len(),
            deduped.len(),
            "every registry token must be unique"
        );
    }

    // Trace: TC-087, FR-005-AC-3
    #[test]
    fn ordered_set_digest_is_stable_and_pinned() {
        // Pinned at this registry revision. Inserting, deleting, reordering,
        // or renaming a token changes this value -- exactly the "successor
        // registry identity" FR-005-AC-3 requires -- and this assertion is
        // the acceptance test that change must update.
        const PINNED: &str = "2f4d8fd0b6fd6d8a69265e920f89fb9862f6dbca905f284304ba21fd192e3283";
        let first = ordered_set_digest();
        let second = ordered_set_digest();
        assert_eq!(
            first, second,
            "the digest must be a pure function of REGISTRY"
        );
        assert_eq!(
            first.len(),
            64,
            "raw_sha256 always emits 64 lowercase hex chars"
        );
        assert_eq!(
            first, PINNED,
            "REGISTRY's ordered token set changed; if this change was reviewed and \
             intentional, update PINNED to the new digest printed here"
        );
    }

    // Trace: TC-087, FR-005-AC-3
    #[test]
    fn ordered_set_digest_is_sensitive_to_order_and_content() {
        // `ordered_set_digest` is exactly `raw_sha256` over the newline-joined
        // token list; this proves that function actually depends on both the
        // set of tokens and their order, independent of the real REGISTRY, so
        // the pin above is not vacuously stable.
        let digest_of = |tokens: &[&str]| super::raw_sha256(tokens.join("\n").as_bytes());
        let original = digest_of(&["a/one", "b/two", "c/three"]);
        let reordered = digest_of(&["b/two", "a/one", "c/three"]);
        let renamed = digest_of(&["a/one", "b/two", "c/three-renamed"]);
        let inserted = digest_of(&["a/one", "b/two", "b/two-and-a-half", "c/three"]);
        assert_ne!(original, reordered, "reordering must change the digest");
        assert_ne!(original, renamed, "renaming a token must change the digest");
        assert_ne!(
            original, inserted,
            "inserting a token must change the digest"
        );
    }

    // Trace: TC-087, FR-005-AC-1, FR-005-AC-2
    #[test]
    fn census_population_partitions_the_registry_and_never_says_not_implemented() {
        let report = census();
        assert_eq!(report.total, REGISTRY.len());
        assert_eq!(
            report.applicable.len() + report.excluded.len() + report.blocked.len(),
            report.total,
            "every entry is classified exactly once"
        );
        for (_, reason_code) in &report.excluded {
            let lower = reason_code.to_lowercase();
            assert!(
                !lower.contains("not implemented") && !lower.contains("not-implemented-yet"),
                "\"not implemented\" is not a valid exclusion reason (FR-005-AC-2): {reason_code}"
            );
        }
        for entry in REGISTRY {
            if let ClassState::Excluded(class) = entry.state {
                let lower = class.reason.to_lowercase();
                assert!(
                    !lower.contains("not implemented"),
                    "exclusion reason text must cite concrete evidence, not \"not implemented\": {}",
                    entry.token
                );
            }
        }
    }

    // Trace: TC-087, FR-005-AC-2
    #[test]
    fn every_blocked_class_names_a_dependency_and_admission_condition() {
        let report = census();
        // As of this registry revision, no native-correspondence class is
        // blocked on an unresolved dependency (every FR-025/FR-026
        // counterpart this census needs is already an accepted, merged
        // contract revision -- see `QUIRE_CONTRACT_IR_FR_026`'s doc comment).
        // That is an honest reviewed finding, not an oversight, so this test
        // pins the count explicitly rather than looping over zero entries:
        // it fails the moment a `Blocked` entry is added, forcing this
        // assertion itself to be updated alongside it, and the loop below
        // then exercises the real per-entry invariant instead of running
        // zero times.
        assert_eq!(report.blocked.len(), 0);
        for entry in REGISTRY {
            if let ClassState::Blocked(class) = entry.state {
                assert!(!class.dependency.is_empty());
                assert!(!class.admission_condition.is_empty());
            } else {
                continue;
            }
            unreachable!("no REGISTRY entry is Blocked at this revision; see the count above");
        }
    }
}
