# quire-mltl documentation

quire-mltl evaluates an MLTL formula against a recorded trace or history and reports a Boolean value or a typed non-value. It is the one TL crate that depends on Quire. This directory documents the crate's modules and wire contracts. The [README](../README.md) has the overview and the smallest example.

| Page | What it covers |
| --- | --- |
| [how-it-works.md](how-it-works.md) | The request, result and mapping documents, the order of checks, the two evaluation lanes, and where the crate sits in the pipeline from text to a Quire outcome. |
| [examples.md](examples.md) | Excerpts from the test suite: a future-lane request mapped to a value, a past-lane request, a superseding result and a dispatch query. |
| [api-reference.md](api-reference.md) | Public items by module. |
| [request-dispatch-report.md](request-dispatch-report.md) | The request, dispatch and report modules: admission checks, limits, execution outcomes, settlement and result lineage. |
| [quire-observation.md](quire-observation.md) | The observation views the crate consumes and the checks that tie them together. |

## Scope

The crate admits inputs, evaluates them through tl-mltl and maps the result. It does not parse formulas (tl-parse), rewrite them (tl-rewrite), implement temporal semantics (tl-mltl) or produce observation facts (quire-observation). tl-oracle is an independent reference evaluator that the other crates use in tests. The public API is not stable and the crate has `publish = false`.
