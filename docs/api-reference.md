# API reference

The crate has five public modules: `request`, `report`, `contract_ir`, `dispatch` and `census`. `OwnerLimits`, `OwnerReadError`, `OwnerReadErrorCode` and `OwnerUsage` come from `tl_mltl::wire`. Every fallible function returns `Result<_, OwnerReadError>`.

## request

| Item | Purpose |
| --- | --- |
| `CONTRACT`, `SCHEMA_BYTES` | The contract label and the checked-in JSON Schema bytes. |
| `RequestInput<'a>` | The owner-read inputs: `formula`, `proposition_map`, `input`, `clock`, `subject_identity`, `correspondence_identity`, `anchor` and `observations`. |
| `TemporalInput<'a>` | `Future(&ValidatedTrace)` or `Past(&ValidatedPositionHistory)`. |
| `ObservationInputs<'a>` | Six observation views: decision-scope progress and closure, surrounding-execution progress and closure, completeness and availability. |
| `derive(input, limits)` | Admits the inputs and returns a `TemporalRequestDocument`. |
| `read(bytes, expected, limits)` | Strict-reads bytes against the expected inputs and returns a `ValidatedTemporalRequest`. |
| `TemporalRequestDocument` | Canonical bytes. `identity()`, `bytes()`, `usage()`. |
| `ValidatedTemporalRequest` | Accessors for the lane, semantic profile, operator profile, formula, proposition map, input artifact, clock, evaluator, subject and correspondence identities, anchor, the four axis assertions, completeness, availability, bytes and usage. |
| `TemporalLane` | `Future` or `Past`. |
| `ArtifactReference` | Contract, identity, revision and SHA-256 digest of one referenced document. |
| `EvaluatorReference` | The evaluator identity. |
| `AxisReference` | One progress or closure assertion: artifact, authority identity, revision and digest, state, scope identity and population identity. |
| `CompletenessReference`, `CompletenessFactReference` | The completeness assertion and its fact population. |
| `AvailabilityReference` | The availability assertion with its required and available result identities. |

## report

| Item | Purpose |
| --- | --- |
| `CONTRACT`, `SCHEMA_BYTES` | The contract label and the checked-in JSON Schema bytes. |
| `evaluate(request, relation, limits)` | Evaluates a validated request and returns a `TemporalResultDocument`. |
| `read(bytes, request, relation, limits)` | Strict-reads the bytes into a `ValidatedTemporalResult` bound to the request and relation, under every owner ceiling, without running an evaluator. The recorded truth is carried, not re-derived. |
| `ResultRelationInput<'a>` | `Original`, `Superseding(&result)` or `Invalidating(&result)`. |
| `AssessmentExecution` | `Completed`, `ResourceIncomplete`, `Unsupported`, `Failed`, `Refused`. |
| `TemporalTruth` | `Satisfied`, `Violated`, `Pending`, `Unavailable`. |
| `SettlementBasis` | `ClosedScope`, `DecisiveWitness`, `DecisiveCounterexample`, `Unsettled`, `Unavailable`. |
| `ResultRelationKind` | `Original`, `Superseding`, `Invalidating`. |
| `TemporalResultDocument` | `identity()`, `revision()`, `bytes()`, `usage()`. |
| `ValidatedTemporalResult` | `identity`, `revision`, `request_identity`, `subject_identity`, `correspondence_identity`, `execution`, `truth`, `is_final`, `settlement`, `decision_support`, the assertion accessors, `relation_kind`, `direct_predecessor_identity`, `bytes`, `usage` and `request`. |

## contract_ir

| Item | Purpose |
| --- | --- |
| `CONTRACT`, `SCHEMA_BYTES` | The contract label and the checked-in JSON Schema bytes. |
| `MappingSelection::for_result(&result)` | Selects the exact result to map. |
| `map(result, selection, limits)` | Returns a `MappingDocument`. |
| `read(bytes, result, selection, limits)` | Re-derives the mapping and returns a `ValidatedMappedResult`. |
| `MappedOutcome` | `Value { value }` or `NonValue { reason }`. |
| `NonValueKind` | `Pending`, `Unavailable`, `Incomplete`, `Unsupported`, `Failed`, `Refused`, `Contradicted`, `ResourceIncomplete`. |
| `MappingDocument`, `ValidatedMappedResult` | `identity()`, `bytes()`, `usage()`. The validated type adds `outcome()`, `source()`, `source_result_identity()`, `subject_identity()` and `correspondence_identity()`. |

## dispatch

| Item | Purpose |
| --- | --- |
| `Contract` | `TemporalAssessment`, `RepairPlan`, `ClosedPopulationQuery`. `label()` returns the wire label. |
| `compatibility(contract)` | Returns `Compatibility::Supported` or `Compatibility::Unsupported` without inspecting an artifact. |
| `consume_temporal(input, limits)` | Forwards to `request::derive`. |
| `REPAIR_PLAN_CONTRACT`, `CLOSED_POPULATION_QUERY_CONTRACT` | The labels of the two contracts that quire-observation owns. |

## census

| Item | Purpose |
| --- | --- |
| `REGISTRY` | The closed registry, ordered by class token. |
| `RegistryEntry` | `token`, `dimensions` and `state`. |
| `ClassState` | `Applicable(ApplicableClass)`, `Excluded(ExcludedClass)` or `Blocked(BlockedClass)`. |
| `census()` | Returns a `CensusReport` with the total and the applicable, excluded and blocked tokens. |
| `applicable_fixture_ids()` | The sorted, deduplicated fixture names of the applicable classes. |
| `CounterpartContract` | A quire-contract-ir contract named by label, with the exchanged document's contract label. |
