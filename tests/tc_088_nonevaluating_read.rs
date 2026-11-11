//! TC-088: strict read of a produced result is non-evaluating.

mod support;

use ix_trace_rs::trace;
use quire_mltl::report::{self, ResultRelationInput, TemporalTruth, ValidatedTemporalResult};
use quire_mltl::request::ValidatedTemporalRequest;
use sha2::{Digest, Sha256};
use support::{
    admit_request, future_formula, future_request_input, observations, owner_views,
    proposition_map, trace_document,
};
use tl_mltl::wire::{trace, OwnerLimits, OwnerReadError, OwnerReadErrorCode};
use tl_syntax::SemanticProfile;

fn admitted_request() -> ValidatedTemporalRequest {
    let decision = owner_views("nonevaluating-decision", 2);
    let surrounding = owner_views("nonevaluating-surrounding", 2);
    let formula = future_formula(SemanticProfile::ClosedTraceV1);
    let propositions = proposition_map();
    let trace = trace::ValidatedTrace::admit(
        &trace_document("trace:nonevaluating", true),
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

fn produced() -> (ValidatedTemporalRequest, String) {
    let request = admitted_request();
    let document = report::evaluate(
        &request,
        ResultRelationInput::Original,
        OwnerLimits::default(),
    )
    .unwrap();
    let text = String::from_utf8(document.bytes().to_vec()).unwrap();
    (request, text)
}

fn read_original(
    bytes: &[u8],
    request: &ValidatedTemporalRequest,
    limits: OwnerLimits,
) -> Result<ValidatedTemporalResult, OwnerReadError> {
    report::read(bytes, request, ResultRelationInput::Original, limits)
}

/// Recomputes the domain-separated content identity so a hand-edited document
/// stays internally consistent.
fn reidentify(text: &str) -> String {
    let old = text
        .split("\"identity\":\"")
        .nth(1)
        .and_then(|rest| rest.split('"').next())
        .unwrap()
        .to_owned();
    let preimage = text.replacen(&format!(",\"identity\":\"{old}\""), "", 1);
    let mut digest = Sha256::new();
    digest.update(report::CONTRACT.as_bytes());
    digest.update([0]);
    digest.update(preimage.as_bytes());
    let new: String = digest
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect();
    text.replacen(&old, &new, 1)
}

fn flip_truth(text: &str, truth: TemporalTruth) -> String {
    let (from, to) = match truth {
        TemporalTruth::Satisfied => ("\"truth\":\"satisfied\"", "\"truth\":\"violated\""),
        TemporalTruth::Violated => ("\"truth\":\"violated\"", "\"truth\":\"satisfied\""),
        other => panic!("fixture must be settled, got {other:?}"),
    };
    text.replacen(from, to, 1)
}

/// The recorded truth is carried, not re-derived: a consistent, re-identified
/// document recording the opposite truth is accepted verbatim, which no
/// evaluator run over the request could produce.
#[test]
#[trace("FR-001-AC-13")]
fn read_returns_recorded_truth_without_running_an_evaluator() {
    let (request, text) = produced();
    assert_eq!(
        reidentify(&text),
        text,
        "helper reproduces the owner identity"
    );
    let honest = read_original(text.as_bytes(), &request, OwnerLimits::default()).unwrap();
    let flipped = reidentify(&flip_truth(&text, honest.truth()));
    let result = read_original(flipped.as_bytes(), &request, OwnerLimits::default()).unwrap();
    assert_ne!(result.truth(), honest.truth());
    assert_eq!(result.bytes(), flipped.as_bytes());
}

/// Every refusal is a distinct typed code, reached without evaluation.
#[test]
#[trace("FR-001-AC-13")]
fn read_refuses_with_typed_codes() {
    let (request, text) = produced();
    let defaults = OwnerLimits::default();
    let code = |bytes: &[u8], limits: OwnerLimits| {
        read_original(bytes, &request, limits).unwrap_err().code()
    };
    let one_under = OwnerLimits {
        max_input_bytes: text.len() - 1,
        ..defaults
    };
    assert_eq!(
        code(text.as_bytes(), one_under),
        OwnerReadErrorCode::ResourceIncomplete
    );
    assert_eq!(
        code(&[0xff, 0xfe], defaults),
        OwnerReadErrorCode::InvalidUtf8
    );
    assert_eq!(code(b"{}", defaults), OwnerReadErrorCode::InvalidJson);
    assert_eq!(
        code(format!("{text}x").as_bytes(), defaults),
        OwnerReadErrorCode::InvalidJson
    );
    assert_eq!(
        code(
            text.replacen("\"revision\":", "\"revision\": ", 1)
                .as_bytes(),
            defaults
        ),
        OwnerReadErrorCode::NonCanonical
    );
    let honest = read_original(text.as_bytes(), &request, defaults).unwrap();
    assert_eq!(
        code(flip_truth(&text, honest.truth()).as_bytes(), defaults),
        OwnerReadErrorCode::IdentityMismatch
    );
}

/// A document that is valid on its own but does not belong to the supplied
/// relation is refused as an expected mismatch.
#[test]
#[trace("FR-001-AC-13")]
fn read_refuses_a_document_that_does_not_match_the_supplied_relation() {
    let (request, text) = produced();
    let original = read_original(text.as_bytes(), &request, OwnerLimits::default()).unwrap();
    let error = report::read(
        text.as_bytes(),
        &request,
        ResultRelationInput::Superseding(&original),
        OwnerLimits::default(),
    )
    .unwrap_err();
    assert_eq!(error.code(), OwnerReadErrorCode::ExpectedMismatch);
}
