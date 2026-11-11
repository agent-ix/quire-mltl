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

fn field_value(text: &str, key: &str) -> String {
    let marker = format!("\"{key}\":");
    let rest = text.split(&marker).nth(1).unwrap();
    let end = rest.find([',', '}']).unwrap();
    rest[..end].to_owned()
}

fn forged_code(request: &ValidatedTemporalRequest, forged: &str) -> OwnerReadErrorCode {
    read_original(
        reidentify(forged).as_bytes(),
        request,
        OwnerLimits::default(),
    )
    .unwrap_err()
    .code()
}

/// Every lexical ceiling is enforced on the bytes read, and the usage returned
/// is measured from them (equal to the producer's), admitted at the measured
/// ceiling and refused one under.
#[test]
#[trace("FR-001-AC-13")]
fn read_enforces_every_ceiling_and_measures_usage() {
    let (request, text) = produced();
    let defaults = OwnerLimits::default();
    let document = report::evaluate(&request, ResultRelationInput::Original, defaults).unwrap();
    let result = read_original(text.as_bytes(), &request, defaults).unwrap();
    assert_eq!(result.usage(), document.usage());
    let usage = result.usage();
    assert!(usage.depth > 1 && usage.string_bytes > 1 && usage.visited_fields > 1);
    let at = |limits: OwnerLimits| read_original(text.as_bytes(), &request, limits);
    let cases: [(OwnerLimits, OwnerLimits, &str); 3] = [
        (
            OwnerLimits {
                max_depth: usage.depth,
                ..defaults
            },
            OwnerLimits {
                max_depth: usage.depth - 1,
                ..defaults
            },
            "depth",
        ),
        (
            OwnerLimits {
                max_string_bytes: usage.string_bytes,
                ..defaults
            },
            OwnerLimits {
                max_string_bytes: usage.string_bytes - 1,
                ..defaults
            },
            "stringBytes",
        ),
        (
            OwnerLimits {
                max_visited_fields: usage.visited_fields,
                ..defaults
            },
            OwnerLimits {
                max_visited_fields: usage.visited_fields - 1,
                ..defaults
            },
            "visitedFields",
        ),
    ];
    for (admit, refuse, field) in cases {
        assert!(at(admit).is_ok(), "{field} at ceiling");
        let error = at(refuse).unwrap_err();
        assert_eq!(
            error.code(),
            OwnerReadErrorCode::ResourceIncomplete,
            "{field}"
        );
        assert_eq!(error.field(), field);
    }
    let support = OwnerLimits {
        max_support: usage.support - 1,
        ..defaults
    };
    assert_eq!(
        at(support).unwrap_err().code(),
        OwnerReadErrorCode::ResourceIncomplete
    );
}

/// A re-identified document that changes decision support or an echoed
/// request field is refused: it is internally consistent but not this
/// request's result.
#[test]
#[trace("FR-001-AC-13")]
fn read_refuses_forged_support_and_echoed_request_fields() {
    let (request, text) = produced();
    let anchor = field_value(&text, "anchor");
    let next: u64 = anchor.parse::<u64>().unwrap() + 1;
    let anchored = text.replacen(
        &format!("\"anchor\":{anchor}"),
        &format!("\"anchor\":{next}"),
        1,
    );
    assert_eq!(
        forged_code(&request, &anchored),
        OwnerReadErrorCode::ExpectedMismatch
    );

    let subject = field_value(&text, "subjectIdentity");
    let subjected = text.replacen(&subject, "\"forged-subject\"", 1);
    assert_eq!(
        forged_code(&request, &subjected),
        OwnerReadErrorCode::ExpectedMismatch
    );

    let support = text
        .split("\"decisionSupport\":[")
        .nth(1)
        .and_then(|rest| rest.split(']').next())
        .unwrap();
    assert!(!support.is_empty(), "fixture result is final with support");
    let count = field_value(&text, "support");
    let emptied = text
        .replacen(
            &format!("\"decisionSupport\":[{support}]"),
            "\"decisionSupport\":[]",
            1,
        )
        .replacen(&format!("\"support\":{count}"), "\"support\":0", 1);
    assert_eq!(
        forged_code(&request, &emptied),
        OwnerReadErrorCode::ExpectedMismatch
    );
}

/// A consistent-looking but impossible state is an invalid combination.
#[test]
#[trace("FR-001-AC-13")]
fn read_reaches_invalid_combination() {
    let (request, text) = produced();
    let revised = text.replacen("\"revision\":1", "\"revision\":2", 1);
    assert_eq!(
        forged_code(&request, &revised),
        OwnerReadErrorCode::InvalidCombination
    );
}
