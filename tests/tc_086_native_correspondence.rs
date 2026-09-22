//! TC-086: FR-004, native-correspondence identity preservation.
//!
//! Exercises `subject_identity`/`correspondence_identity` admission,
//! content-identity preimage membership, verbatim propagation into the
//! result and Contract-IR mapping, correction-chain identity stability,
//! read-time and evaluation-time refusal, and the three contracts' shape
//! and no-`quire-contract-ir`-dependency stability (FR-004-AC-1..8). Reuses
//! `tc_084_temporal_owner_wire.rs`'s owner-admission fixtures via
//! `tests/support`.

mod support;

use std::collections::BTreeSet;

use quire_mltl::contract_ir::{self, MappedOutcome, MappingSelection, NonValueKind};
use quire_mltl::{report, request};
use serde_json::Value;
use support::{
    admit_history, admit_request, future_formula, observations, owner_views,
    owner_views_with_clock, past_formula, proposition_map, trace_document, FixtureClock,
};
use tl_mltl::wire::{trace, OwnerLimits, OwnerReadErrorCode};
use tl_syntax::SemanticProfile;

fn base_input<'a>(
    formula: &'a tl_syntax::FormulaDocument,
    propositions: &'a tl_syntax::PropositionMapDocument,
    trace: &'a trace::ValidatedTrace,
    decision: &'a support::OwnerViews,
    surrounding: &'a support::OwnerViews,
    subject_identity: &'a str,
    correspondence_identity: &'a str,
) -> request::RequestInput<'a> {
    request::RequestInput {
        formula,
        proposition_map: propositions,
        input: request::TemporalInput::Future(trace),
        clock: &decision.clock,
        subject_identity,
        correspondence_identity,
        anchor: 0,
        observations: observations(
            decision,
            surrounding,
            true,
            true,
            true,
            true,
            &decision.completeness_complete,
            &decision.availability_available,
        ),
    }
}

// Trace: TC-086, FR-004-AC-1
#[test]
fn tc_086_absent_empty_or_oversized_identity_is_refused_with_no_partial_document() {
    let decision = owner_views("decision-ac1", 2);
    let surrounding = owner_views("surrounding-ac1", 2);
    let formula = future_formula(SemanticProfile::ClosedTraceV1);
    let propositions = proposition_map();
    let trace =
        trace::ValidatedTrace::admit(&trace_document("trace:ac1", true), OwnerLimits::default())
            .unwrap();

    // Empty subject identity.
    let empty_subject = base_input(
        &formula,
        &propositions,
        &trace,
        &decision,
        &surrounding,
        "",
        "native-tl-correspondence:ac1",
    );
    let error = request::derive(empty_subject, OwnerLimits::default()).unwrap_err();
    assert_eq!(error.code(), OwnerReadErrorCode::ExpectedMismatch);
    assert_eq!(error.field(), "subjectIdentity");

    // Empty correspondence identity.
    let empty_correspondence = base_input(
        &formula,
        &propositions,
        &trace,
        &decision,
        &surrounding,
        "native-subject:ac1",
        "",
    );
    let error = request::derive(empty_correspondence, OwnerLimits::default()).unwrap_err();
    assert_eq!(error.code(), OwnerReadErrorCode::ExpectedMismatch);
    assert_eq!(error.field(), "correspondenceIdentity");

    // Oversized (> 256 bytes) subject identity is equally malformed, not a
    // resource ceiling this crate silently truncates.
    let oversized = "x".repeat(257);
    let too_long = base_input(
        &formula,
        &propositions,
        &trace,
        &decision,
        &surrounding,
        &oversized,
        "native-tl-correspondence:ac1",
    );
    let error = request::derive(too_long, OwnerLimits::default()).unwrap_err();
    assert_eq!(error.code(), OwnerReadErrorCode::ExpectedMismatch);
    assert_eq!(error.field(), "subjectIdentity");

    // A well-formed pair is admitted, and carried verbatim -- never derived,
    // defaulted, or substituted from the formula, clock, or any owner view.
    let valid = base_input(
        &formula,
        &propositions,
        &trace,
        &decision,
        &surrounding,
        "native-subject:ac1-exact",
        "native-tl-correspondence:ac1-exact",
    );
    let request = admit_request(valid, OwnerLimits::default());
    assert_eq!(request.subject_identity(), "native-subject:ac1-exact");
    assert_eq!(
        request.correspondence_identity(),
        "native-tl-correspondence:ac1-exact"
    );
}

// Trace: TC-086, FR-004-AC-2
#[test]
fn tc_086_correspondence_identity_is_inside_the_content_identity_preimage() {
    let decision = owner_views("decision-ac2", 2);
    let surrounding = owner_views("surrounding-ac2", 2);
    let formula = future_formula(SemanticProfile::ClosedTraceV1);
    let propositions = proposition_map();
    let trace =
        trace::ValidatedTrace::admit(&trace_document("trace:ac2", true), OwnerLimits::default())
            .unwrap();

    let first_input = base_input(
        &formula,
        &propositions,
        &trace,
        &decision,
        &surrounding,
        "native-subject:ac2",
        "native-tl-correspondence:ac2-a",
    );
    let second_input = base_input(
        &formula,
        &propositions,
        &trace,
        &decision,
        &surrounding,
        "native-subject:ac2",
        "native-tl-correspondence:ac2-b",
    );

    let first_request = admit_request(first_input, OwnerLimits::default());
    let second_request = admit_request(second_input, OwnerLimits::default());
    assert_ne!(first_request.identity(), second_request.identity());

    let first_result = report::read(
        report::evaluate(
            &first_request,
            report::ResultRelationInput::Original,
            OwnerLimits::default(),
        )
        .unwrap()
        .bytes(),
        &first_request,
        report::ResultRelationInput::Original,
        OwnerLimits::default(),
    )
    .unwrap();
    let second_result = report::read(
        report::evaluate(
            &second_request,
            report::ResultRelationInput::Original,
            OwnerLimits::default(),
        )
        .unwrap()
        .bytes(),
        &second_request,
        report::ResultRelationInput::Original,
        OwnerLimits::default(),
    )
    .unwrap();
    assert_ne!(first_result.identity(), second_result.identity());

    let first_selection = MappingSelection::for_result(&first_result);
    let second_selection = MappingSelection::for_result(&second_result);
    let first_mapping =
        contract_ir::map(&first_result, &first_selection, OwnerLimits::default()).unwrap();
    let second_mapping =
        contract_ir::map(&second_result, &second_selection, OwnerLimits::default()).unwrap();
    assert_ne!(first_mapping.identity(), second_mapping.identity());
}

// Trace: TC-086, FR-004-AC-3
#[test]
fn tc_086_result_and_mapping_carry_identities_verbatim_and_refuse_disagreement() {
    let decision = owner_views("decision-ac3", 2);
    let surrounding = owner_views("surrounding-ac3", 2);
    let formula = future_formula(SemanticProfile::ClosedTraceV1);
    let propositions = proposition_map();
    let trace =
        trace::ValidatedTrace::admit(&trace_document("trace:ac3", true), OwnerLimits::default())
            .unwrap();
    let input = base_input(
        &formula,
        &propositions,
        &trace,
        &decision,
        &surrounding,
        "native-subject:ac3",
        "native-tl-correspondence:ac3",
    );
    let request = admit_request(input, OwnerLimits::default());
    let result_document = report::evaluate(
        &request,
        report::ResultRelationInput::Original,
        OwnerLimits::default(),
    )
    .unwrap();
    let result = report::read(
        result_document.bytes(),
        &request,
        report::ResultRelationInput::Original,
        OwnerLimits::default(),
    )
    .unwrap();
    assert_eq!(result.subject_identity(), "native-subject:ac3");
    assert_eq!(
        result.correspondence_identity(),
        "native-tl-correspondence:ac3"
    );

    let selection = MappingSelection::for_result(&result);
    let mapping_document = contract_ir::map(&result, &selection, OwnerLimits::default()).unwrap();
    let mapping = contract_ir::read(
        mapping_document.bytes(),
        &result,
        &selection,
        OwnerLimits::default(),
    )
    .unwrap();
    assert_eq!(mapping.subject_identity(), "native-subject:ac3");
    assert_eq!(
        mapping.correspondence_identity(),
        "native-tl-correspondence:ac3"
    );

    // A second, independently self-consistent result -- built for a
    // different correspondence identity -- is a perfectly valid document on
    // its own (its own embedded identity matches its own recomputed hash).
    // Handed to `report::read` alongside the *first* request, its embedded
    // subject/correspondence identity disagrees with what that caller
    // expects, and it is refused rather than silently accepted or repaired.
    let other_input = base_input(
        &formula,
        &propositions,
        &trace,
        &decision,
        &surrounding,
        "native-subject:ac3",
        "native-tl-correspondence:ac3-other",
    );
    let other_request = admit_request(other_input, OwnerLimits::default());
    let other_result = report::read(
        report::evaluate(
            &other_request,
            report::ResultRelationInput::Original,
            OwnerLimits::default(),
        )
        .unwrap()
        .bytes(),
        &other_request,
        report::ResultRelationInput::Original,
        OwnerLimits::default(),
    )
    .unwrap();
    let error = report::read(
        other_result.bytes(),
        &request,
        report::ResultRelationInput::Original,
        OwnerLimits::default(),
    )
    .unwrap_err();
    assert_eq!(error.code(), OwnerReadErrorCode::ExpectedMismatch);

    // The same holds for the mapping: `other_result`'s own mapping is a
    // valid document naming `other_result`'s own identity, so reading it
    // back against the *first* result's selection is refused rather than
    // corrected (mirrors `tc_084_request_result_map_and_lineage_reject_substitution`'s
    // `wrong_selection` check).
    let other_selection = MappingSelection::for_result(&other_result);
    let other_mapping =
        contract_ir::map(&other_result, &other_selection, OwnerLimits::default()).unwrap();
    let error = contract_ir::read(
        other_mapping.bytes(),
        &result,
        &selection,
        OwnerLimits::default(),
    )
    .unwrap_err();
    assert_eq!(error.code(), OwnerReadErrorCode::ExpectedMismatch);
}

// Trace: TC-086, FR-004-AC-4
#[test]
fn tc_086_correction_chain_cannot_repoint_either_identity() {
    let decision = owner_views("decision-ac4", 2);
    let surrounding = owner_views("surrounding-ac4", 2);
    let formula = future_formula(SemanticProfile::ClosedTraceV1);
    let propositions = proposition_map();
    let first_trace = trace::ValidatedTrace::admit(
        &trace_document("trace:ac4-first", true),
        OwnerLimits::default(),
    )
    .unwrap();
    let first_input = base_input(
        &formula,
        &propositions,
        &first_trace,
        &decision,
        &surrounding,
        "native-subject:ac4",
        "native-tl-correspondence:ac4",
    );
    let first_request = admit_request(first_input, OwnerLimits::default());
    let first_result = report::read(
        report::evaluate(
            &first_request,
            report::ResultRelationInput::Original,
            OwnerLimits::default(),
        )
        .unwrap()
        .bytes(),
        &first_request,
        report::ResultRelationInput::Original,
        OwnerLimits::default(),
    )
    .unwrap();

    let second_trace = trace::ValidatedTrace::admit(
        &trace_document("trace:ac4-second", true),
        OwnerLimits::default(),
    )
    .unwrap();
    // Same subject, but a different correspondence -- a correction may
    // never repoint the correspondence it corrects.
    let repointing_input = base_input(
        &formula,
        &propositions,
        &second_trace,
        &decision,
        &surrounding,
        "native-subject:ac4",
        "native-tl-correspondence:ac4-repointed",
    );
    let repointing_request = admit_request(repointing_input, OwnerLimits::default());
    let error = report::evaluate(
        &repointing_request,
        report::ResultRelationInput::Superseding(&first_result),
        OwnerLimits::default(),
    )
    .unwrap_err();
    assert_eq!(error.code(), OwnerReadErrorCode::ExpectedMismatch);

    // A same-identity correction is admitted, and preserves both identities
    // unchanged at the new revision.
    let same_identity_input = base_input(
        &formula,
        &propositions,
        &second_trace,
        &decision,
        &surrounding,
        "native-subject:ac4",
        "native-tl-correspondence:ac4",
    );
    let same_identity_request = admit_request(same_identity_input, OwnerLimits::default());
    let corrected = report::read(
        report::evaluate(
            &same_identity_request,
            report::ResultRelationInput::Superseding(&first_result),
            OwnerLimits::default(),
        )
        .unwrap()
        .bytes(),
        &same_identity_request,
        report::ResultRelationInput::Superseding(&first_result),
        OwnerLimits::default(),
    )
    .unwrap();
    assert_eq!(corrected.subject_identity(), "native-subject:ac4");
    assert_eq!(
        corrected.correspondence_identity(),
        "native-tl-correspondence:ac4"
    );
    assert_eq!(corrected.revision(), 2);
}

// Trace: TC-086, FR-004-AC-5
#[test]
fn tc_086_read_time_refusals_never_produce_a_partial_document() {
    // Unsupported clock family: a TimestampedEvent clock is refused during
    // `derive`/`read`, before any request document exists.
    let decision = owner_views_with_clock("decision-ac5-family", 2, FixtureClock::TimestampedEvent);
    let surrounding =
        owner_views_with_clock("surrounding-ac5-family", 2, FixtureClock::TimestampedEvent);
    let formula = future_formula(SemanticProfile::ClosedTraceV1);
    let propositions = proposition_map();
    let trace = trace::ValidatedTrace::admit(
        &trace_document("trace:ac5-family", true),
        OwnerLimits::default(),
    )
    .unwrap();
    let input = base_input(
        &formula,
        &propositions,
        &trace,
        &decision,
        &surrounding,
        "native-subject:ac5-family",
        "native-tl-correspondence:ac5-family",
    );
    let error = request::derive(input, OwnerLimits::default()).unwrap_err();
    assert_eq!(error.code(), OwnerReadErrorCode::ExpectedMismatch);
    assert_eq!(error.field(), "clockFamily");

    // Disagreeing past-lane clock binding: an EventPosition clock over a
    // FixedSample-bound history.
    let past_decision = owner_views("decision-ac5-binding", 2);
    let past_surrounding = owner_views("surrounding-ac5-binding", 2);
    let past_formula = past_formula();
    let mismatched_history = support::fixed_history_document("history:ac5-binding-fixed");
    let history = admit_history(&mismatched_history);
    let past_input = request::RequestInput {
        formula: &past_formula,
        proposition_map: &propositions,
        input: request::TemporalInput::Past(&history),
        clock: &past_decision.clock,
        subject_identity: "native-subject:ac5-binding",
        correspondence_identity: "native-tl-correspondence:ac5-binding",
        anchor: 1,
        observations: observations(
            &past_decision,
            &past_surrounding,
            true,
            true,
            true,
            true,
            &past_decision.completeness_complete,
            &past_decision.availability_available,
        ),
    };
    let error = request::derive(past_input, OwnerLimits::default()).unwrap_err();
    assert_eq!(error.code(), OwnerReadErrorCode::ExpectedMismatch);
    assert_eq!(error.field(), "clockBinding");

    // An absent binding on a past request cannot reach quire-mltl's own
    // `clockBinding` check at all: `tl_mltl::PositionHistoryDocument::new`
    // refuses to construct a history with no clock binding one layer below
    // this crate's boundary (`HistoryError::MissingClock`). FR-005's census
    // classifies this catalog class `excluded` (invalid by construction)
    // rather than `applicable`; `tc_087_absent_clock_binding_is_refused_by_tl_mltl_itself`
    // proves this directly against `tl-mltl`'s own constructor.
    assert!(tl_mltl::PositionHistoryDocument::new(
        "history:ac5-absent",
        1,
        0,
        1,
        None,
        vec![tl_mltl::PositionObservation::new(0, vec![], None)],
    )
    .is_err());

    // An over-ceiling resource case discovered at read time is a typed
    // `OwnerReadError` (`ResourceIncomplete`), never a `NonValueKind`: there
    // is no result or mapping document to carry one.
    let decision = owner_views("decision-ac5-resource", 2);
    let surrounding = owner_views("surrounding-ac5-resource", 2);
    let trace = trace::ValidatedTrace::admit(
        &trace_document("trace:ac5-resource", true),
        OwnerLimits::default(),
    )
    .unwrap();
    let input = base_input(
        &formula,
        &propositions,
        &trace,
        &decision,
        &surrounding,
        "native-subject:ac5-resource",
        "native-tl-correspondence:ac5-resource",
    );
    let over_ceiling = OwnerLimits {
        max_positions: 1,
        ..OwnerLimits::default()
    };
    let error = request::derive(input, over_ceiling).unwrap_err();
    assert_eq!(error.code(), OwnerReadErrorCode::ResourceIncomplete);
}

// Trace: TC-086, FR-004-AC-6
#[test]
fn tc_086_evaluation_time_refusal_is_a_typed_non_value_never_a_boolean() {
    let decision = owner_views("decision-ac6", 2);
    let surrounding = owner_views("surrounding-ac6", 2);
    let formula = future_formula(SemanticProfile::ClosedTraceV1);
    let propositions = proposition_map();
    let trace =
        trace::ValidatedTrace::admit(&trace_document("trace:ac6", true), OwnerLimits::default())
            .unwrap();
    let input = base_input(
        &formula,
        &propositions,
        &trace,
        &decision,
        &surrounding,
        "native-subject:ac6",
        "native-tl-correspondence:ac6",
    );
    let request = admit_request(input, OwnerLimits::default());

    // An over-ceiling resource case discovered during evaluation (rather
    // than at read time) yields Completed's typed non-value, never a
    // Boolean, a pending truth, a defaulted value, or a dropped outcome.
    let starved = OwnerLimits {
        max_evaluation_steps: 0,
        ..OwnerLimits::default()
    };
    let result_document =
        report::evaluate(&request, report::ResultRelationInput::Original, starved).unwrap();
    let result = report::read(
        result_document.bytes(),
        &request,
        report::ResultRelationInput::Original,
        starved,
    )
    .unwrap();
    assert_eq!(
        result.execution(),
        report::AssessmentExecution::ResourceIncomplete
    );
    let selection = MappingSelection::for_result(&result);
    let mapping_document = contract_ir::map(&result, &selection, starved).unwrap();
    let mapping =
        contract_ir::read(mapping_document.bytes(), &result, &selection, starved).unwrap();
    assert_eq!(
        mapping.outcome(),
        MappedOutcome::NonValue {
            reason: NonValueKind::ResourceIncomplete
        }
    );

    // `contract_ir::outcome`'s match over `AssessmentExecution` has no
    // wildcard arm (see `src/contract_ir.rs`): every execution disposition
    // -- including the three (`Unsupported`, `Failed`, `Refused`) that
    // FR-005's census documents as unreachable through this crate's own
    // strict admission boundary -- is exhaustively typed at compile time,
    // so none can be silently dropped even though this crate never
    // constructs a request that reaches them.
}

// Trace: TC-086, FR-004-AC-7
#[test]
fn tc_086_unknown_contract_label_is_refused_and_field_sets_are_pinned() {
    let decision = owner_views("decision-ac7", 2);
    let surrounding = owner_views("surrounding-ac7", 2);
    let formula = future_formula(SemanticProfile::ClosedTraceV1);
    let propositions = proposition_map();
    let trace =
        trace::ValidatedTrace::admit(&trace_document("trace:ac7", true), OwnerLimits::default())
            .unwrap();
    let input = base_input(
        &formula,
        &propositions,
        &trace,
        &decision,
        &surrounding,
        "native-subject:ac7",
        "native-tl-correspondence:ac7",
    );
    let request = admit_request(input, OwnerLimits::default());
    let result_document = report::evaluate(
        &request,
        report::ResultRelationInput::Original,
        OwnerLimits::default(),
    )
    .unwrap();
    let result = report::read(
        result_document.bytes(),
        &request,
        report::ResultRelationInput::Original,
        OwnerLimits::default(),
    )
    .unwrap();
    let selection = MappingSelection::for_result(&result);
    let mapping_document = contract_ir::map(&result, &selection, OwnerLimits::default()).unwrap();

    let request_document = request::derive(input, OwnerLimits::default()).unwrap();
    for (bytes, needle, contract) in [
        (
            request_document.bytes(),
            request::CONTRACT,
            "quire-mltl.temporal-assessment-request/v0-unknown",
        ),
        (
            result_document.bytes(),
            report::CONTRACT,
            "quire-mltl.temporal-assessment-result/v0-unknown",
        ),
        (
            mapping_document.bytes(),
            contract_ir::CONTRACT,
            "quire-mltl.contract-ir-result-map/v0-unknown",
        ),
    ] {
        let mutated = splice(
            bytes,
            &format!("\"contractVersion\":\"{needle}\""),
            &format!("\"contractVersion\":\"{contract}\""),
        );
        assert_ne!(mutated, bytes);
        let error = match needle {
            n if n == request::CONTRACT => {
                request::read(&mutated, input, OwnerLimits::default()).unwrap_err()
            }
            n if n == report::CONTRACT => report::read(
                &mutated,
                &request,
                report::ResultRelationInput::Original,
                OwnerLimits::default(),
            )
            .unwrap_err(),
            _ => contract_ir::read(&mutated, &result, &selection, OwnerLimits::default())
                .unwrap_err(),
        };
        assert_eq!(error.code(), OwnerReadErrorCode::ContractMismatch);
    }

    // The exact top-level field set of each of the three contracts is
    // pinned: an addition, removal, or rename changes this set and fails
    // the assertion, which is exactly the "test that fails when any of them
    // changes without a successor label" FR-004-AC-7 requires.
    let request_keys = top_level_keys(request_document.bytes());
    let expected_request_keys: BTreeSet<&str> = [
        "contractVersion",
        "identity",
        "lane",
        "semanticProfile",
        "operatorProfile",
        "formula",
        "input",
        "propositionMap",
        "clock",
        "clockIdentity",
        "clockRevision",
        "subjectIdentity",
        "correspondenceIdentity",
        "anchor",
        "evaluator",
        "syntaxRevision",
        "observationRevision",
        "decisionScopeProgress",
        "decisionScopeClosure",
        "surroundingExecutionProgress",
        "surroundingExecutionClosure",
        "completeness",
        "availability",
        "limits",
    ]
    .into_iter()
    .collect();
    assert_eq!(as_str_set(&request_keys), expected_request_keys);

    let result_keys = top_level_keys(result_document.bytes());
    let expected_result_keys: BTreeSet<&str> = [
        "contractVersion",
        "identity",
        "revision",
        "request",
        "lane",
        "subjectIdentity",
        "correspondenceIdentity",
        "formula",
        "input",
        "propositionMap",
        "clock",
        "evaluator",
        "anchor",
        "execution",
        "truth",
        "finalResult",
        "settlement",
        "decisionSupport",
        "decisionScopeProgress",
        "decisionScopeClosure",
        "surroundingExecutionProgress",
        "surroundingExecutionClosure",
        "completeness",
        "availability",
        "relation",
        "usage",
    ]
    .into_iter()
    .collect();
    assert_eq!(as_str_set(&result_keys), expected_result_keys);

    let mapping_keys = top_level_keys(mapping_document.bytes());
    let expected_mapping_keys: BTreeSet<&str> = [
        "contractVersion",
        "identity",
        "sourceResult",
        "outcome",
        "source",
    ]
    .into_iter()
    .collect();
    assert_eq!(as_str_set(&mapping_keys), expected_mapping_keys);
}

// Trace: TC-086, FR-004-AC-8
#[test]
fn tc_086_manifest_declares_no_quire_contract_ir_dependency() {
    let manifest = include_str!("../Cargo.toml");
    assert!(
        !manifest.contains("quire-contract-ir"),
        "quire-mltl must never depend on quire-contract-ir: that dependency \
         would close the TL-181 cycle ADR-002 and FR-004's own Non-goals \
         forbid"
    );
    let lock = include_str!("../Cargo.lock");
    assert!(
        !lock.contains("name = \"quire-contract-ir\""),
        "the lockfile must not resolve a transitive quire-contract-ir crate either"
    );
}

fn splice(bytes: &[u8], needle: &str, replacement: &str) -> Vec<u8> {
    let text = std::str::from_utf8(bytes).unwrap();
    assert!(
        text.contains(needle),
        "expected {needle:?} in the exact canonical bytes"
    );
    text.replacen(needle, replacement, 1).into_bytes()
}

fn top_level_keys(bytes: &[u8]) -> BTreeSet<String> {
    let value: Value = serde_json::from_slice(bytes).unwrap();
    value
        .as_object()
        .unwrap()
        .keys()
        .cloned()
        .collect::<BTreeSet<_>>()
}

fn as_str_set(keys: &BTreeSet<String>) -> BTreeSet<&str> {
    keys.iter().map(String::as_str).collect()
}
