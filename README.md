# quire-mltl

`quire-mltl` bridges `tl-mltl`'s TL-owned evaluation types to
`quire-observation`'s owner-assertion views.

`tl-mltl` is intended to be fully independent of the agent-ix/Quire ecosystem:
an architect ruling (Linear epic TL-175) requires every TL-* crate to stay
independent of Quire, with only a dedicated integration crate permitted to
bridge them. `quire-mltl` is that one deliberate exception. It exists so
`tl-mltl` itself never has to depend on, or know about, Quire Observation.

## Current state

This repository is freshly scaffolded (TL-176) and has no implementation yet.
`src/lib.rs` is a placeholder. `Cargo.toml` pre-declares and pins the
`tl-mltl`, `quire-observation`, and `tl-syntax` dependencies (plus
`agent-ix-baseline-producer` as a dev-dependency) that the bridging
implementation will use, so that port does not need to redo this scaffolding
step. The actual wire/mapping code that uses them lands in a later ticket,
TL-178. The `spec/` tree is likewise structurally seeded but not yet
populated with real requirements; that is TL-177.

## Build

```bash
make ci
```

GitHub Actions is intentionally `workflow_dispatch`-only. Use local `make ci`
while iterating and dispatch hosted CI only for a finalized revision.

## License

Licensed under `AGPL-3.0-or-later`, this org's default for `quire-*` program
repositories. `quire-mltl` directly incorporates `quire-observation`, which
is itself `AGPL-3.0-or-later` as of its 2026-09-13 relicense, so nothing in
this crate's dependency graph requires a narrower term. See
`LICENSE-DECISION.md`.

Registry publication is disabled (`publish = false`) until a v0.1 assurance
review is complete, matching `tl-mltl`'s own convention.
