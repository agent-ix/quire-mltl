# License decision

This repository is licensed under `AGPL-3.0-only`, not this org's usual
`AGPL-3.0-or-later`. That is a deliberate, non-default choice for this crate
specifically.

`quire-mltl` directly incorporates `quire-observation`
(`https://github.com/agent-ix/quire-observation`), which is itself licensed
`AGPL-3.0-only` (see its `Cargo.toml` `license` field and its own
`LICENSE-DECISION.md`). Combining a work under mismatched GPL-family "or
later" terms follows the most restrictive included term, so declaring
`AGPL-3.0-or-later` here would misrepresent the actual right granted to
downstream users: this crate can never lawfully be distributed under a later
AGPL version unless `quire-observation` also permits that.

The license body text is the unmodified GNU Affero General Public License,
version 3 (the FSF text in `LICENSE`, identical across every AGPL repository
in this org — there is only one license body). The `-only` vs `-or-later`
distinction is a licensing statement about which versions apply, not a
different license body, and is carried by:

- `license = "AGPL-3.0-only"` in `Cargo.toml` (the SPDX identifier), and
- this file.

Tracked under `agent-ix/quire-mltl` TL-176, part of epic TL-175.
