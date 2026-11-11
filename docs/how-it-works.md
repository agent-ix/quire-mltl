# How it works

## Three documents

The crate exchanges three canonical JSON documents. Each carries a `contractVersion` label and an `identity` computed from its own canonical bytes.

| Document | Module | Label |
| --- | --- | --- |
| Temporal assessment request | `request` | `quire-mltl.temporal-assessment-request/v1` |
| Temporal assessment result | `report` | `quire-mltl.temporal-assessment-result/v1` |
| Contract-IR result map | `contract_ir` | `quire-mltl.contract-ir-result-map/v1` |

`request::derive` writes a request from owner-read inputs. `report::evaluate` evaluates a validated request and writes a result. `contract_ir::map` maps a validated result to a value or a non-value. Each module also has a `read` function that takes bytes plus the exact inputs the document should have been derived from. `read` re-derives the document (except `report::read`, which binds the recorded result to its request and relation without evaluating) and refuses with an `OwnerReadError` on any difference, including a stale identity, a different contract label, a non-canonical encoding or a resource ceiling exceeded. The three schemas are checked in under [`schemas/`](../schemas/).

## Lanes

A request selects one of two evaluator lanes from its formula and input.

| Lane | Formula | Input | Evaluator |
| --- | --- | --- | --- |
| `future` | Formula document v1 with the `mltl.closed-trace/v1` or `mltl.online-prefix/v1` profile and no past operator | A validated trace | The tl-mltl future evaluator |
| `past` | Formula document v2 with the `mltl.origin-complete-history/v1` profile and no future operator | A validated origin-complete position history | The tl-mltl past evaluator |

A `closed-trace/v1` formula requires a closed trace. The past lane requires the anchor to be at or before the last history position. A formula with a mismatched profile, schema version or operator family is refused at admission.

## Checks in a result

A result keeps four independent facts: the execution disposition, the truth, the settlement basis and the observation assertions copied from the request. `execution` is `completed` when the tl-mltl evaluator returned a verdict. A closed trace settles as `closed-scope`. An open prefix that decides settles as `decisive-witness` or `decisive-counterexample`, and one that does not stays `pending` and `unsettled`. Decision support lists the observation identities of the available completeness facts, and only for a final result.

`contract_ir::map` reads those facts in a fixed order and the first row that applies decides the outcome:

1. An execution other than `completed` maps to the non-value with the same name.
2. Completeness `contradicted` or `incomplete` maps to the non-value of that name.
3. An availability state other than `available` maps to `unavailable`.
4. A result that is not final maps to `pending` when its truth is pending and to `unavailable` otherwise.
5. A final `satisfied` or `violated` result maps to `Value { value: true }` or `Value { value: false }`.

The mapping embeds the whole source result, so a consumer can join the two without loss.

## Where the crate sits

A formula passes through several crates on its way from text to a Quire outcome. quire-mltl is the last step.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="images/pipeline-dark.svg">
  <img alt="A formula moves from text to a Quire outcome. tl-parse turns MLTL text into a formula document, tl-rewrite returns a rewritten document, and tl-mltl evaluates it against a trace. In tests, tl-oracle evaluates the same formula and trace independently and the verdicts are compared. quire-mltl maps the result to a Boolean Value or a typed NonValue reason. All documents use the vocabulary of tl-syntax." src="images/pipeline-light.svg">
</picture>

The calling program moves documents from one stage to the next. The dependency structure between the crates is in the [README](../README.md#the-tl-stack).

## A worked example

quire-mltl is step 6 of a debugging session that asks why a deadline requirement failed. The requirement is `F[0,3] p2`, the recorded trace has four instants, `[[], [0, 1], [1], [1]]`, and `p2` (signal `s2`) is absent at every one of them. tl-mltl returns `false` at time 0 and R2U2 agrees. This crate takes that result and decides whether `false` can be reported as a value.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="images/scenario-sequence-dark.svg">
  <img alt="A deadline requirement fails on a recorded trace, and the engineer asks why. The engineer sends F[0,3] p2 to tl-parse and gets a formula document, binds p2 to signal s2 with tl-syntax, and sends the document to tl-rewrite, which reports Unchanged under the closed-trace profile. tl-mltl evaluates the formula and trace and returns false at time 0 with a buffer of 4 instants, and R2U2, which runs outside the crates, returns false at time 0 for the C2PO expression F[0,3](p2). tl-oracle evaluates the same formula and trace independently and returns false. quire-mltl maps the result to Value false. Numbered rings mark where to look at each step." src="images/scenario-sequence-light.svg">
</picture>

At this step, look at three things:

- The `truth` of the result. For this case it is `violated`, the execution is `completed`, and the result is final because the closed trace covers the whole window `[0,3]`.
- The completeness and availability views in the request. They are independent of the truth. A `violated` result with an `incomplete` completeness view maps to a non-value, and `false` is not produced.
- The `outcome` of the mapping. Only a final result with complete, available facts becomes `Value { value: false }`.

The crate's test `tc_084_future_and_past_requests_evaluate_and_strict_read` builds a request from the test fixtures in `tests/support`, evaluates it, reads the result back, and maps it. That test uses a one-proposition formula that is true, not this scenario's formula, so it ends in `Value { value: true }`. Running it:

```text
$ cargo test --test tc_084_temporal_owner_wire tc_084_future_and_past_requests_evaluate_and_strict_read
running 1 test
test tc_084_future_and_past_requests_evaluate_and_strict_read ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 9 filtered out; finished in 0.05s
```

The corpus case has no observation views, so the scenario cannot be replayed from the corpus alone. The same fixtures can be reused for it. A scratch test (not committed) that uses `tests/support` with four positions, a proposition map that binds proposition 2 to `s2`, the formula `F[0,3] p2` and the four-instant trace gave this output, for the `online-prefix` and the `closed-trace` profile alike:

```text
profile OnlinePrefixV1: truth Violated final true execution Completed
outcome Value { value: false }
incomplete: truth Violated outcome NonValue { reason: Incomplete }
```

The first two lines come from the complete, available views in the fixtures. The third line comes from the same request with the `incomplete` completeness view: the truth is still `violated`, and the mapped outcome is the non-value `Incomplete`. These views are test fixtures, not facts produced by quire-observation for the R2U2 case.

The test `tc_084_owner_axes_remain_independent_and_non_values_are_total` runs 192 combinations of progress, closure, completeness and availability views and checks the mapped outcome of each against the order of checks above. It passes on this checkout.

The full walkthrough, with the steps before and after this one, is in [tl-mltl's scenario page](https://github.com/agent-ix/tl-mltl/blob/main/docs/scenario.md).
