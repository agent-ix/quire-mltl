---
id: ADR-002
title: "quire-mltl owns the native-correspondence dimension, and owns it as the supplier"
type: ADR
status: accepted
owner: kreneskyp
relationships:
  - target: ix://agent-ix/quire-mltl/MRS-001
    type: relates_to
  - target: ix://agent-ix/quire-mltl/ADR-001
    type: relates_to
  - target: ix://agent-ix/quire-contract-ir/FR-025
    type: relates_to
  - target: ix://agent-ix/quire-contract-ir/FR-026
    type: relates_to
---
# ADR-002: quire-mltl owns the native-correspondence dimension, and owns it as the supplier

## Status

**Accepted** — owner decision recorded 2026-09-22 against Linear epic
[TL-175](https://linear.app/agent-ix/issue/TL-175), extending
[ADR-001](./ADR-001-tl-crates-stay-quire-independent-quire-mltl-bridges.md) to
the one part of the boundary that had been specified in the wrong repository.

## Context

`tl-mltl`'s M4 corpus campaign (`agent-ix/tl-mltl` issue #38, PR #44, never
merged) declared a native-predicate/native-temporal correspondence coverage
dimension owned by `tl-mltl`. Its plan task read:

```yaml
owner_repository: agent-ix/tl-mltl
producer_repository: agent-ix/quire-contract-ir
```

and described the work as "consume accepted native predicate/temporal owner
families and move their exact campaign rows from blocked to applicable through
a successor manifest", blocked on `quire-contract-ir` #63 and #64.

Two things are wrong with that, not one.

**First, the repository.** Under ADR-001 every TL-* crate stays independent of
the agent-ix/Quire ecosystem, and only `quire-mltl` may bridge one to Quire
types. A `tl-mltl` campaign lane whose producer is `quire-contract-ir` is
exactly the coupling ADR-001 removed, re-entering through the specification
rather than through `Cargo.toml`. `tl-mltl` is dropping the same lane in its
own ADR-002 (see [agent-ix/tl-mltl#86](https://github.com/agent-ix/tl-mltl/pull/86),
pending merge as of 2026-09-21); the dimension has to live somewhere, and this
crate is the only place the ruling allows.

**Second, the direction.** The task cast `tl-mltl` as the *consumer* of a
`quire-contract-ir` producer. Measured against `quire-contract-ir`'s own
specification, that is backwards on the Quire side too.
`quire-contract-ir` FR-026 states that "Contract IR owns correspondence,
construction and the agreement decision", that its `temporal::project`
constructs "the exact TL artifacts for that profile" — formula, semantic,
history/trace, **request**, evaluator-report and selected-result-mapping — and
that it "invokes neither evaluator". Its `temporal::join` consumes a
`TlMappedResultView`. Under ADR-001, TL-181 repoints exactly those imports
from `tl_mltl::wire::*` / `tl_mltl::mapping::contract_ir::*` to
`quire_mltl::*`.

So `quire-contract-ir` is this crate's **consumer**. `quire-mltl` supplies the
TL-side request, result and Contract-IR mapping documents that FR-026
constructs, reads and joins. There is no producer for `quire-mltl` to consume
here, and a `quire-mltl -> quire-contract-ir` crate dependency would close a
cycle.

## Decision

The native-correspondence dimension is specified in `quire-mltl`, as a
**supplier-side obligation** on the three contracts this crate already owns:

- [FR-004](../requirements/FR-004-preserve-native-correspondence-identity.md)
  requires that the native subject and correspondence identities a request
  carries are validated, bound into the request's content identity, and carried
  verbatim into the result and the Contract-IR mapping — so a correspondence
  cannot be dropped, re-keyed onto a different assessment, or reinterpreted.
  It also requires that a correspondence class this crate cannot represent
  surfaces as a typed non-value the consumer can record, never as a Boolean,
  and that the three contracts cannot change shape without a successor identity
  that the consumer's expectation can detect.
- [FR-005](../requirements/FR-005-census-native-correspondence-classes.md)
  requires a closed, reviewable census of which native-correspondence classes
  the bridge carries losslessly, one canonical fixture and independent expected
  outcome per applicable class, exact-digest replay against the accepted
  `quire-contract-ir` FR-025/FR-026 contract revisions, and blocked classes
  that stay visible and out of the denominator.

`quire-mltl` adds no native grammar, predicate semantics, correspondence
derivation, clock/history correspondence, or agreement decision. Those stay
`quire-contract-ir`'s under FR-025/FR-026 and `quire-spec-language`'s under
the native source contracts.

## Why this needs no new dependency, and no copied vocabulary

The native correspondence reaches this crate as *data inside a request it
already strict-reads*. `RequestInput` already carries `subject_identity` and
`correspondence_identity`, and `RequestWire` already persists them beside the
formula, proposition-map, clock, anchor, evaluator and dependency-revision
references. FR-004 constrains how those fields are validated and propagated; it
does not introduce a type from another crate.

The `quire-contract-ir` contract labels, schema digests and revisions that
FR-005's fixtures pin are asserted **by identity and digest over the bytes this
crate actually exchanged**, never by copying a schema, a vocabulary table or a
fixture out of `quire-contract-ir`. A copy would drift, and it would put
another repository's content in this one. The digest is checkable here,
offline, forever; the upstream contract identity is provenance beside it.

## Consequences

- `Cargo.toml` gains no dependency. The dependency graph stays `quire-mltl ->
  {tl-mltl, quire-observation, tl-syntax}`, with `quire-contract-ir` above it.
- `quire-contract-ir` #63 and #64 were the specification tickets for FR-025 and
  FR-026 and have both closed; their implementations are #70 (delivered by
  `quire-contract-ir` PR #77) and #71. So the "blocked on #63/#64" condition
  PR #44 carried is no longer the live gate. FR-005 pins the accepted
  *contract revisions* instead of the ticket states, because a ticket state is
  not a measurement of what the contract now says.
- A correspondence class this crate refuses is a specification finding for
  whichever side owns the gap, not a fallback to invent here. FR-001 already
  refuses a timestamped-event clock family as unsupported; FR-004 makes that
  refusal a first-class, joinable disposition rather than an opaque error.
- The dimension is a census, not a coverage ratio. FR-005 forbids reporting a
  percentage without its population, exclusions and blocked set, for the same
  reason `tl-mltl`'s NFR-004 does.

## Alternatives Considered

- **Specify the dimension in `quire-contract-ir`, next to FR-025/FR-026.**
  Rejected: FR-026 already owns the correspondence, the construction and the
  agreement decision. What is unspecified is the *counterparty* obligation — that
  the TL-side documents FR-026 constructs and reads carry the correspondence
  losslessly and refuse legibly. That obligation belongs to the crate that owns
  those documents, and a consumer cannot specify its supplier's contract
  stability.
- **Keep it in `tl-mltl` and depend on an artifact rather than a crate.**
  Rejected in `tl-mltl`'s own ADR-002: the direction of the dependency is what
  the ruling is about, and a campaign row that cannot become applicable until
  another repository's contract lands is a cross-repository dependency however
  it is spelled.
- **Leave it unspecified, since FR-001 already transports the two identity
  fields.** Rejected: FR-001 validates them as identity *text* and says nothing
  about them naming an accepted native correspondence, about propagation into
  the mapping, about successor identity on contract change, or about an
  unrepresentable class. A field that is merely carried is not a preserved
  identity, and "it round-trips today" is not a contract.
- **Add a `quire-contract-ir` dependency and consume its projection types
  directly.** Rejected: it closes a dependency cycle with TL-181, and it is not
  needed — the correspondence arrives as validated data, not as a type.
