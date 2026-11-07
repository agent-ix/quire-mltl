# Request, dispatch and report

## request

`request::derive(input, limits)` takes a `RequestInput` and writes one canonical request. The admission checks run in this order and the first failure is returned.

1. The subject and correspondence identities are each 1 to 256 bytes.
2. The formula and proposition map are re-encoded and strict-read, so both are canonical.
3. Every proposition the formula references is present in the proposition map.
4. The formula graph depth is computed.
5. The lane checks match the formula to the input, as described in [how-it-works.md](how-it-works.md#lanes). The past lane also requires the anchor to be at or before the last history position.
6. The formula nodes, trace or history positions, propositions and history span are within the limits.
7. The clock, the four axis assertions, completeness and availability agree with each other, as described in [quire-observation.md](quire-observation.md).

A request records the semantic profile, the operator profile, the evaluator identity (`tl-mltl.future-evaluator/v1` for the future lane), the anchor and the limits it was derived under. `request::read` rebuilds the request from the expected inputs and refuses if the supplied limits differ from the recorded ones.

## Limits

`OwnerLimits` comes from tl-mltl. `OwnerLimits::default()` equals the owner maximum, and any ceiling a caller sets is clamped to it.

| Limit | Owner maximum |
| --- | --- |
| `max_input_bytes`, `max_output_bytes` | 8 MiB |
| `max_depth` | 64 |
| `max_string_bytes` | 1 MiB |
| `max_formula_nodes`, `max_propositions`, `max_support` | 100,000 |
| `max_formula_depth` | 4,096 |
| `max_positions`, `max_visited_fields` | 1,000,000 |
| `max_history_span` | 100,000 |
| `max_evaluation_steps` | 4,000,000 |

A request or result that would exceed a limit is refused with `OwnerReadErrorCode::ResourceIncomplete`. During evaluation, a step, span or recursion ceiling produces the execution `resource-incomplete`, and so does a decision-support set larger than `max_support`.

## dispatch

`dispatch` reports which Quire observation contracts the crate consumes.

| Contract | Label | Compatibility |
| --- | --- | --- |
| `TemporalAssessment` | `quire-mltl.temporal-assessment-request/v1` | Supported |
| `RepairPlan` | `quire.observation.repair-plan/v1` | Unsupported |
| `ClosedPopulationQuery` | `quire.observation.closed-population-query/v1` | Unsupported |

`compatibility` answers without reading an artifact. `consume_temporal` forwards a temporal handoff to `request::derive` and returns the same result.

## report

`report::evaluate(request, relation, limits)` runs the tl-mltl evaluator that matches the request lane, then builds the result.

| Execution | Meaning |
| --- | --- |
| `completed` | The evaluator returned a verdict. |
| `resource-incomplete` | A step, span, recursion or support ceiling stopped evaluation. |
| `unsupported` | The profile, clock or operator is not supported. |
| `failed` | The evaluator failed after admission without a Boolean. |
| `refused` | The evaluator refused the runtime combination. |

For a completed future-lane evaluation on a closed trace, a true verdict is `satisfied` and a false verdict is `violated`, both with settlement `closed-scope`. On an open prefix a decided verdict settles as `decisive-witness` or `decisive-counterexample`, and an undecided one is `pending` with settlement `unsettled`. A past-lane verdict is `satisfied` or `violated`. A result without a truth has truth and settlement `unavailable`.

The result copies the request's artifact references and assertions and adds `final`, `decisionSupport`, the `relation` and a `usage` record of the work done.

The three execution outcomes `unsupported`, `failed` and `refused` are part of the wire contract. No request that passes admission reaches them, so the census registry lists them as excluded.

### Result lineage

`ResultRelationInput` states how a result relates to an earlier one. `Original` is the first result and has revision 1. `Superseding(&earlier)` and `Invalidating(&earlier)` take the complete earlier validated result. They require the new request to match the earlier one in lane, formula, proposition map, clock, evaluator, anchor, subject identity and correspondence identity, and refuse with `correctionContext` otherwise. They record the earlier result's identity and digest as the direct predecessor, record the new request identity as the corrected request, and give the new result the next revision. `report::read` re-evaluates with the same relation and refuses a result that was read against a different predecessor.

## census

The census registry classifies the combinations of lane, clock, completeness, availability, execution, mapped outcome, resource boundary and result relation that the correspondence contract needs. It has 41 classes: 34 applicable, 7 excluded and none blocked. Each applicable class names the test fixture that replays it.

The seven excluded classes are not reachable. Six are execution or mapped-outcome classes (`failed`, `refused` and `unsupported`) that request admission rules out. The seventh is a past request with no clock binding, which tl-mltl cannot construct. Each exclusion carries a reason code and the evidence for it.
