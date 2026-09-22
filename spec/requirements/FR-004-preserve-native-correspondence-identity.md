---
id: FR-004
title: "Preserve native-correspondence identity losslessly across the request, result and Contract-IR mapping"
type: FR
relationships:
  - target: ix://agent-ix/quire-mltl/MRS-001
    type: implements
  - target: ix://agent-ix/quire-mltl/FR-001
    type: depends_on
  - target: ix://agent-ix/quire-mltl/FR-003
    type: depends_on
  - target: ix://agent-ix/quire-contract-ir/FR-025
    type: references
  - target: ix://agent-ix/quire-contract-ir/FR-026
    type: references
---
# FR-004: Preserve native-correspondence identity losslessly across the request, result and Contract-IR mapping

## Description

When a temporal assessment is derived on behalf of a native Quire
correspondence, quire-mltl SHALL validate the supplied native subject and
correspondence identities, bind both into the request's content identity, and
carry both verbatim into the result and the Contract-IR mapping, so that the
consuming correspondence owner can join a mapped outcome back to the exact
native subject and correspondence profile it constructed the request for.
quire-mltl SHALL NOT derive, parse, interpret, default, or substitute either
identity, and SHALL NOT own any native grammar, predicate semantics,
correspondence derivation, or agreement decision.

## Inputs

- The `subject_identity` and `correspondence_identity` text
  [FR-001](./FR-001-admit-temporal-assessment-requests-and-results.md) already
  admits on `RequestInput`.
- The exact `quire-contract-ir` predicate-projection and
  temporal-correspondence contract identities and schema revisions the caller
  selected, as identity text and digests, not as types.
- For a mapping: the constructor-private `ValidatedTemporalResult` and
  `MappingSelection` [FR-003](./FR-003-derive-contract-ir-result-mapping.md)
  requires.

## Outputs

- A request, result and mapping whose native subject and correspondence
  identities are byte-identical to the ones supplied, and whose content
  identities each cover those fields.
- A typed `OwnerReadError` when either identity is absent, malformed, or
  disagrees with the expectation a strict reader was given, with no partial
  document emitted.
- For a correspondence class this crate cannot represent, a typed non-value
  outcome naming the exact refused dimension, joinable by the consumer as an
  unrepresented or refused correspondence.

## Behavior

### The two identities are opaque, exact, and load-bearing

`subject_identity` names the checked native temporal subject the correspondence
owner admitted. `correspondence_identity` names the correspondence profile
under which that subject was lowered to a TL formula. quire-mltl treats both as
closed identity text: it validates their shape and requires them present and
non-empty, and it performs no other reading of them. It derives neither from
the other, from the formula, or from any `quire_observation` owner view.

Both fields are inside the preimage of the request's content identity. A
request that differs only in its correspondence identity is therefore a
different request, and a mapped outcome cannot be re-keyed from one
correspondence onto another by rewriting a field: the identity, and every
downstream identity derived from it, changes.

### Propagation is verbatim, and reaches the mapping

The result already binds the request identity and copies the subject and
correspondence identities forward. FR-003's mapping embeds the source result's
complete wire content. This requirement makes that reach explicit and
non-optional: the mapped `tl-mltl.contract-ir-result-map/v1` document SHALL
carry the native subject and correspondence identities of the request its
source result answered, so a consumer joining a mapped outcome needs no second
fetch and no out-of-band correlation. A mapping whose embedded correspondence
identity disagrees with its source result's is refused with
`ExpectedMismatch`, not repaired.

A correction accepted under FR-001-AC-5 already requires the predecessor's
subject and correspondence to match exactly. This requirement adds that a
correction chain SHALL NOT change either identity at any revision: a
correspondence is not re-pointed by superseding a result.

### An unrepresentable class refuses legibly

Some correspondence classes cannot be carried by this crate's contracts. A
timestamped-event clock family is the concrete case FR-001 already refuses as
unsupported; a past-lane request whose clock range family disagrees with the
history's own binding is another. For every such class quire-mltl SHALL emit a
typed non-value naming the exact dimension refused — clock family, clock
binding, semantic profile, lane, proposition coverage, or resource ceiling —
and SHALL NOT emit a Boolean, a `Pending`, a defaulted value, or a partial
document. A refusal is an outcome the consumer records; it is never a gap the
consumer has to infer from an absent reply.

This crate does not invent a representation for a refused class. A class that
ought to be representable and is not is a specification finding for whichever
side owns the gap, raised as such rather than closed with a local fallback.

### Contract stability is part of the obligation

The consuming correspondence owner admits this crate's documents against an
expected contract label, schema digest and revision. A silent change to the
shape of `tl-mltl.temporal-assessment-request/v1`,
`tl-mltl.temporal-assessment-result/v1`, or
`tl-mltl.contract-ir-result-map/v1` would therefore invalidate that join
without either side failing. Accordingly, quire-mltl SHALL NOT change the
field set, field meaning, canonical byte form, or identity preimage of any of
the three contracts without a successor contract label, and SHALL NOT reuse a
label across two shapes. A strict reader SHALL refuse an unknown or
non-selected contract label rather than reading it permissively.

### What this requirement does not add

No native grammar, predicate parser, predicate evaluator, signal-catalog
derivation, clock/history correspondence derivation, agreement decision,
Boolean coercion, callback, plugin, or trust flag. No dependency on
`quire-contract-ir`: the correspondence reaches this crate as validated data
inside a request, and the crate's dependency graph stays `tl-mltl`,
`quire-observation` and `tl-syntax` (see
[ADR-002](../decisions/ADR-002-quire-mltl-owns-the-native-correspondence-dimension.md)).

## Acceptance Criteria

| ID | Criteria | Verification |
|---|---|---|
| FR-004-AC-1 | A request is admitted only with a present, well-formed native subject identity and correspondence identity; an absent, empty, or malformed value is refused with a typed error and no partial document, and neither identity is ever derived, defaulted, or substituted from the formula, the clock, or any owner view. | Test |
| FR-004-AC-2 | Both identities are inside the request's content-identity preimage: two requests identical except for the correspondence identity produce different request identities, and every result and mapping identity derived from them differs accordingly. | Test |
| FR-004-AC-3 | The emitted result and the emitted Contract-IR mapping both carry the native subject and correspondence identities of the originating request byte-identically; a mapping or result whose embedded identities disagree with its source is refused with `ExpectedMismatch` rather than corrected. | Test |
| FR-004-AC-4 | A correction chain preserves both identities unchanged at every revision; a superseding or invalidating result that would change either identity is refused. | Test |
| FR-004-AC-5 | Every correspondence class this crate cannot represent — including at least an unsupported clock family, a disagreeing past-lane clock binding, an incompatible semantic profile or lane, and an over-ceiling resource case — yields a typed non-value naming the exact refused dimension, and never a Boolean, a pending truth, a defaulted value, or a partial document. | Test |
| FR-004-AC-6 | A strict reader refuses a document whose contract label is unknown or is not the label the reader selected, and the field set, canonical byte form, and identity preimage of each of the three contracts are pinned by a test that fails when any of them changes without a successor label. | Test |
| FR-004-AC-7 | No public operation accepts a native grammar, predicate parser or evaluator, catalog derivation, correspondence derivation, agreement decision, Boolean coercion, callback, plugin, or trust flag, and the crate manifest declares no `quire-contract-ir` dependency. | Test |

## Dependencies

Depends on [FR-001](./FR-001-admit-temporal-assessment-requests-and-results.md)
for the request/result admission and identity discipline this requirement
constrains, and on
[FR-003](./FR-003-derive-contract-ir-result-mapping.md) for the mapping whose
embedded wire content it makes load-bearing.

It references `quire-contract-ir` FR-025 and FR-026 as the *consumer* contracts
this obligation exists to serve — FR-026 owns the correspondence, the
construction of the TL artifacts, and the agreement decision, and joins a
`TlMappedResultView` this crate supplies. The reference is deliberately not a
`depends_on`: nothing in this requirement is gated on `quire-contract-ir`
landing anything, and the dependency between the crates runs
`quire-contract-ir -> quire-mltl`, never the reverse.
