//! Shared fixture-construction helpers for this crate's integration tests.
//!
//! Factored out of `tc_084_temporal_owner_wire.rs` (TL-178) so the FR-004
//! (`tc_086`) and FR-005 (`tc_087`) test modules can reuse the exact same
//! owner-admission, clock, and formula/trace/history construction paths
//! rather than re-deriving them. Every item here is `pub(crate)`: visible to
//! every integration-test binary that does `mod support;`, never part of
//! this crate's own public API.
//!
//! `cargo` compiles this module fresh, as its own crate, inside every
//! integration-test binary that includes it (`tc_084`, `tc_086`, `tc_087`).
//! Dead-code analysis runs per binary, so a helper only `tc_084` calls looks
//! unused from `tc_086`'s copy and vice versa -- there is no single crate
//! boundary across which `cargo`/`clippy` could see the union of callers.
//! `#![allow(dead_code)]` here is the standard shape for a shared
//! test-support module split across independent integration-test binaries;
//! it does not suppress dead code within any one binary's own real code.

#![allow(dead_code)]

use std::collections::{BTreeMap, BTreeSet};

use agent_ix_baseline_producer::{
    configuration_digest, AdmittedStaticBundle, ArtifactKind, ArtifactReference,
    ConfigurationDocument, DeclarationSource, DigestDomainSelection, DigestSelection,
    FormalDocument, InventoryCompleteness, InventoryDeclaration, ModelSelection, NativeSourceLabel,
    NumericResourceLimit, ProfileSelection, RawByteDigest,
    ResourceLimits as ProducerResourceLimits, Revision as ProducerRevision, StaticClosure,
    StaticProducerBundle, WireReference, BASELINE_VERSION, INTERFACE_VERSION,
    PRODUCER_REVISION_NAMESPACE,
};
use quire_mltl::request;
use quire_observation::authority::{
    self, AuthoritySelection, Context, History, Limits as ObservationLimits, OpenClosed,
    SubjectSelection, TemporalBoundary,
};
use quire_observation::{
    admit, AdmissionOutcome, AdmissionRequest, AdmittedRecord, Anchor, ClockRange, Digest,
    Identity, Member, ObservationBinding, PackageSelection, QualifiedObservation, QualifiedSubject,
    ResourceLimits, ScopeKind, ScopeSelection, SubjectIdentity, SubjectKind, ValueState,
    Visibility, NATIVE_LINKED_PACKAGE_FORMAT,
};
use serde_json::Value;
use sha2::{Digest as _, Sha256};
use tl_mltl::wire::{trace, OwnerLimits as TlOwnerLimits};
use tl_mltl::{
    ClockBinding, ExactNumber, PositionHistoryDocument, PositionObservation, TraceDocument,
    TraceSchemaVersion,
};
use tl_syntax::{
    FormulaDocument, Interval, Node, NodeId, NodeKind, PropositionEntry, PropositionId,
    PropositionMapDocument, SemanticProfile,
};

pub(crate) struct OwnerViews {
    pub(crate) clock: authority::clock::View,
    pub(crate) progress_open: authority::progress::View,
    pub(crate) progress_closed: authority::progress::View,
    pub(crate) closure_open: authority::closure::View,
    pub(crate) closure_closed: authority::closure::View,
    pub(crate) completeness_complete: authority::completeness::View,
    pub(crate) completeness_incomplete: authority::completeness::View,
    pub(crate) completeness_contradicted: authority::completeness::View,
    pub(crate) availability_available: authority::availability::View,
    pub(crate) availability_not_yet: authority::availability::View,
    pub(crate) availability_producer_unavailable: authority::availability::View,
    pub(crate) availability_contract_unavailable: authority::availability::View,
}

#[derive(Clone, Copy)]
pub(crate) enum FixtureClock {
    EventPosition,
    FixedSample,
    /// Half-open timestamped-event range. `quire-mltl` never admits this
    /// clock family (FR-004-AC-5): it exists solely so `tc_087` can build a
    /// read-time-refused `clockFamily` fixture through the same owner
    /// admission path every other clock family uses.
    TimestampedEvent,
}

impl FixtureClock {
    pub(crate) const fn identity(self) -> &'static str {
        match self {
            Self::EventPosition => "clock:event-position",
            Self::FixedSample => "clock:fixed-sample",
            Self::TimestampedEvent => "clock:timestamped-event",
        }
    }

    pub(crate) const fn range(self, positions: u64) -> ClockRange {
        match self {
            Self::EventPosition => ClockRange::EventPosition {
                start: 0,
                end_exclusive: positions,
            },
            Self::FixedSample => ClockRange::FixedSample {
                start: 0,
                end_exclusive: positions,
                epoch_nanos: 100,
                period_nanos: 10,
            },
            Self::TimestampedEvent => ClockRange::Timestamp {
                start_nanos: 0,
                end_nanos: positions as i128 * 10,
            },
        }
    }

    pub(crate) const fn anchor(self) -> Anchor {
        match self {
            Self::EventPosition => Anchor::EventPosition(0),
            Self::FixedSample => Anchor::FixedSample {
                index: 0,
                epoch_nanos: 100,
                period_nanos: 10,
            },
            Self::TimestampedEvent => Anchor::TimestampNanos(0),
        }
    }

    pub(crate) fn boundary(self, state: OpenClosed, positions: u64) -> TemporalBoundary {
        let watermark = if state == OpenClosed::Closed {
            positions
        } else {
            positions - 1
        };
        match self {
            Self::EventPosition => TemporalBoundary::EventPosition {
                lower: 0,
                upper_inclusive: positions - 1,
                carrier_end_exclusive: positions,
                watermark,
            },
            Self::FixedSample => TemporalBoundary::FixedSample {
                lower: 0,
                upper_inclusive: positions - 1,
                carrier_end_exclusive: positions,
                watermark,
            },
            Self::TimestampedEvent => TemporalBoundary::TimestampedEvent {
                lower_nanos: 0,
                upper_inclusive_nanos: (positions as i128 - 1) * 10,
                carrier_end_exclusive_nanos: positions as i128 * 10,
                watermark_nanos: watermark as i128 * 10,
            },
        }
    }
}

pub(crate) fn id(value: impl Into<String>) -> Identity {
    Identity::new(value)
}

pub(crate) fn digest(value: u8) -> Digest {
    Digest::new([value; 32])
}

/// Builds one admitted producer-interface-1.2 static bundle from the compiled
/// `agent-ix-baseline-producer` dependency's own public types. See
/// `tc_084_temporal_owner_wire.rs`'s original doc comment (TL-178) for why
/// this crate builds one directly rather than reading a vendored fixture.
pub(crate) fn minimal_producer_bundle(tag: &str) -> AdmittedStaticBundle {
    let dummy_digest = format!("sha256:{}", "0".repeat(64));

    let mut configuration = ConfigurationDocument {
        configuration_identity: format!("configuration:{tag}"),
        baseline_version: BASELINE_VERSION.to_owned(),
        digest: DigestSelection::canonical(dummy_digest.clone()),
        model_authority: format!("authority:{tag}"),
        profile_identities: BTreeSet::new(),
        adapter_identities: BTreeSet::new(),
        mapping_targets: BTreeSet::new(),
        loss_policy: format!("loss:{tag}"),
        resource_limits: ProducerResourceLimits {
            numeric_resource_limit: Some(NumericResourceLimit::new(64, 32)),
            declared_bounds: BTreeMap::new(),
        },
        digest_selections: DigestDomainSelection::baseline(),
        revision_namespaces: [PRODUCER_REVISION_NAMESPACE.to_owned()]
            .into_iter()
            .collect(),
        trusted_references: BTreeSet::new(),
    };
    configuration.digest = configuration_digest(&configuration).unwrap();

    let model = ModelSelection {
        model_identity: format!("model:{tag}"),
        model_revision: ProducerRevision::producer("1"),
        digest: DigestSelection::canonical(dummy_digest.clone()),
    };
    let profile = ProfileSelection {
        profile_identity: format!("profile:{tag}"),
        profile_revision: ProducerRevision::producer("1"),
        digest: DigestSelection::canonical(dummy_digest.clone()),
    };
    let inventory = InventoryDeclaration {
        inventory_identity: format!("inventory:{tag}"),
        completeness: InventoryCompleteness::Complete,
        component_identities: BTreeSet::new(),
        endpoint_identities: BTreeSet::new(),
        relationship_identities: BTreeSet::new(),
    };
    let static_closure = StaticClosure {
        configuration_identity: configuration.configuration_identity.clone(),
        configuration_digest: configuration.digest.clone(),
        model_identity: model.model_identity.clone(),
        model_digest: model.digest.clone(),
        profile_identity: profile.profile_identity.clone(),
        profile_digest: profile.digest.clone(),
        declaration_sources: vec![DeclarationSource {
            source: ArtifactReference {
                ref_version: "1".to_owned(),
                kind: ArtifactKind::Source,
                authority: format!("authority:{tag}"),
                identity: format!("source:{tag}"),
                revision: ProducerRevision::producer("1"),
                digest: RawByteDigest::new(dummy_digest.clone()).unwrap(),
                wire: WireReference {
                    identity: format!("wire:{tag}"),
                    version: "1".to_owned(),
                },
            },
            native: NativeSourceLabel::new(format!("native:{tag}"), "label-1"),
            path: format!("{tag}.md"),
            formal: FormalDocument {
                document: format!("formal:{tag}"),
                revision: ProducerRevision::producer("1"),
            },
        }],
    };

    let mut bundle = StaticProducerBundle {
        bundle_identity: Some(format!("bundle:{tag}")),
        bundle_revision: Some(ProducerRevision::producer("1")),
        digest: None,
        interface_version: Some(INTERFACE_VERSION.to_owned()),
        model: Some(model),
        profile: Some(profile),
        components: Vec::new(),
        endpoints: Vec::new(),
        relationships: Vec::new(),
        inventory: Some(inventory),
        configuration: Some(configuration),
        static_closure: Some(static_closure),
        correspondences: Vec::new(),
    };
    bundle.digest = Some(bundle.canonical_digest_selection().unwrap());
    bundle
        .admit()
        .expect("the minimal shared producer bundle is admitted")
}

pub(crate) fn qualified_with_clock(
    tag: &str,
    positions: u64,
    fixture_clock: FixtureClock,
) -> Box<QualifiedObservation> {
    let producer = minimal_producer_bundle(tag);
    let subject = QualifiedSubject::new(
        &producer,
        SubjectKind::new("ix://agent-ix/commerce/type/Order").unwrap(),
        SubjectIdentity::new(format!("order:{tag}")).unwrap(),
    );
    let mut request = AdmissionRequest {
        package: PackageSelection {
            format: NATIVE_LINKED_PACKAGE_FORMAT.to_owned(),
            identity: id(format!("package:{tag}")),
            revision: id("1"),
            digest: digest(1),
        },
        producer,
        binding: ObservationBinding {
            identity: id(format!("binding:{tag}")),
            source_identity: id(format!("source:{tag}")),
            schema_identity: id(format!("schema:{tag}")),
            signal_identity: id("signal:p0"),
            trigger_identity: id(format!("trigger:{tag}")),
            unit: id("boolean"),
            subject_kind: subject.kind().clone(),
            required: true,
        },
        expected_subject: subject.clone(),
        relationships: vec![],
        required_relationships: vec![],
        scope: ScopeSelection {
            population_identity: id("unsealed-population"),
            membership_rule_identity: id(format!("membership:{tag}")),
            membership_digest: digest(4),
            membership_document: vec![],
            required_member_identities: vec![id(format!("member:{tag}"))],
            observation_sources: vec![id(format!("source:{tag}"))],
            completeness_dependencies: vec![id(format!("completeness:{tag}"))],
            progress_dependencies: vec![id(format!("progress:{tag}"))],
            clock_identity: id(fixture_clock.identity()),
            clock_revision: id("1"),
            membership_complete: true,
            closure_identity: Some(id(format!("closure:{tag}"))),
            closure_digest: Some(digest(5)),
            kind: ScopeKind::Snapshot {
                snapshot_identity: id(format!("snapshot:{tag}")),
            },
            range: fixture_clock.range(positions),
            members: vec![Member {
                object_identity: id(format!("member:{tag}")),
                record_identity: id("unsealed-record"),
                anchor: fixture_clock.anchor(),
            }],
        },
        records: vec![AdmittedRecord {
            identity: id("unsealed-record"),
            binding_identity: id(format!("binding:{tag}")),
            source_identity: id(format!("source:{tag}")),
            schema_identity: id(format!("schema:{tag}")),
            subject,
            signal_identity: id("signal:p0"),
            trigger_identity: id(format!("trigger:{tag}")),
            unit: id("boolean"),
            value: ValueState::Present {
                value_type: id("boolean"),
                canonical_value: "true".to_owned(),
            },
            visibility: Visibility::External,
            anchor: fixture_clock.anchor(),
            event_time_nanos: 0,
            ingestion_time_nanos: 1,
            causal_relationship_identity: None,
            clock_identity: id(fixture_clock.identity()),
            clock_revision: id("1"),
            clock_uncertainty_nanos: 0,
        }],
        limits: ResourceLimits {
            max_records: 1,
            max_members: 1,
            max_relationships: 0,
            max_required_relationships: 0,
        },
    };
    let record_identity = authority::observation::record_identity(
        &request.records[0],
        ObservationLimits::owner_max(),
    )
    .unwrap();
    request.records[0].identity = record_identity.clone();
    request.scope.members[0].record_identity = record_identity;
    authority::population::assign_request_identities(&mut request, ObservationLimits::owner_max())
        .unwrap();
    match admit(request) {
        AdmissionOutcome::Available { observation } => observation,
        other => panic!("observation admission failed: {other:?}"),
    }
}

pub(crate) fn owner_views(tag: &str, positions: u64) -> OwnerViews {
    owner_views_with_clock(tag, positions, FixtureClock::EventPosition)
}

pub(crate) fn owner_views_with_clock(
    tag: &str,
    positions: u64,
    fixture_clock: FixtureClock,
) -> OwnerViews {
    let qualified = qualified_with_clock(tag, positions, fixture_clock);
    let owner = AuthoritySelection {
        definition_identity: id(format!("definition:{tag}")),
        definition_revision: id("1"),
        definition_digest: digest(9),
    };
    let subject = SubjectSelection {
        scope_identity: id(format!("snapshot:{tag}")),
        population_identity: qualified.scope().population_identity.clone(),
    };
    let context = Context::new(History::batch(&qualified), &owner, &subject, 1, None);
    let clock_selection = authority::clock::Selection::new(id(fixture_clock.identity()), id("1"));
    let clock_document =
        authority::clock::derive(context, &clock_selection, ObservationLimits::owner_max())
            .unwrap();
    let clock = authority::clock::read(
        clock_document.bytes(),
        context,
        &clock_selection,
        ObservationLimits::owner_max(),
    )
    .unwrap();

    let progress = |state| {
        let selection = authority::progress::Selection::new(
            clock_selection.clone(),
            vec![id(format!("source:{tag}"))],
            fixture_clock.boundary(state, positions),
            state,
            id(format!("trigger:{tag}")),
            cutoff(fixture_clock),
            id(format!("restoration:{tag}")),
        );
        let document =
            authority::progress::derive(context, &selection, ObservationLimits::owner_max())
                .unwrap();
        authority::progress::read(
            document.bytes(),
            context,
            &selection,
            ObservationLimits::owner_max(),
        )
        .unwrap()
    };
    let closure = |state| {
        let selection = authority::closure::Selection::new(
            id(fixture_clock.identity()),
            id("1"),
            vec![id(format!("source:{tag}"))],
            fixture_clock.boundary(state, positions),
            state,
        );
        let document =
            authority::closure::derive(context, &selection, ObservationLimits::owner_max())
                .unwrap();
        authority::closure::read(
            document.bytes(),
            context,
            &selection,
            ObservationLimits::owner_max(),
        )
        .unwrap()
    };
    let completeness = |status: authority::completeness::FactStatus, observed: bool| {
        let selection = authority::completeness::Selection::new(
            id(format!("boundary:{tag}")),
            vec![authority::completeness::Fact::new(
                id(format!("member:{tag}")),
                observed.then(|| qualified.records()[0].identity.clone()),
                status,
            )],
        );
        let document =
            authority::completeness::derive(context, &selection, ObservationLimits::owner_max())
                .unwrap();
        authority::completeness::read(
            document.bytes(),
            context,
            &selection,
            ObservationLimits::owner_max(),
        )
        .unwrap()
    };
    let availability = |available: bool,
                        producer: authority::availability::DependencyState,
                        contract: authority::availability::DependencyState| {
        let required = vec![id(format!("required-result:{tag}"))];
        let selection = authority::availability::Selection::new(
            required.clone(),
            if available { required } else { Vec::new() },
            producer,
            contract,
        );
        let document =
            authority::availability::derive(context, &selection, ObservationLimits::owner_max())
                .unwrap();
        authority::availability::read(
            document.bytes(),
            context,
            &selection,
            ObservationLimits::owner_max(),
        )
        .unwrap()
    };

    OwnerViews {
        clock,
        progress_open: progress(OpenClosed::Open),
        progress_closed: progress(OpenClosed::Closed),
        closure_open: closure(OpenClosed::Open),
        closure_closed: closure(OpenClosed::Closed),
        completeness_complete: completeness(authority::completeness::FactStatus::Available, true),
        completeness_incomplete: completeness(
            authority::completeness::FactStatus::Incomplete,
            false,
        ),
        completeness_contradicted: completeness(
            authority::completeness::FactStatus::Contradicted,
            true,
        ),
        availability_available: availability(
            true,
            authority::availability::DependencyState::Available,
            authority::availability::DependencyState::Available,
        ),
        availability_not_yet: availability(
            false,
            authority::availability::DependencyState::Available,
            authority::availability::DependencyState::Available,
        ),
        availability_producer_unavailable: availability(
            false,
            authority::availability::DependencyState::Unavailable,
            authority::availability::DependencyState::Available,
        ),
        availability_contract_unavailable: availability(
            false,
            authority::availability::DependencyState::Available,
            authority::availability::DependencyState::Unavailable,
        ),
    }
}

pub(crate) fn cutoff(fixture_clock: FixtureClock) -> authority::observation::CutoffSelection {
    authority::observation::CutoffSelection::new(
        id(fixture_clock.identity()),
        id("1"),
        2,
        authority::observation::CutoffRule::IngestionTimeAtOrBefore,
        id("1"),
    )
}

pub(crate) fn proposition_map() -> PropositionMapDocument {
    PropositionMapDocument::new(vec![PropositionEntry {
        id: PropositionId(0),
        name: "p0".to_owned(),
    }])
    .unwrap()
}

pub(crate) fn future_formula(profile: SemanticProfile) -> FormulaDocument {
    FormulaDocument::new(
        profile,
        NodeId(0),
        vec![Node::new(NodeKind::Proposition {
            proposition: PropositionId(0),
        })],
    )
    .unwrap()
}

pub(crate) fn past_formula() -> FormulaDocument {
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
    .unwrap()
}

pub(crate) fn trace_document(id: &str, closed: bool) -> TraceDocument {
    TraceDocument {
        schema_version: TraceSchemaVersion::V1,
        trace_id: id.to_owned(),
        closed,
        instants: vec![vec![PropositionId(0)], vec![]],
    }
}

pub(crate) fn history_document(
    id: &str,
    revision: u64,
    values: [bool; 2],
) -> PositionHistoryDocument {
    PositionHistoryDocument::new(
        id,
        revision,
        0,
        1,
        Some(ClockBinding::EventPosition),
        values
            .into_iter()
            .enumerate()
            .map(|(position, value)| {
                PositionObservation::new(
                    u64::try_from(position).unwrap(),
                    value.then_some(PropositionId(0)).into_iter().collect(),
                    None,
                )
            })
            .collect(),
    )
    .unwrap()
}

pub(crate) fn fixed_history_document(id: &str) -> PositionHistoryDocument {
    let epoch = ExactNumber::new(100, 1).unwrap();
    let period = ExactNumber::new(10, 1).unwrap();
    PositionHistoryDocument::new(
        id,
        1,
        0,
        1,
        Some(ClockBinding::FixedSample {
            epoch,
            period,
            unit: "nanoseconds".to_owned(),
        }),
        [true, false]
            .into_iter()
            .enumerate()
            .map(|(position, value)| {
                let position = u64::try_from(position).unwrap();
                PositionObservation::new(
                    position,
                    value.then_some(PropositionId(0)).into_iter().collect(),
                    Some(tl_mltl::ClockSample {
                        instant: tl_mltl::fixed_sample_instant(epoch, period, position).unwrap(),
                        unit: "nanoseconds".to_owned(),
                    }),
                )
            })
            .collect(),
    )
    .unwrap()
}

pub(crate) fn observations<'a>(
    decision: &'a OwnerViews,
    surrounding: &'a OwnerViews,
    decision_progress: bool,
    decision_closure: bool,
    surrounding_progress: bool,
    surrounding_closure: bool,
    completeness: &'a authority::completeness::View,
    availability: &'a authority::availability::View,
) -> request::ObservationInputs<'a> {
    request::ObservationInputs {
        decision_scope_progress: if decision_progress {
            &decision.progress_closed
        } else {
            &decision.progress_open
        },
        decision_scope_closure: if decision_closure {
            &decision.closure_closed
        } else {
            &decision.closure_open
        },
        surrounding_execution_progress: if surrounding_progress {
            &surrounding.progress_closed
        } else {
            &surrounding.progress_open
        },
        surrounding_execution_closure: if surrounding_closure {
            &surrounding.closure_closed
        } else {
            &surrounding.closure_open
        },
        completeness,
        availability,
    }
}

pub(crate) fn future_request_input<'a>(
    formula: &'a FormulaDocument,
    propositions: &'a PropositionMapDocument,
    trace: &'a trace::ValidatedTrace,
    decision: &'a OwnerViews,
    _surrounding: &'a OwnerViews,
    observation_inputs: request::ObservationInputs<'a>,
) -> request::RequestInput<'a> {
    request::RequestInput {
        formula,
        proposition_map: propositions,
        input: request::TemporalInput::Future(trace),
        clock: &decision.clock,
        subject_identity: "native-subject:order-1",
        correspondence_identity: "native-tl-correspondence:order-1",
        anchor: 0,
        observations: observation_inputs,
    }
}

pub(crate) fn admit_request<'a>(
    input: request::RequestInput<'a>,
    limits: TlOwnerLimits,
) -> request::ValidatedTemporalRequest {
    let document = request::derive(input, limits).unwrap();
    request::read(document.bytes(), input, limits).unwrap()
}

pub(crate) fn admit_history(
    history: &PositionHistoryDocument,
) -> tl_mltl::past::history::ValidatedPositionHistory {
    let document = tl_mltl::past::history::derive(history, TlOwnerLimits::default()).unwrap();
    tl_mltl::past::history::read(document.bytes(), history, TlOwnerLimits::default()).unwrap()
}

pub(crate) fn sha256(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

pub(crate) fn exact_wire_identity(contract: &str, bytes: &[u8], identity: &str) -> String {
    let member = format!(",\"identity\":\"{identity}\"");
    let position = bytes
        .windows(member.len())
        .position(|window| window == member.as_bytes())
        .unwrap();
    let mut preimage = Vec::with_capacity(bytes.len() - member.len());
    preimage.extend_from_slice(&bytes[..position]);
    preimage.extend_from_slice(&bytes[position + member.len()..]);
    let mut digest = Sha256::new();
    digest.update(contract.as_bytes());
    digest.update([0]);
    digest.update(preimage);
    digest
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

pub(crate) fn assert_closed_schema(value: &Value) {
    match value {
        Value::Object(object) => {
            if object.get("type") == Some(&Value::String("object".to_owned())) {
                assert_eq!(
                    object.get("additionalProperties"),
                    Some(&Value::Bool(false))
                );
            }
            if let Some(reference) = object.get("$ref").and_then(Value::as_str) {
                assert!(reference.starts_with("#/$defs/"));
            }
            for child in object.values() {
                assert_closed_schema(child);
            }
        }
        Value::Array(values) => {
            for child in values {
                assert_closed_schema(child);
            }
        }
        _ => {}
    }
}

pub(crate) fn structural_mutations(
    bytes: &[u8],
    version_field: &str,
    version: &str,
    required_field: &str,
) -> Vec<Vec<u8>> {
    let mut unknown: Value = serde_json::from_slice(bytes).unwrap();
    unknown["unknown"] = Value::Bool(true);
    let mut missing: Value = serde_json::from_slice(bytes).unwrap();
    missing.as_object_mut().unwrap().remove(required_field);
    let reordered: Value = serde_json::from_slice(bytes).unwrap();
    let duplicate = String::from_utf8(bytes.to_vec())
        .unwrap()
        .replacen('{', &format!("{{\"{version_field}\":\"{version}\","), 1)
        .into_bytes();
    let mut trailing = bytes.to_vec();
    trailing.extend_from_slice(b"\nnull");
    vec![
        serde_json::to_vec(&unknown).unwrap(),
        serde_json::to_vec(&missing).unwrap(),
        serde_json::to_vec(&reordered).unwrap(),
        duplicate,
        trailing,
    ]
}
