//! TC-084/TC-085: the exact QObs admission boundary end to end (FR-001,
//! FR-002, FR-003). Ported from `tl-mltl`'s `tests/tc_084_temporal_owner_wire.rs`
//! (TL-178/TL-175); `tl-mltl`'s own FR-018/FR-019 described this same wire
//! boundary before the port.
//!
//! `quire_observation::AdmissionRequest.producer` is typed
//! `agent_ix_baseline_producer::AdmittedStaticBundle` — a real, currently
//! load-bearing member of QObs's public admission API, pinned by both this
//! repository and `quire-observation` itself at the same pre-deletion
//! `filament-core-data` git revision (`a6fe662`, PR agent-ix/filament-core-data#99).
//! `filament-core-data` deliberately deleted the producer-interface-1.2
//! surface from its own HEAD (agent-ix/filament-core-data#144/#145,
//! 2026-09-16) in favor of a Semantic IR / extraction-frontend architecture
//! that answers a different question (spec-artifact lifting); it did not
//! retire the boundary QObs actually depends on, which still resolves fine
//! by pinned SHA.
//!
//! `AdmittedStaticBundle` has no public constructor and no `Deserialize`
//! impl by the producer crate's own design (FR-117-CON-2): the only way to
//! obtain one is `StaticProducerBundle::admit`/`admit_json`, which validate
//! and construct indivisibly. This module used to reach that boundary
//! through a vendored copy of one of `filament-core-data`'s own fixtures
//! (`fixtures/baseline-1-2/static-bundle-a.json`, AGPL-3.0-only) via
//! `admit_json`. `StaticProducerBundle` itself — the *unvalidated offer*, as
//! opposed to the admitted type — has entirely public fields and derives
//! `Default`, so [`minimal_producer_bundle`] builds one directly from the
//! compiled `agent-ix-baseline-producer` dependency's own public types
//! (`ConfigurationDocument`, `ModelSelection`, `StaticClosure`, ...) and
//! calls `.admit()`, the crate's real validation path — no JSON, no
//! vendored bytes, and no in-repo copy of anything `filament-core-data`
//! authors.

mod support;

use quire_mltl::contract_ir::{self, MappedOutcome, MappingSelection, NonValueKind};
use quire_mltl::{dispatch, report, request};
use quire_observation::authority;
use serde_json::Value;
use support::{
    admit_history, admit_request, assert_closed_schema, exact_wire_identity,
    fixed_history_document, future_formula, future_request_input, history_document, observations,
    owner_views, owner_views_with_clock, past_formula, proposition_map, sha256,
    structural_mutations, trace_document, FixtureClock,
};
use tl_mltl::past::{history, requirement, result};
use tl_mltl::wire::{command, trace};
use tl_mltl::wire::{OwnerLimits, OwnerReadErrorCode};
use tl_mltl::{
    analyze_required_history, CommandDocument, CommandSchemaVersion, Operation,
    PastEvaluationLimits, PastEvaluationRelationInput, TraceDocument, TraceSchemaVersion,
};
use tl_syntax::{
    FormulaDocument, Interval, Node, NodeId, NodeKind, PropositionId, SemanticProfile,
};

// Trace: TC-084, FR-001-AC-1, FR-003-AC-1
#[test]
fn tc_084_future_and_past_requests_evaluate_and_strict_read() {
    let decision = owner_views("decision", 2);
    let surrounding = owner_views("surrounding", 2);
    let propositions = proposition_map();
    let formula = future_formula(SemanticProfile::ClosedTraceV1);
    let trace = trace::ValidatedTrace::admit(
        &trace_document("trace:future", true),
        OwnerLimits::default(),
    )
    .unwrap();
    let future_input = future_request_input(
        &formula,
        &propositions,
        &trace,
        &decision,
        &surrounding,
        observations(
            &decision,
            &surrounding,
            true,
            true,
            false,
            true,
            &decision.completeness_complete,
            &decision.availability_available,
        ),
    );
    let future_request = admit_request(future_input, OwnerLimits::default());
    assert_eq!(future_request.lane(), request::TemporalLane::Future);
    assert_eq!(
        future_request.semantic_profile(),
        SemanticProfile::ClosedTraceV1
    );
    assert_eq!(future_request.input_artifact().contract(), trace::CONTRACT);
    assert_eq!(future_request.clock_identity(), "clock:event-position");
    assert_eq!(future_request.clock_revision(), "1");
    assert_eq!(future_request.anchor(), 0);
    assert_eq!(
        future_request.completeness().population_identity(),
        future_request
            .decision_scope_progress()
            .population_identity()
    );
    assert_eq!(
        future_request.identity(),
        exact_wire_identity(
            request::CONTRACT,
            future_request.bytes(),
            future_request.identity(),
        )
    );
    let result_document = report::evaluate(
        &future_request,
        report::ResultRelationInput::Original,
        OwnerLimits::default(),
    )
    .unwrap();
    let result = report::read(
        result_document.bytes(),
        &future_request,
        report::ResultRelationInput::Original,
        OwnerLimits::default(),
    )
    .unwrap();
    assert_eq!(result.truth(), report::TemporalTruth::Satisfied);
    assert_eq!(
        result.decision_scope_progress().authority_identity(),
        future_request
            .decision_scope_progress()
            .authority_identity()
    );
    assert_eq!(result.relation_kind(), report::ResultRelationKind::Original);
    assert_eq!(result.direct_predecessor_identity(), None);
    assert_eq!(
        result.identity(),
        exact_wire_identity(report::CONTRACT, result.bytes(), result.identity())
    );
    assert_eq!(result.usage().evaluation_steps, 1);
    let selection = MappingSelection::for_result(&result);
    let mapped_document = contract_ir::map(&result, &selection, OwnerLimits::default()).unwrap();
    let mapped = contract_ir::read(
        mapped_document.bytes(),
        &result,
        &selection,
        OwnerLimits::default(),
    )
    .unwrap();
    assert_eq!(mapped.outcome(), MappedOutcome::Value { value: true });
    assert_eq!(mapped.source().identity(), result.identity());
    assert_eq!(
        mapped.identity(),
        exact_wire_identity(contract_ir::CONTRACT, mapped.bytes(), mapped.identity())
    );

    let formula = past_formula();
    let history_value = history_document("history:past", 1, [true, false]);
    let history = admit_history(&history_value);
    let past_input = request::RequestInput {
        formula: &formula,
        proposition_map: &propositions,
        input: request::TemporalInput::Past(&history),
        clock: &decision.clock,
        subject_identity: "native-subject:order-1",
        correspondence_identity: "native-tl-correspondence:order-1",
        anchor: 1,
        observations: observations(
            &decision,
            &surrounding,
            true,
            true,
            true,
            true,
            &decision.completeness_complete,
            &decision.availability_available,
        ),
    };
    let past_request = admit_request(past_input, OwnerLimits::default());
    let past_document = report::evaluate(
        &past_request,
        report::ResultRelationInput::Original,
        OwnerLimits::default(),
    )
    .unwrap();
    let past = report::read(
        past_document.bytes(),
        &past_request,
        report::ResultRelationInput::Original,
        OwnerLimits::default(),
    )
    .unwrap();
    assert_eq!(past.truth(), report::TemporalTruth::Satisfied);
    assert!(past.is_final());
}

// Trace: TC-084, FR-001-AC-2, FR-001-AC-4
#[test]
fn tc_084_all_profiles_past_operators_and_supported_clocks_use_one_owner_path() {
    let decision = owner_views("decision-profiles", 2);
    let surrounding = owner_views("surrounding-profiles", 2);
    let propositions = proposition_map();
    let history_value = history_document("history:operators", 1, [true, false]);
    let history = admit_history(&history_value);
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
    let past_cases = [
        (
            FormulaDocument::new_v2(
                SemanticProfile::OriginCompleteHistoryV1,
                NodeId(1),
                vec![
                    Node::new(NodeKind::Proposition {
                        proposition: PropositionId(0),
                    }),
                    Node::new(NodeKind::Once {
                        interval: Interval::new(0, 1).unwrap(),
                        operand: NodeId(0),
                    }),
                ],
            )
            .unwrap(),
            report::TemporalTruth::Satisfied,
        ),
        (
            FormulaDocument::new_v2(
                SemanticProfile::OriginCompleteHistoryV1,
                NodeId(1),
                vec![
                    Node::new(NodeKind::Proposition {
                        proposition: PropositionId(0),
                    }),
                    Node::new(NodeKind::Historically {
                        interval: Interval::new(1, 3).unwrap(),
                        operand: NodeId(0),
                    }),
                ],
            )
            .unwrap(),
            report::TemporalTruth::Violated,
        ),
        (
            FormulaDocument::new_v2(
                SemanticProfile::OriginCompleteHistoryV1,
                NodeId(1),
                vec![
                    Node::new(NodeKind::Proposition {
                        proposition: PropositionId(0),
                    }),
                    Node::new(NodeKind::StrongPrevious { operand: NodeId(0) }),
                ],
            )
            .unwrap(),
            report::TemporalTruth::Satisfied,
        ),
        (
            FormulaDocument::new_v2(
                SemanticProfile::OriginCompleteHistoryV1,
                NodeId(2),
                vec![
                    Node::new(NodeKind::True),
                    Node::new(NodeKind::Proposition {
                        proposition: PropositionId(0),
                    }),
                    Node::new(NodeKind::Since {
                        interval: Interval::new(1, 1).unwrap(),
                        left: NodeId(0),
                        right: NodeId(1),
                    }),
                ],
            )
            .unwrap(),
            report::TemporalTruth::Satisfied,
        ),
        (
            FormulaDocument::new_v2(
                SemanticProfile::OriginCompleteHistoryV1,
                NodeId(2),
                vec![
                    Node::new(NodeKind::True),
                    Node::new(NodeKind::True),
                    Node::new(NodeKind::Triggered {
                        interval: Interval::new(0, 1).unwrap(),
                        left: NodeId(0),
                        right: NodeId(1),
                    }),
                ],
            )
            .unwrap(),
            report::TemporalTruth::Satisfied,
        ),
    ];
    for (formula, expected) in past_cases {
        let input = request::RequestInput {
            formula: &formula,
            proposition_map: &propositions,
            input: request::TemporalInput::Past(&history),
            clock: &decision.clock,
            subject_identity: "native-subject:operator-catalog",
            correspondence_identity: "native-tl-correspondence:operator-catalog",
            anchor: 1,
            observations: axes,
        };
        let request = admit_request(input, OwnerLimits::default());
        let document = report::evaluate(
            &request,
            report::ResultRelationInput::Original,
            OwnerLimits::default(),
        )
        .unwrap();
        let result = report::read(
            document.bytes(),
            &request,
            report::ResultRelationInput::Original,
            OwnerLimits::default(),
        )
        .unwrap();
        assert_eq!(result.truth(), expected);
    }

    let prefix_decision = owner_views("decision-prefix", 1);
    let prefix_surrounding = owner_views("surrounding-prefix", 1);
    let prefix_formula = FormulaDocument::new(
        SemanticProfile::OnlinePrefixV1,
        NodeId(1),
        vec![
            Node::new(NodeKind::Proposition {
                proposition: PropositionId(0),
            }),
            Node::new(NodeKind::Future {
                interval: Interval::new(1, 1).unwrap(),
                operand: NodeId(0),
            }),
        ],
    )
    .unwrap();
    let prefix_trace = trace::ValidatedTrace::admit(
        &TraceDocument {
            schema_version: TraceSchemaVersion::V1,
            trace_id: "trace:open-prefix".to_owned(),
            closed: false,
            instants: vec![vec![]],
        },
        OwnerLimits::default(),
    )
    .unwrap();
    let prefix_input = future_request_input(
        &prefix_formula,
        &propositions,
        &prefix_trace,
        &prefix_decision,
        &prefix_surrounding,
        observations(
            &prefix_decision,
            &prefix_surrounding,
            false,
            false,
            false,
            false,
            &prefix_decision.completeness_complete,
            &prefix_decision.availability_available,
        ),
    );
    let prefix_request = admit_request(prefix_input, OwnerLimits::default());
    let prefix_document = report::evaluate(
        &prefix_request,
        report::ResultRelationInput::Original,
        OwnerLimits::default(),
    )
    .unwrap();
    let prefix_result = report::read(
        prefix_document.bytes(),
        &prefix_request,
        report::ResultRelationInput::Original,
        OwnerLimits::default(),
    )
    .unwrap();
    assert_eq!(prefix_result.truth(), report::TemporalTruth::Pending);
    let selection = MappingSelection::for_result(&prefix_result);
    let document = contract_ir::map(&prefix_result, &selection, OwnerLimits::default()).unwrap();
    let mapped = contract_ir::read(
        document.bytes(),
        &prefix_result,
        &selection,
        OwnerLimits::default(),
    )
    .unwrap();
    assert_eq!(
        mapped.outcome(),
        MappedOutcome::NonValue {
            reason: NonValueKind::Pending
        }
    );

    let fixed_decision = owner_views_with_clock("decision-fixed", 2, FixtureClock::FixedSample);
    let fixed_surrounding =
        owner_views_with_clock("surrounding-fixed", 2, FixtureClock::FixedSample);
    let fixed_formula = past_formula();
    let fixed_history_value = fixed_history_document("history:fixed");
    let fixed_history = admit_history(&fixed_history_value);
    let fixed_input = request::RequestInput {
        formula: &fixed_formula,
        proposition_map: &propositions,
        input: request::TemporalInput::Past(&fixed_history),
        clock: &fixed_decision.clock,
        subject_identity: "native-subject:fixed-clock",
        correspondence_identity: "native-tl-correspondence:fixed-clock",
        anchor: 1,
        observations: observations(
            &fixed_decision,
            &fixed_surrounding,
            true,
            true,
            true,
            true,
            &fixed_decision.completeness_complete,
            &fixed_decision.availability_available,
        ),
    };
    let fixed_request = admit_request(fixed_input, OwnerLimits::default());
    let fixed_document = report::evaluate(
        &fixed_request,
        report::ResultRelationInput::Original,
        OwnerLimits::default(),
    )
    .unwrap();
    let fixed_result = report::read(
        fixed_document.bytes(),
        &fixed_request,
        report::ResultRelationInput::Original,
        OwnerLimits::default(),
    )
    .unwrap();
    assert_eq!(fixed_result.truth(), report::TemporalTruth::Satisfied);
}

// Trace: TC-084, FR-001-AC-9
#[test]
fn tc_084_past_lane_refuses_mismatched_clock_binding() {
    // An `EventPosition` clock paired with a `FixedSample`-bound history is
    // neither of the two pairings FR-001-AC-9 admits (`EventPosition`/
    // `EventPosition` or an exactly-agreeing `FixedSample`/`FixedSample`), so
    // it must be refused with `ExpectedMismatch("clockBinding")` rather than
    // silently evaluated.
    let decision = owner_views("decision-clock-binding-mismatch", 2);
    let surrounding = owner_views("surrounding-clock-binding-mismatch", 2);
    let propositions = proposition_map();
    let formula = past_formula();
    let mismatched_history_value = fixed_history_document("history:clock-binding-mismatch");
    let mismatched_history = admit_history(&mismatched_history_value);
    let input = request::RequestInput {
        formula: &formula,
        proposition_map: &propositions,
        input: request::TemporalInput::Past(&mismatched_history),
        clock: &decision.clock,
        subject_identity: "native-subject:clock-binding-mismatch",
        correspondence_identity: "native-tl-correspondence:clock-binding-mismatch",
        anchor: 1,
        observations: observations(
            &decision,
            &surrounding,
            true,
            true,
            true,
            true,
            &decision.completeness_complete,
            &decision.availability_available,
        ),
    };
    let error = request::derive(input, OwnerLimits::default()).unwrap_err();
    assert_eq!(error.code(), OwnerReadErrorCode::ExpectedMismatch);
    assert_eq!(error.field(), "clockBinding");
}

// Trace: TC-084, FR-001-AC-7, NFR-001-AC-1
#[test]
fn tc_084_schemas_are_pinned_and_all_readers_fail_closed() {
    for (bytes, digest) in [
        (trace::SCHEMA_BYTES, trace::SCHEMA_SHA256),
        (command::SCHEMA_BYTES, command::SCHEMA_SHA256),
        (history::SCHEMA_BYTES, history::SCHEMA_SHA256),
        (requirement::SCHEMA_BYTES, requirement::SCHEMA_SHA256),
        (result::SCHEMA_BYTES, result::SCHEMA_SHA256),
        (request::SCHEMA_BYTES, request::SCHEMA_SHA256),
        (report::SCHEMA_BYTES, report::SCHEMA_SHA256),
        (contract_ir::SCHEMA_BYTES, contract_ir::SCHEMA_SHA256),
    ] {
        assert_eq!(sha256(bytes), digest);
        let schema: Value = serde_json::from_slice(bytes).unwrap();
        assert_closed_schema(&schema);
    }

    let trace_document = trace_document("trace:strict", true);
    let trace_owner = trace::derive(&trace_document, OwnerLimits::default()).unwrap();
    trace::read(trace_owner.bytes(), &trace_document, OwnerLimits::default()).unwrap();
    for mutated in structural_mutations(
        trace_owner.bytes(),
        "schemaVersion",
        trace::CONTRACT,
        "traceId",
    ) {
        assert!(trace::read(&mutated, &trace_document, OwnerLimits::default()).is_err());
    }

    let command = CommandDocument {
        schema_version: CommandSchemaVersion::V1,
        operation: Operation::Evaluate,
        formula_id: "formula:strict".to_owned(),
        formula: future_formula(SemanticProfile::ClosedTraceV1),
        trace: Some(trace_document.clone()),
    };
    let command_owner = command::derive(&command, OwnerLimits::default()).unwrap();
    command::read(command_owner.bytes(), &command, OwnerLimits::default()).unwrap();
    for mutated in structural_mutations(
        command_owner.bytes(),
        "schemaVersion",
        command::CONTRACT,
        "formulaId",
    ) {
        assert!(command::read(&mutated, &command, OwnerLimits::default()).is_err());
    }

    let history_value = history_document("history:strict", 1, [true, false]);
    let history_owner = history::derive(&history_value, OwnerLimits::default()).unwrap();
    history::read(
        history_owner.bytes(),
        &history_value,
        OwnerLimits::default(),
    )
    .unwrap();
    for mutated in structural_mutations(
        history_owner.bytes(),
        "schemaVersion",
        tl_mltl::POSITION_HISTORY_V1,
        "historyId",
    ) {
        assert!(history::read(&mutated, &history_value, OwnerLimits::default()).is_err());
    }

    let requirement_value =
        analyze_required_history(past_formula().validate().unwrap(), "formula:strict-past")
            .unwrap();
    let requirement_owner =
        requirement::derive(&requirement_value, OwnerLimits::default()).unwrap();
    requirement::read(
        requirement_owner.bytes(),
        &requirement_value,
        OwnerLimits::default(),
    )
    .unwrap();
    for mutated in structural_mutations(
        requirement_owner.bytes(),
        "schemaVersion",
        tl_mltl::HISTORY_REQUIREMENT_V1,
        "formulaId",
    ) {
        assert!(requirement::read(&mutated, &requirement_value, OwnerLimits::default(),).is_err());
    }

    let past_value = tl_mltl::evaluate_past(
        past_formula().validate().unwrap(),
        "formula:strict-past",
        &history_value,
        1,
        "map:strict",
        1,
        PastEvaluationRelationInput::Original,
        PastEvaluationLimits::default(),
    )
    .unwrap();
    let past_owner = result::derive(&past_value, OwnerLimits::default()).unwrap();
    result::read(past_owner.bytes(), &past_value, OwnerLimits::default()).unwrap();
    for mutated in structural_mutations(
        past_owner.bytes(),
        "schemaVersion",
        tl_mltl::PAST_EVALUATION_V1,
        "resultSha256",
    ) {
        assert!(result::read(&mutated, &past_value, OwnerLimits::default()).is_err());
    }
}

// Trace: TC-084, FR-001-AC-3, FR-003-AC-1
#[test]
fn tc_084_owner_axes_remain_independent_and_non_values_are_total() {
    let decision = owner_views("decision-combinations", 2);
    let surrounding = owner_views("surrounding-combinations", 2);
    let formula = future_formula(SemanticProfile::ClosedTraceV1);
    let propositions = proposition_map();
    let trace = trace::ValidatedTrace::admit(
        &trace_document("trace:combinations", true),
        OwnerLimits::default(),
    )
    .unwrap();
    let completeness = [
        &decision.completeness_complete,
        &decision.completeness_incomplete,
        &decision.completeness_contradicted,
    ];
    let availability = [
        &decision.availability_available,
        &decision.availability_not_yet,
        &decision.availability_producer_unavailable,
        &decision.availability_contract_unavailable,
    ];
    let mut combinations = 0usize;
    for decision_progress in [false, true] {
        for decision_closure in [false, true] {
            for surrounding_progress in [false, true] {
                for surrounding_closure in [false, true] {
                    for completeness in completeness {
                        for availability in availability {
                            let input = future_request_input(
                                &formula,
                                &propositions,
                                &trace,
                                &decision,
                                &surrounding,
                                observations(
                                    &decision,
                                    &surrounding,
                                    decision_progress,
                                    decision_closure,
                                    surrounding_progress,
                                    surrounding_closure,
                                    completeness,
                                    availability,
                                ),
                            );
                            let request = admit_request(input, OwnerLimits::default());
                            let document = report::evaluate(
                                &request,
                                report::ResultRelationInput::Original,
                                OwnerLimits::default(),
                            )
                            .unwrap();
                            let result = report::read(
                                document.bytes(),
                                &request,
                                report::ResultRelationInput::Original,
                                OwnerLimits::default(),
                            )
                            .unwrap();
                            let selection = MappingSelection::for_result(&result);
                            let document =
                                contract_ir::map(&result, &selection, OwnerLimits::default())
                                    .unwrap();
                            let mapped = contract_ir::read(
                                document.bytes(),
                                &result,
                                &selection,
                                OwnerLimits::default(),
                            )
                            .unwrap();
                            let expected = match completeness.payload().state() {
                                authority::completeness::State::Incomplete => {
                                    MappedOutcome::NonValue {
                                        reason: NonValueKind::Incomplete,
                                    }
                                }
                                authority::completeness::State::Contradicted => {
                                    MappedOutcome::NonValue {
                                        reason: NonValueKind::Contradicted,
                                    }
                                }
                                authority::completeness::State::Complete
                                    if availability.payload().state()
                                        != authority::availability::State::Available =>
                                {
                                    MappedOutcome::NonValue {
                                        reason: NonValueKind::Unavailable,
                                    }
                                }
                                authority::completeness::State::Complete => {
                                    MappedOutcome::Value { value: true }
                                }
                            };
                            assert_eq!(mapped.outcome(), expected);
                            combinations += 1;
                        }
                    }
                }
            }
        }
    }
    assert_eq!(combinations, 192);
}

// Trace: TC-084, FR-001-AC-5, FR-001-AC-7, FR-003-AC-2
#[test]
fn tc_084_request_result_map_and_lineage_reject_substitution() {
    let decision = owner_views("decision-lineage", 2);
    let surrounding = owner_views("surrounding-lineage", 2);
    let formula = future_formula(SemanticProfile::ClosedTraceV1);
    let propositions = proposition_map();
    let first_trace =
        trace::ValidatedTrace::admit(&trace_document("trace:first", true), OwnerLimits::default())
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
    let first_input = future_request_input(
        &formula,
        &propositions,
        &first_trace,
        &decision,
        &surrounding,
        axes,
    );
    let first_request_document = request::derive(first_input, OwnerLimits::default()).unwrap();
    let first_request = request::read(
        first_request_document.bytes(),
        first_input,
        OwnerLimits::default(),
    )
    .unwrap();
    let first_document = report::evaluate(
        &first_request,
        report::ResultRelationInput::Original,
        OwnerLimits::default(),
    )
    .unwrap();
    let first = report::read(
        first_document.bytes(),
        &first_request,
        report::ResultRelationInput::Original,
        OwnerLimits::default(),
    )
    .unwrap();
    let original_bytes = first.bytes().to_vec();

    let second_trace = trace::ValidatedTrace::admit(
        &trace_document("trace:corrected", true),
        OwnerLimits::default(),
    )
    .unwrap();
    let second_input = future_request_input(
        &formula,
        &propositions,
        &second_trace,
        &decision,
        &surrounding,
        axes,
    );
    let second_request = admit_request(second_input, OwnerLimits::default());
    let corrected_document = report::evaluate(
        &second_request,
        report::ResultRelationInput::Superseding(&first),
        OwnerLimits::default(),
    )
    .unwrap();
    let corrected = report::read(
        corrected_document.bytes(),
        &second_request,
        report::ResultRelationInput::Superseding(&first),
        OwnerLimits::default(),
    )
    .unwrap();
    assert_eq!(corrected.revision(), 2);
    assert_eq!(first.bytes(), original_bytes);

    assert_eq!(
        report::read(
            corrected.bytes(),
            &second_request,
            report::ResultRelationInput::Invalidating(&first),
            OwnerLimits::default(),
        )
        .unwrap_err()
        .code(),
        OwnerReadErrorCode::ExpectedMismatch
    );

    let selection = MappingSelection::for_result(&corrected);
    let map = contract_ir::map(&corrected, &selection, OwnerLimits::default()).unwrap();
    let wrong_selection = MappingSelection::for_result(&first);
    assert_eq!(
        contract_ir::read(
            map.bytes(),
            &corrected,
            &wrong_selection,
            OwnerLimits::default(),
        )
        .unwrap_err()
        .code(),
        OwnerReadErrorCode::ExpectedMismatch
    );

    for mutated in structural_mutations(
        first_request_document.bytes(),
        "contractVersion",
        request::CONTRACT,
        "identity",
    ) {
        assert!(request::read(&mutated, first_input, OwnerLimits::default()).is_err());
    }
    for mutated in structural_mutations(
        corrected.bytes(),
        "contractVersion",
        report::CONTRACT,
        "identity",
    ) {
        assert!(report::read(
            &mutated,
            &second_request,
            report::ResultRelationInput::Superseding(&first),
            OwnerLimits::default(),
        )
        .is_err());
    }
    for mutated in structural_mutations(
        map.bytes(),
        "contractVersion",
        contract_ir::CONTRACT,
        "identity",
    ) {
        assert!(
            contract_ir::read(&mutated, &corrected, &selection, OwnerLimits::default(),).is_err()
        );
    }

    let mut mutated_request = first_request_document.bytes().to_vec();
    let identity = first_request.identity();
    let replacement = if identity.starts_with('0') { '1' } else { '0' };
    let position = mutated_request
        .windows(identity.len())
        .position(|window| window == identity.as_bytes())
        .unwrap();
    mutated_request[position] = replacement as u8;
    assert_eq!(
        request::read(&mutated_request, first_input, OwnerLimits::default())
            .unwrap_err()
            .code(),
        OwnerReadErrorCode::IdentityMismatch
    );
}

// Trace: TC-084, FR-001-AC-6
#[test]
fn tc_084_exact_and_one_over_limits_cover_each_owner_dimension() {
    let value = trace_document("trace:limits", true);
    let document = trace::derive(&value, OwnerLimits::default()).unwrap();
    let lexical = document.usage();
    trace::read(
        document.bytes(),
        &value,
        OwnerLimits {
            max_input_bytes: document.bytes().len(),
            max_output_bytes: document.bytes().len(),
            max_depth: lexical.depth,
            max_string_bytes: lexical.string_bytes,
            max_positions: 2,
            max_propositions: 1,
            max_visited_fields: lexical.visited_fields,
            ..OwnerLimits::default()
        },
    )
    .unwrap();
    trace::derive(
        &value,
        OwnerLimits {
            max_output_bytes: document.bytes().len(),
            ..OwnerLimits::default()
        },
    )
    .unwrap();
    for limits in [
        OwnerLimits {
            max_input_bytes: document.bytes().len() - 1,
            ..OwnerLimits::default()
        },
        OwnerLimits {
            max_output_bytes: document.bytes().len() - 1,
            ..OwnerLimits::default()
        },
        OwnerLimits {
            max_depth: lexical.depth - 1,
            ..OwnerLimits::default()
        },
        OwnerLimits {
            max_string_bytes: lexical.string_bytes - 1,
            ..OwnerLimits::default()
        },
        OwnerLimits {
            max_positions: 1,
            ..OwnerLimits::default()
        },
        OwnerLimits {
            max_propositions: 0,
            ..OwnerLimits::default()
        },
        OwnerLimits {
            max_visited_fields: lexical.visited_fields - 1,
            ..OwnerLimits::default()
        },
    ] {
        let error = if limits.max_output_bytes < document.bytes().len() {
            trace::derive(&value, limits).unwrap_err()
        } else {
            trace::read(document.bytes(), &value, limits).unwrap_err()
        };
        assert_eq!(error.code(), OwnerReadErrorCode::ResourceIncomplete);
    }

    let history_value = history_document("history:limits", 1, [true, false]);
    let history_document = history::derive(&history_value, OwnerLimits::default()).unwrap();
    history::read(
        history_document.bytes(),
        &history_value,
        OwnerLimits {
            max_history_span: 1,
            ..OwnerLimits::default()
        },
    )
    .unwrap();
    assert_eq!(
        history::read(
            history_document.bytes(),
            &history_value,
            OwnerLimits {
                max_history_span: 0,
                ..OwnerLimits::default()
            },
        )
        .unwrap_err()
        .code(),
        OwnerReadErrorCode::ResourceIncomplete
    );

    let nested = br#"{"a":{"b":{"c":true}}}"#;
    let trace_error = trace::ValidatedTrace::from_json_bytes(
        nested,
        OwnerLimits {
            max_depth: 2,
            ..OwnerLimits::default()
        },
    )
    .unwrap_err();
    assert_eq!(trace_error.code(), OwnerReadErrorCode::ResourceIncomplete);

    let decision = owner_views("decision-limits", 2);
    let surrounding = owner_views("surrounding-limits", 2);
    let propositions = proposition_map();
    let formula = FormulaDocument::new(
        SemanticProfile::ClosedTraceV1,
        NodeId(1),
        vec![
            Node::new(NodeKind::Proposition {
                proposition: PropositionId(0),
            }),
            Node::new(NodeKind::Not { operand: NodeId(0) }),
        ],
    )
    .unwrap();
    let trace = trace::ValidatedTrace::admit(&value, OwnerLimits::default()).unwrap();
    let input = future_request_input(
        &formula,
        &propositions,
        &trace,
        &decision,
        &surrounding,
        observations(
            &decision,
            &surrounding,
            true,
            true,
            true,
            true,
            &decision.completeness_complete,
            &decision.availability_available,
        ),
    );
    let request_limits = OwnerLimits {
        max_formula_nodes: 2,
        max_formula_depth: 2,
        ..OwnerLimits::default()
    };
    let request = admit_request(input, request_limits);
    for limits in [
        OwnerLimits {
            max_formula_nodes: 1,
            ..OwnerLimits::default()
        },
        OwnerLimits {
            max_formula_depth: 1,
            ..OwnerLimits::default()
        },
    ] {
        assert_eq!(
            request::derive(input, limits).unwrap_err().code(),
            OwnerReadErrorCode::ResourceIncomplete
        );
    }

    let exact_execution = report::evaluate(
        &request,
        report::ResultRelationInput::Original,
        OwnerLimits {
            max_evaluation_steps: 2,
            max_recursion_depth: 1,
            max_support: 1,
            ..OwnerLimits::default()
        },
    )
    .unwrap();
    assert_eq!(exact_execution.usage().evaluation_steps, 2);
    assert_eq!(exact_execution.usage().recursion_depth, 1);
    assert_eq!(exact_execution.usage().support, 1);
    for limits in [
        OwnerLimits {
            max_evaluation_steps: 1,
            ..OwnerLimits::default()
        },
        OwnerLimits {
            max_recursion_depth: 0,
            ..OwnerLimits::default()
        },
        OwnerLimits {
            max_support: 0,
            ..OwnerLimits::default()
        },
    ] {
        let document =
            report::evaluate(&request, report::ResultRelationInput::Original, limits).unwrap();
        let result = report::read(
            document.bytes(),
            &request,
            report::ResultRelationInput::Original,
            limits,
        )
        .unwrap();
        assert_eq!(
            result.execution(),
            report::AssessmentExecution::ResourceIncomplete
        );
        assert!(!result.is_final());
        assert!(result.decision_support().is_empty());
    }
}

// Trace: TC-084, FR-001-AC-2, FR-001-AC-6, FR-003-AC-1
#[test]
fn tc_084_profile_clock_and_resource_refusals_never_coerce_boolean() {
    let decision = owner_views("decision-refusal", 2);
    let surrounding = owner_views("surrounding-refusal", 2);
    let propositions = proposition_map();
    let trace = trace::ValidatedTrace::admit(
        &trace_document("trace:refusal", true),
        OwnerLimits::default(),
    )
    .unwrap();
    let past = past_formula();
    let mismatched = future_request_input(
        &past,
        &propositions,
        &trace,
        &decision,
        &surrounding,
        observations(
            &decision,
            &surrounding,
            true,
            true,
            true,
            true,
            &decision.completeness_complete,
            &decision.availability_available,
        ),
    );
    assert_eq!(
        request::derive(mismatched, OwnerLimits::default())
            .unwrap_err()
            .code(),
        OwnerReadErrorCode::ExpectedMismatch
    );

    let formula = future_formula(SemanticProfile::ClosedTraceV1);
    let input = future_request_input(
        &formula,
        &propositions,
        &trace,
        &decision,
        &surrounding,
        observations(
            &decision,
            &surrounding,
            true,
            true,
            true,
            true,
            &decision.completeness_complete,
            &decision.availability_available,
        ),
    );
    let request = admit_request(input, OwnerLimits::default());
    let constrained = OwnerLimits {
        max_evaluation_steps: 0,
        ..OwnerLimits::default()
    };
    let document =
        report::evaluate(&request, report::ResultRelationInput::Original, constrained).unwrap();
    let result = report::read(
        document.bytes(),
        &request,
        report::ResultRelationInput::Original,
        constrained,
    )
    .unwrap();
    assert_eq!(
        result.execution(),
        report::AssessmentExecution::ResourceIncomplete
    );
    assert_eq!(result.truth(), report::TemporalTruth::Unavailable);
    let selection = MappingSelection::for_result(&result);
    let document = contract_ir::map(&result, &selection, constrained).unwrap();
    let mapped = contract_ir::read(document.bytes(), &result, &selection, constrained).unwrap();
    assert_eq!(
        mapped.outcome(),
        MappedOutcome::NonValue {
            reason: NonValueKind::ResourceIncomplete
        }
    );
}

// Trace: TC-084, FR-001-AC-3
#[test]
fn tc_084_owner_evidence_contexts_cannot_be_cross_wired() {
    let decision = owner_views("decision-cross-wire", 2);
    let surrounding = owner_views("surrounding-cross-wire", 2);
    let formula = future_formula(SemanticProfile::ClosedTraceV1);
    let propositions = proposition_map();
    let trace = trace::ValidatedTrace::admit(
        &trace_document("trace:cross-wire", true),
        OwnerLimits::default(),
    )
    .unwrap();
    let input = future_request_input(
        &formula,
        &propositions,
        &trace,
        &decision,
        &surrounding,
        observations(
            &decision,
            &surrounding,
            true,
            true,
            true,
            true,
            &surrounding.completeness_complete,
            &surrounding.availability_available,
        ),
    );
    assert_eq!(
        request::derive(input, OwnerLimits::default())
            .unwrap_err()
            .field(),
        "decisionEvidenceContext"
    );
}

// Trace: TC-085, FR-002-AC-1, FR-002-AC-2, FR-002-AC-3
#[test]
fn tc_085_qobs_c00_temporal_dispatch_and_unsupported_contracts_are_exact() {
    const C00_REVISION: &str = "2bdeb833a330bfa777c19eb4c28c423f856f3ba6";

    let decision = owner_views("decision-c00", 2);
    let surrounding = owner_views("surrounding-c00", 2);
    let formula = future_formula(SemanticProfile::ClosedTraceV1);
    let propositions = proposition_map();
    let trace =
        trace::ValidatedTrace::admit(&trace_document("trace:c00", true), OwnerLimits::default())
            .unwrap();
    let input = future_request_input(
        &formula,
        &propositions,
        &trace,
        &decision,
        &surrounding,
        observations(
            &decision,
            &surrounding,
            true,
            true,
            true,
            true,
            &decision.completeness_complete,
            &decision.availability_available,
        ),
    );

    let direct = request::derive(input, OwnerLimits::default()).unwrap();
    let dispatched = dispatch::consume_temporal(input, OwnerLimits::default()).unwrap();
    assert_eq!(dispatched, direct);
    let direct_wire: Value =
        serde_json::from_slice(direct.bytes()).expect("temporal request is canonical JSON");
    assert_eq!(direct_wire["observationRevision"], C00_REVISION);

    let tightening = OwnerLimits {
        max_output_bytes: direct.bytes().len(),
        ..OwnerLimits::default()
    };
    let tightened = request::derive(input, tightening).unwrap();
    let exact = OwnerLimits {
        max_output_bytes: tightened.bytes().len(),
        ..OwnerLimits::default()
    };
    let exact_direct = request::derive(input, exact).unwrap();
    assert_eq!(exact_direct.bytes().len(), exact.max_output_bytes);
    let exact_dispatched = dispatch::consume_temporal(input, exact).unwrap();
    assert_eq!(exact_dispatched, exact_direct);
    let one_over = OwnerLimits {
        max_output_bytes: exact.max_output_bytes - 1,
        ..OwnerLimits::default()
    };
    let one_over_direct = request::derive(input, one_over);
    let one_over_dispatched = dispatch::consume_temporal(input, one_over);
    assert_eq!(one_over_dispatched, one_over_direct);
    assert_eq!(
        one_over_direct.unwrap_err().code(),
        OwnerReadErrorCode::ResourceIncomplete
    );

    for (dimension, limits) in [
        (
            "maxInputBytes",
            OwnerLimits {
                max_input_bytes: 1,
                ..OwnerLimits::default()
            },
        ),
        (
            "maxOutputBytes",
            OwnerLimits {
                max_output_bytes: exact.max_output_bytes,
                ..OwnerLimits::default()
            },
        ),
        (
            "maxDepth",
            OwnerLimits {
                max_depth: 1,
                ..OwnerLimits::default()
            },
        ),
        (
            "maxStringBytes",
            OwnerLimits {
                max_string_bytes: 1,
                ..OwnerLimits::default()
            },
        ),
        (
            "maxFormulaNodes",
            OwnerLimits {
                max_formula_nodes: 1,
                ..OwnerLimits::default()
            },
        ),
        (
            "maxFormulaDepth",
            OwnerLimits {
                max_formula_depth: 1,
                ..OwnerLimits::default()
            },
        ),
        (
            "maxPositions",
            OwnerLimits {
                max_positions: 1,
                ..OwnerLimits::default()
            },
        ),
        (
            "maxPropositions",
            OwnerLimits {
                max_propositions: 0,
                ..OwnerLimits::default()
            },
        ),
        (
            "maxSupport",
            OwnerLimits {
                max_support: 0,
                ..OwnerLimits::default()
            },
        ),
        (
            "maxHistorySpan",
            OwnerLimits {
                max_history_span: 0,
                ..OwnerLimits::default()
            },
        ),
        (
            "maxEvaluationSteps",
            OwnerLimits {
                max_evaluation_steps: 1,
                ..OwnerLimits::default()
            },
        ),
        (
            "maxRecursionDepth",
            OwnerLimits {
                max_recursion_depth: 1,
                ..OwnerLimits::default()
            },
        ),
        (
            "maxVisitedFields",
            OwnerLimits {
                max_visited_fields: 1,
                ..OwnerLimits::default()
            },
        ),
    ] {
        assert_eq!(
            dispatch::consume_temporal(input, limits),
            request::derive(input, limits),
            "dispatch must forward {dimension} without reconstruction"
        );
    }

    let dispatch::Compatibility::Supported(supported) =
        dispatch::compatibility(dispatch::Contract::TemporalAssessment)
    else {
        panic!("temporal assessment contract must remain supported by TL");
    };
    assert_eq!(supported.contract(), dispatch::Contract::TemporalAssessment);
    assert_eq!(supported.contract_label(), request::CONTRACT);
    assert_eq!(
        supported.observation_revision(),
        quire_mltl::QUIRE_OBSERVATION_REVISION
    );
    for (contract, expected_label) in [
        (dispatch::Contract::RepairPlan, authority::repair::CONTRACT),
        (
            dispatch::Contract::ClosedPopulationQuery,
            authority::query::CONTRACT,
        ),
    ] {
        let dispatch::Compatibility::Unsupported(unsupported) = dispatch::compatibility(contract)
        else {
            panic!("QObs-owned contract must remain unsupported by TL");
        };
        assert_eq!(unsupported.contract(), contract);
        assert_eq!(unsupported.contract_label(), expected_label);
        assert_eq!(
            unsupported.observation_revision(),
            quire_mltl::QUIRE_OBSERVATION_REVISION
        );
    }

    assert_eq!(quire_mltl::QUIRE_OBSERVATION_REVISION, C00_REVISION);
    let manifest_entry = include_str!("../Cargo.toml")
        .lines()
        .find(|line| line.starts_with("quire-observation = "))
        .expect("manifest has one direct QObs dependency");
    assert_eq!(
        manifest_entry,
        format!(
            "quire-observation = {{ version = \"=0.1.0\", git = \
             \"https://github.com/agent-ix/quire-observation\", rev = \"{C00_REVISION}\" }}"
        )
    );
    let lock_entry = include_str!("../Cargo.lock")
        .split("[[package]]")
        .find(|entry| {
            entry
                .lines()
                .any(|line| line == "name = \"quire-observation\"")
        })
        .expect("lockfile has the QObs package");
    let lock_source = lock_entry
        .lines()
        .find(|line| line.starts_with("source = "))
        .expect("QObs lock entry has an exact source");
    assert_eq!(
        lock_source,
        format!(
            "source = \"git+https://github.com/agent-ix/quire-observation?rev={0}#{0}\"",
            C00_REVISION
        )
    );
}
