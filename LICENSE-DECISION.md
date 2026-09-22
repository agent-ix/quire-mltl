# License decision

This repository is licensed under `AGPL-3.0-or-later`, this org's default for
`quire-*` program repositories (PGM-01-R04).

An earlier version of this decision declared `AGPL-3.0-only` as a deliberate
exception, reasoning that `quire-mltl` directly incorporates
`quire-observation` (`https://github.com/agent-ix/quire-observation`), which
was believed to be `AGPL-3.0-only` itself. That premise was wrong: as of
`quire-observation`'s 2026-09-13 owner decision, it is `AGPL-3.0-or-later`
(see its `Cargo.toml` `license` field and its own `LICENSE-DECISION.md`,
confirmed live at the exact `quire-mltl` dependency pin). The reasoning for an
exception no longer applies — nothing in this crate's dependency graph forces
`-only`: `quire-observation` and `tl-syntax`/`tl-mltl` all permit `-or-later`
or a compatible permissive license. The one `AGPL-3.0-only` term present,
`agent-ix-baseline-producer`, is a `[dev-dependencies]` entry and does not
constrain the distributed library.

The license body text is the unmodified GNU Affero General Public License,
version 3 (the FSF text in `LICENSE`, identical across every AGPL repository
in this org — there is only one license body). The `-only` vs `-or-later`
distinction is a licensing statement about which versions apply, not a
different license body, and is carried by:

- `license = "AGPL-3.0-or-later"` in `Cargo.toml` (the SPDX identifier), and
- this file.

Tracked under `agent-ix/quire-mltl` TL-176, part of epic TL-175. Corrected
during TL-176's review — caught by an Opus review agent that fetched
`quire-observation`'s actual `origin/main` instead of trusting a local
checkout sitting on a stale branch.
