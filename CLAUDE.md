# quire-mltl

Bridges `tl-mltl`'s TL-owned evaluation types to `quire-observation`'s
owner-assertion views — the one deliberate exception to `tl-mltl`'s
independence from the agent-ix/Quire ecosystem (architect ruling, Linear epic
TL-175: every TL-* crate stays independent of Quire; only a dedicated
integration crate may bridge them).

## Current state

Freshly scaffolded (TL-176). No bridging implementation yet — `src/lib.rs` is
a placeholder. `Cargo.toml` pre-declares and pins the `tl-mltl`,
`quire-observation`, and `tl-syntax` dependencies, plus the
`agent-ix-baseline-producer` dev-dependency, so the implementation ticket
(TL-178) doesn't have to redo this step. `spec/` is structurally seeded, not
populated — real FR/NFR/MRS content is TL-177. Do not port wire/mapping code
here before TL-178 is picked up.

## Commands

```bash
make fmt              # format with rustfmt
make fmt-check        # verify formatting (CI gate)
make lint             # clippy with -D warnings
make test             # cargo test
make deny             # cargo deny check licenses and sources
make audit-unsafe     # check that every unsafe block has a // SAFETY: comment
make docs             # cargo doc, denying missing/broken doc links
make ci               # complete local gate
```

GitHub Actions is intentionally `workflow_dispatch`-only. Use local `make ci`
while iterating and dispatch hosted CI only for a finalized revision.

## Specification workflow

All new or changed work must be specified before implementation: use `quoin
write` to obtain the current artifact contracts, then update the relevant
requirements, plans, tasks, matrix rows, and evidence links. Before requesting
review, run `quoin review` over the affected scope and validate with Quire.
Record selected analyses and findings; final Quoin acceptance remains a human
decision and must not be advanced automatically.

## Safety scaffolding

Mirrors `tl-mltl`'s own scaffolding:

- `clippy.toml` pins MSRV to `1.98` and caps cognitive complexity / arg count
- `deny.toml` allow-lists licenses (including the AGPL-3.0-only /
  AGPL-3.0-or-later terms this dependency graph actually carries) and denies
  unknown registries/git sources
- `scripts/check_unsafe_comments.sh` runs locally via `make audit-unsafe`.
  Every `unsafe {` block must have a `// SAFETY:` comment within the 3
  preceding lines, or be listed in `scripts/unsafe_comment_baseline.txt`.
- `rustfmt.toml` uses 100-char width
- `rust-toolchain.toml` pins to `1.98.1` + rustfmt + clippy

## License

`AGPL-3.0-only`, not this org's usual `AGPL-3.0-or-later` — a deliberate
choice because `quire-observation` (an `AGPL-3.0-only` dependency) is
incorporated directly. See `LICENSE-DECISION.md`.

## Layout

```
src/lib.rs             # crate root (placeholder; TL-178 lands the real port)
tests/integration.rs   # end-to-end tests
spec/                  # requirements artifacts (structure only; TL-177 fills content)
scripts/               # local tooling
```
