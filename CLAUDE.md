# quire-mltl

Bridges `tl-mltl`'s TL-owned evaluation types to `quire-observation`'s
owner-assertion views — the one deliberate exception to `tl-mltl`'s
independence from the agent-ix/Quire ecosystem (architect ruling, Linear epic
TL-175: every TL-* crate stays independent of Quire; only a dedicated
integration crate may bridge them).

## Current state

TL-176 scaffolded the repository, TL-177 authored MRS-001/FR-001..FR-003/
NFR-001/ADR-001, and TL-178 landed the port: `src/{request,report,dispatch,
contract_ir}.rs` carry `tl-mltl`'s former `wire::request`, `wire::report`,
`wire::observation` (renamed `dispatch`) and `mapping::contract_ir` modules.
`Cargo.toml` pins exact `tl-mltl`, `quire-observation` and `tl-syntax` source
revisions plus the `agent-ix-baseline-producer` dev-dependency.

FR-004 and FR-005 add the native-correspondence dimension: identity
preservation across the three documents, a typed refusal for a class this
crate cannot represent, and a closed correspondence-class census. They are
specification only — no implementation yet. ADR-002 records why that dimension
is here and why this crate is the *supplier* to `quire-contract-ir` FR-026,
not its consumer. **This crate takes no `quire-contract-ir` dependency**; one
would close a cycle with TL-181.

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

`AGPL-3.0-or-later`, this org's default for `quire-*` program repositories.
See `LICENSE-DECISION.md`.

## Layout

```
src/lib.rs             # crate root (placeholder; TL-178 lands the real port)
tests/integration.rs   # end-to-end tests
spec/                  # requirements artifacts (structure only; TL-177 fills content)
scripts/               # local tooling
```
