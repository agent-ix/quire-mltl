# Examples

The excerpts below are taken from the crate's tests, which pass against the current source. They omit the fixture setup in `tests/support/mod.rs`, which builds the owner views, the formula, the proposition map and the trace. The setup needs quire-observation's admission types, so the excerpts start from values the tests have already built.

## A future-lane request mapped to a value

This is from `tc_084_future_and_past_requests_evaluate_and_strict_read` in `tests/tc_084_temporal_owner_wire.rs`. `future_request` is a validated request for a closed trace. The excerpt evaluates it, reads the result back, maps it and reads the mapping back.

```rust
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
```

## A past-lane request

The same test builds a request over an origin-complete history. `RequestInput` takes a formula document, a proposition map, a `TemporalInput`, the clock view, the native subject identity, the correspondence identity, the anchor and the observation views.

```rust
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
```

Here `observations` is a helper in `tests/support/mod.rs` that returns an `ObservationInputs` value. The six fields of `ObservationInputs` are described in [quire-observation.md](quire-observation.md).

## A superseding result

A corrected input produces a second request. The new result names the first result as its direct predecessor and has revision 2. This is from `tc_087_result_relation_superseding` in `tests/tc_087_native_correspondence_census.rs`.

```rust
let superseding_document = report::evaluate(
    &second_request,
    report::ResultRelationInput::Superseding(&first_result),
    OwnerLimits::default(),
)
.unwrap();
let superseding = report::read(
    superseding_document.bytes(),
    &second_request,
    report::ResultRelationInput::Superseding(&first_result),
    OwnerLimits::default(),
)
.unwrap();
assert_eq!(
    superseding.relation_kind(),
    report::ResultRelationKind::Superseding
);
assert_eq!(superseding.revision(), 2);
```

## Ask which contracts the crate consumes

This is from `tc_085_qobs_c00_temporal_dispatch_and_unsupported_contracts_are_exact` in `tests/tc_084_temporal_owner_wire.rs`. Temporal assessment is supported. The repair-plan and closed-population-query contracts belong to quire-observation, and `dispatch::compatibility` reports them as unsupported with their exact labels.

```rust
let dispatch::Compatibility::Supported(supported) =
    dispatch::compatibility(dispatch::Contract::TemporalAssessment)
else {
    panic!("temporal assessment contract must remain supported by TL");
};
assert_eq!(supported.contract_label(), request::CONTRACT);

let dispatch::Compatibility::Unsupported(unsupported) =
    dispatch::compatibility(dispatch::Contract::RepairPlan)
else {
    panic!("QObs-owned contract must remain unsupported by TL");
};
assert_eq!(unsupported.contract_label(), authority::repair::CONTRACT);
```

## Count the census classes

`census::census()` returns the population of the native-correspondence registry without running any fixture. The tests in `tests/tc_087_native_correspondence_census.rs` replay one fixture for each applicable class.

```rust
let report = quire_mltl::census::census();
assert_eq!(
    report.applicable.len() + report.excluded.len() + report.blocked.len(),
    report.total
);
```

This assertion is from `census_population_partitions_the_registry_and_never_says_not_implemented` in `src/census.rs`.
