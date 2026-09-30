---
id: FR-005
title: "Census the native-correspondence classes the bridge carries, and replay each one"
type: FR
relationships:
  - target: ix://agent-ix/quire-mltl/MRS-001
    type: implements
  - target: ix://agent-ix/quire-mltl/FR-004
    type: depends_on
  - target: ix://agent-ix/quire-contract-ir/FR-026
    type: references
---
# FR-005: Census the native-correspondence classes the bridge carries, and replay each one

## Description

When the native-correspondence bridge is measured, quire-mltl SHALL enumerate a
closed, reviewed set of correspondence classes, classify every class as
`applicable`, `excluded`, or `blocked` exactly once, bind each applicable class
to exactly one canonical fixture with an independently derived expected
outcome, and replay every applicable class.
The census SHALL report its population, its exclusions and its blocked set
rather than a bare ratio, and SHALL NOT admit a class whose state it cannot
justify.

## Inputs

- The closed class catalog below, and the reviewed class registry that
  enumerates it.
- One canonical fixture per applicable class: the exact request bytes, the
  exact supplied owner views and formula/proposition-map/trace-or-history
  documents, and the exact expected result and mapped-outcome bytes.
- The exact `quire-contract-ir` predicate-projection and
  temporal-correspondence contract labels each class was reviewed against,
  as identity text.

## Outputs

- A deterministic, ordered class census with separate `applicable`, `excluded`
  and `blocked` populations.
- A replay record per applicable class: the recorded expected outcome, the
  re-derived outcome, and their exact agreement or a typed mismatch.
- A typed refusal naming every duplicate, omitted, unknown, contradictory, or
  unjustified class, and every fixture whose bytes do not match what it
  declares.

## Behavior

### The class catalog is closed

A correspondence class is the tuple of dimensions the bridge documents must
carry losslessly:

| Dimension | Classes |
|---|---|
| lane | future closed-trace, future online-prefix, past origin-complete history |
| clock family | event-position, fixed-sample; unsupported timestamped-event |
| clock/history binding | agreeing event-position pair, agreeing fixed-sample pair, disagreeing pair, absent binding for a past request |
| proposition coverage | every referenced proposition present, a referenced proposition absent from the map |
| completeness | complete, incomplete, contradicted |
| availability | available, not-yet-available, producer-unavailable, contract-unavailable |
| execution and truth | completed Satisfied, completed Violated, completed Pending, resource-incomplete, unsupported, failed, refused |
| result relation | original, superseding, invalidating |
| mapped outcome | value true, value false, and each of the eight typed non-value reasons |
| resource boundary | at each declared ceiling, and one over it |

The registry is the reviewed ordered enumeration of that catalog, not its full
Cartesian product: every class above appears in at least one registry entry,
and a combination that is invalid by construction appears as an explicit
`excluded` entry with a stable reason code when it is a boundary the census
intends to keep visible. The registry is sorted by the UTF-8 bytes of its class
tokens.

A class's lifecycle state is **not** part of its key. A class cannot be
re-keyed when it moves from `blocked` to `applicable`, so the transition is
recorded against the same class rather than presented as new coverage.

### Each state has to earn itself

- `applicable` requires exactly one canonical fixture, its independently
  derived expected outcome. One fixture may serve several classes, but each class-to-fixture
  reference stays explicit.
- `excluded` requires a stable reason code establishing that the combination is
  invalid by construction, redundant under another class, or outside this
  crate's scope. "Not implemented" is not an exclusion reason.
- `blocked` requires a named unresolved dependency and the exact condition that
  would admit it. A blocked class is visible, carries no outcome, and is
  excluded from the applicable denominator.

### Expected outcomes are independent, and replay is exact

For each applicable class the expected result and mapped outcome are recorded
before replay and identify either an independently derived expectation or a
reviewed hand-derived argument. An independent expectation shares only the
public canonical document types: it cannot call this crate's `derive`,
`evaluate`, `read`, or `map` paths, and it cannot read the expectation it is
deriving. The production path cannot generate its own expected value during the
replay.

Replay strict-reads the documents, then re-derives the outcome and compares
it. A fixture whose bytes or contract labels disagree with what it declares is
a typed refusal, not a mismatch to investigate later.

### Consumer contracts are referenced by identity, never copied

Each class records the `quire-contract-ir` predicate-projection and
temporal-correspondence contract labels it was reviewed against. No schema, vocabulary table, or fixture is copied out of
`quire-contract-ir` into this repository; a class references its counterpart
contract by identity (see
[ADR-002](../decisions/ADR-002-quire-mltl-owns-the-native-correspondence-dimension.md)).

### The census reports a population, never a bare score

Any reported figure names its population, its exclusions, and its blocked
set. A class count, a fixture count, or a
percentage is not by itself evidence that the bridge is correct, complete,
qualified, or released, and the census records no approval or release decision
— that authority remains with the named human release authority, as NFR-001
requires.

## Acceptance Criteria

| ID | Criteria | Verification |
|---|---|---|
| FR-005-AC-1 | Every entry in the reviewed class registry appears exactly once as `applicable`, `excluded`, or `blocked`; every class in the closed catalog appears in at least one entry; and deleting, duplicating, reordering, reclassifying, or adding an unknown entry makes the census fail while naming the complete conflicting set. | Test |
| FR-005-AC-2 | Every `applicable` class has exactly one canonical fixture and an independently derived expected outcome, every `excluded` class has a stable reason code that is not "not implemented", and every `blocked` class names an unresolved dependency and its exact admission condition and carries no outcome. | Test |
| FR-005-AC-3 | A class key excludes lifecycle state, so moving a class from `blocked` to `applicable` preserves its key and its prior population remains comparable. | Inspection |
| FR-005-AC-4 | Replay of every applicable class strict-reads its documents, re-derives the outcome, and agrees exactly with the recorded expectation; an expectation produced by calling this crate's own `derive`/`evaluate`/`read`/`map` paths is refused as non-independent. | Test |
| FR-005-AC-5 | A fixture whose bytes or contract label disagrees with what it declares is a typed refusal; the same fixture with agreeing identities replays. | Test |
| FR-005-AC-6 | A test enumerates every file under `spec/` and `tests/` in this repository and confirms none embeds a `quire-contract-ir` schema, vocabulary table, or fixture body: each class's `quire-contract-ir` reference is exactly a bounded identity record — contract labels, as text. | Test |
| FR-005-AC-7 | Every reported figure carries its population, exclusions, and blocked set; a census output that states a ratio without them, or that records an approval, qualification, or release decision, is refused. | Test |

## Dependencies

Depends on [FR-004](./FR-004-preserve-native-correspondence-identity.md) for
the identity-preservation and refusal obligations the classes are the coverage
model for, and through it on FR-001 and FR-003.

It references `quire-contract-ir` FR-026 as the consumer correspondence each
class is reviewed against. As in FR-004 this is deliberately not a
`depends_on`: the crate dependency runs `quire-contract-ir -> quire-mltl`.
