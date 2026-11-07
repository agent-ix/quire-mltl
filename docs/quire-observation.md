# Observation views

quire-mltl does not produce observation facts. It consumes six views that [quire-observation](https://github.com/agent-ix/quire-observation) reports, together with a clock view, and records a reference to each in the request.

| View | Request field | What it asserts |
| --- | --- | --- |
| Clock | `clock`, `clockIdentity`, `clockRevision` | The range of positions the evidence covers, as an event-position or fixed-sample selection. |
| Decision-scope progress | `decisionScopeProgress` | The open or closed progress state of the decision scope. |
| Decision-scope closure | `decisionScopeClosure` | The open or closed closure state of the decision scope. |
| Surrounding-execution progress | `surroundingExecutionProgress` | The same assertion for the execution around the decision scope. |
| Surrounding-execution closure | `surroundingExecutionClosure` | The same assertion for the surrounding execution. |
| Completeness | `completeness` | Whether the required fact population is `complete`, `incomplete` or `contradicted`, with one status per required member. |
| Availability | `availability` | Whether the required results are available, with the required and available result identities. |

Each reference is an `ArtifactReference` (contract, identity, revision and SHA-256 digest of the exact bytes), and the axis references add the authority identity, revision and digest, the scope identity, the population identity and the open or closed state.

## Cross-checks

`request::derive` refuses the inputs unless the views belong together.

- Each progress and closure pair has the same subject, authority and scope identity, and both refer to the request's clock identity and revision.
- The clock, the completeness view and the availability view have the same subject and authority as the decision-scope progress view.
- The completeness population identity equals the decision-scope population identity.
- The clock selection is an event-position or fixed-sample range, and its length equals the number of trace or history positions. A timestamped-event selection is refused.
- For a past request, the history's clock binding agrees with the clock view. An event-position binding matches an event-position selection. A fixed-sample binding matches a fixed-sample selection that has the same epoch and period in nanoseconds.

A failure is reported as an `OwnerReadError` with code `ExpectedMismatch` and the field name, for example `decisionEvidenceContext`, `clockRange`, `clockFamily` or `clockBinding`.

## Effect on the outcome

The observation states are independent of the truth. The result can say `satisfied` while completeness is `incomplete`, and `contract_ir::map` then returns `NonValue { reason: incomplete }`. [how-it-works.md](how-it-works.md#checks-in-a-result) lists the order in which the states are checked.
