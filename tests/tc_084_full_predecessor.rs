//! TC-084: complete predecessor custody and non-evaluating exact validation.

mod support;

use quire_mltl::report::{
    self, ResultRelationInput, ResultRelationKind, SuppliedPredecessorError,
    ValidatedTemporalResult,
};
use quire_mltl::request::ValidatedTemporalRequest;
use support::{
    admit_request, future_formula, future_request_input, observations, owner_views,
    proposition_map, trace_document,
};
use tl_mltl::wire::{trace, OwnerLimits, OwnerReadErrorCode};
use tl_syntax::SemanticProfile;

fn admitted_request() -> ValidatedTemporalRequest {
    let decision = owner_views("predecessor-decision", 2);
    let surrounding = owner_views("predecessor-surrounding", 2);
    let formula = future_formula(SemanticProfile::ClosedTraceV1);
    let propositions = proposition_map();
    let trace = trace::ValidatedTrace::admit(
        &trace_document("trace:predecessor", true),
        OwnerLimits::default(),
    )
    .unwrap();
    let axes = observations(
        &decision,
        &surrounding,
        true,
        true,
        true,
        true,
        &decision.completeness_complete,
        &decision.availability_available,
    );
    admit_request(
        future_request_input(
            &formula,
            &propositions,
            &trace,
            &decision,
            &surrounding,
            axes,
        ),
        OwnerLimits::default(),
    )
}

fn admitted_result(
    request: &ValidatedTemporalRequest,
    relation: ResultRelationInput<'_>,
) -> ValidatedTemporalResult {
    let document = report::evaluate(request, relation, OwnerLimits::default()).unwrap();
    report::read(document.bytes(), request, relation, OwnerLimits::default()).unwrap()
}

/// Trace: TC-084, FR-001-AC-11
#[test]
fn corrections_keep_complete_direct_predecessor_after_its_source_is_dropped() {
    let request = admitted_request();
    for invalidating in [false, true] {
        let original = admitted_result(&request, ResultRelationInput::Original);
        let original_bytes = original.bytes().to_vec();
        let relation = if invalidating {
            ResultRelationInput::Invalidating(&original)
        } else {
            ResultRelationInput::Superseding(&original)
        };
        let correction = admitted_result(&request, relation);
        assert_eq!(correction.revision(), 2);
        assert_eq!(
            correction.relation_kind(),
            if invalidating {
                ResultRelationKind::Invalidating
            } else {
                ResultRelationKind::Superseding
            }
        );
        assert_eq!(
            correction.direct_predecessor_bytes(),
            Some(original.bytes())
        );
        assert_eq!(
            correction.direct_predecessor_identity(),
            Some(original.identity())
        );
        let correction_bytes = correction.bytes().to_vec();
        let next = admitted_result(&request, ResultRelationInput::Superseding(&correction));
        drop(original);
        assert_eq!(
            correction.direct_predecessor_bytes(),
            Some(original_bytes.as_slice())
        );
        drop(correction);
        assert_eq!(next.revision(), 3);
        assert_eq!(
            next.direct_predecessor_bytes(),
            Some(correction_bytes.as_slice())
        );
        assert_ne!(
            next.direct_predecessor_bytes(),
            Some(original_bytes.as_slice())
        );
        assert_eq!(
            next.validate_supplied_predecessor(Some(&correction_bytes)),
            Ok(())
        );
    }
}

/// Trace: TC-084, FR-001-AC-12
#[test]
fn exact_validation_refuses_same_identity_with_changed_content_and_partial_offers() {
    let request = admitted_request();
    let original = admitted_result(&request, ResultRelationInput::Original);
    let correction = admitted_result(&request, ResultRelationInput::Superseding(&original));
    let unchanged = correction.clone();
    assert_eq!(
        correction.validate_supplied_predecessor(Some(original.bytes())),
        Ok(())
    );
    assert_eq!(
        correction.validate_supplied_predecessor(None),
        Err(SuppliedPredecessorError::Missing)
    );
    let mut changed: serde_json::Value = serde_json::from_slice(original.bytes()).unwrap();
    let original_identity = changed["identity"].clone();
    let prior_truth = changed["truth"].clone();
    changed["truth"] = if prior_truth == "satisfied" {
        serde_json::json!("violated")
    } else {
        serde_json::json!("satisfied")
    };
    assert_eq!(changed["identity"], original_identity);
    assert_ne!(changed["truth"], prior_truth);
    let changed_bytes = serde_json::to_vec(&changed).unwrap();
    let mut extended = original.bytes().to_vec();
    extended.push(b' ');
    let original_bytes = original.bytes();
    for offered in [
        changed_bytes.as_slice(),
        &original_bytes[..original_bytes.len() - 1],
        extended.as_slice(),
        original.identity().as_bytes(),
        b"not JSON".as_slice(),
        b"".as_slice(),
    ] {
        assert_eq!(
            correction.validate_supplied_predecessor(Some(offered)),
            Err(SuppliedPredecessorError::Mismatch)
        );
    }
    assert_eq!(correction, unchanged);
    assert_eq!(correction.usage(), unchanged.usage());
    assert_eq!(correction.bytes(), unchanged.bytes());
    assert_eq!(
        correction.direct_predecessor_bytes(),
        Some(original.bytes())
    );
}

/// Trace: TC-084, FR-001-AC-12
#[test]
fn original_accepts_only_absence_without_changing_its_result() {
    let request = admitted_request();
    let original = admitted_result(&request, ResultRelationInput::Original);
    let unchanged = original.clone();
    assert_eq!(original.direct_predecessor_bytes(), None);
    assert_eq!(original.validate_supplied_predecessor(None), Ok(()));
    for supplied in [original.bytes(), b"".as_slice(), b"not JSON".as_slice()] {
        assert_eq!(
            original.validate_supplied_predecessor(Some(supplied)),
            Err(SuppliedPredecessorError::Unexpected)
        );
    }
    assert_eq!(original, unchanged);
}

/// Trace: TC-084, FR-001-AC-11
#[test]
fn predecessor_retention_obeys_the_current_callers_input_byte_ceiling() {
    let request = admitted_request();
    let original = admitted_result(&request, ResultRelationInput::Original);
    let predecessor = admitted_result(&request, ResultRelationInput::Invalidating(&original));
    let relation = ResultRelationInput::Superseding(&predecessor);
    let document = report::evaluate(&request, relation, OwnerLimits::default()).unwrap();
    // "invalidating" is one byte longer than "superseding". Both revisions
    // have one digit, so the earlier complete document is the larger input.
    assert!(predecessor.bytes().len() > document.bytes().len());
    let at_ceiling = OwnerLimits {
        max_input_bytes: predecessor.bytes().len(),
        ..OwnerLimits::default()
    };
    let result = report::read(document.bytes(), &request, relation, at_ceiling).unwrap();
    assert_eq!(result.direct_predecessor_bytes(), Some(predecessor.bytes()));
    let error = report::read(
        document.bytes(),
        &request,
        relation,
        OwnerLimits {
            max_input_bytes: predecessor.bytes().len() - 1,
            ..OwnerLimits::default()
        },
    )
    .unwrap_err();
    assert_eq!(error.code(), OwnerReadErrorCode::ResourceIncomplete);
    assert_eq!(error.field(), "directPredecessor");
    assert_eq!(error.usage(), predecessor.usage());
    assert_eq!(
        result.validate_supplied_predecessor(Some(predecessor.bytes())),
        Ok(())
    );
}
