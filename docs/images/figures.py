# SPDX-License-Identifier: AGPL-3.0-or-later
# Copyright (C) 2026 Agent-IX
"""quire-mltl's documentation figures, rendered by the docs-figures skill.

Regenerate from the repository root with the agent-ix/dev-tools skill:

    python3 -m pip install fonttools
    python3 <dev-tools>/skills/docs-figures/scripts/render.py docs/images/figures.py

Add --check to confirm the committed SVGs are current. The outcome rows follow
the order of checks in src/contract_ir.rs (`outcome`).
"""

from docs_figures import FONTS, SMALL, TITLE, Figure  # noqa: F401

STYLE = "ix-docs"

# One role per kind of thing, fixed across the Quire repositories.
ROLES = [
    ("code", "authored source", "box"),
    ("logic", "compiler stage", "box"),
    ("model", "checked package / IR", "box"),
    ("evidence", "outcome", "box"),
    ("ink", "input", "box"),
]


def hero():
    """README hero: owner-read inputs become a request, a result, then a mapped outcome."""
    f = Figure(
        "hero",
        980,
        372,
        "quire-mltl takes an MLTL formula, a proposition map, a trace or history, a clock and the "
        "observation facts about that evidence, and derives one canonical temporal request. The "
        "request is evaluated by tl-mltl into one result, and the result is mapped to either a "
        "Boolean value or a typed non-value reason.",
    )
    inputs = [
        ("formula", "MLTL, from tl-syntax"),
        ("proposition map", "names → ids"),
        ("trace or history", "future or past lane"),
        ("clock", "from quire-observation"),
        ("observation facts", "progress · closure"),
    ]
    for i, (title, sub) in enumerate(inputs):
        f.box(0, i * 64, 180, 56, "ink", title, sub)
    row = 128
    f.box(232, row, 150, 56, "logic", "request::derive", "check and seal")
    f.box(452, row, 150, 56, "logic", "report::evaluate", "tl-mltl runs it")
    f.box(672, row, 150, 56, "logic", "contract_ir::map", "value or non-value")
    f.line([(180, 28), (206, 28), (206, row + 28), (229, row + 28)], "ink")
    for i in (1, 2, 3):
        f.line([(180, i * 64 + 28), (206, i * 64 + 28)], "ink", arrow=False)
    f.line([(206, 28), (206, 4 * 64 + 28)], "ink", arrow=False)
    f.line([(180, 4 * 64 + 28), (206, 4 * 64 + 28)], "ink", arrow=False)
    f.line([(382, row + 28), (449, row + 28)], "logic")
    f.line([(602, row + 28), (669, row + 28)], "logic")
    f.note(416, row + 18, "request", "model", anchor="middle")
    f.note(636, row + 18, "result", "model", anchor="middle")
    f.line([(822, row + 28), (846, row + 28)], "logic", arrow=False)
    f.line([(846, 76), (846, 204)], "logic", arrow=False)
    f.line([(846, 76), (870, 76)], "logic")
    f.line([(846, 204), (870, 204)], "evidence")
    f.box(872, 48, 108, 56, "logic", "Value", "true / false")
    f.box(872, 176, 108, 56, "evidence", "NonValue", "typed reason")
    f.legend(0, 366, [ROLES[4], ROLES[1], ("model", "document passed on", "box"), ("evidence", "no Boolean", "box")])
    return f


def outcomes():
    """The order in which a result is checked on its way to a mapped outcome."""
    rows = [
        ("execution did not complete", "resource-incomplete · unsupported · failed · refused",
         "NonValue", "that same reason", "evidence"),
        ("completeness is not complete", "contradicted · incomplete",
         "NonValue", "contradicted · incomplete", "evidence"),
        ("result is not available", "not yet observed · producer or contract unavailable",
         "NonValue", "unavailable", "evidence"),
        ("truth is not settled", "an open prefix that has not decided",
         "NonValue", "pending", "evidence"),
        ("truth is settled", "satisfied · violated",
         "Value", "true · false", "logic"),
    ]
    f = Figure(
        "outcomes",
        980,
        312,
        "A result is checked in order. Execution that did not complete, then completeness that is "
        "not complete, then an unavailable result, then unsettled truth each map to a NonValue "
        "with its own reason. Only a settled satisfied or violated result maps to a Boolean Value.",
    )
    f.text(0, 14, "CHECKED TOP TO BOTTOM · THE FIRST ROW THAT APPLIES WINS", "muted", "mono", SMALL)
    for i, (title, sub, out, out_sub, role) in enumerate(rows):
        y = 28 + i * 56
        f.box(0, y, 540, 48, "logic", title, sub)
        f.line([(540, y + 24), (617, y + 24)], role)
        f.box(620, y, 360, 48, role, out, out_sub)
    return f


REPOS = [
    ("ORCHESTRATION", [("quire-verification", True), ("quire-driver", True), ("quire-integration", True)]),
    ("BACKENDS", [("quire-contract-codegen", False), ("quire-contract-runtime", False),
                  ("quire-analyze", False), ("quire-protocol", True), ("quire-mltl", False)]),
    ("LANGUAGE", [("quire-specification", True), ("quire-spec-language", False), ("quire-contract-ir", False)]),
    ("FOUNDATIONS", [("quire-exact", False), ("quire-semantic-value", False), ("quire-walk", False),
                     ("quire-canonical", False), ("quire-verification-contracts", False),
                     ("quire-observation", False)]),
]


def ecosystem(here="quire-spec-language"):
    """The Quire repositories in bands; each band builds on the ones below."""
    f = Figure(
        "ecosystem",
        980,
        364,
        "The Quire repositories in four bands. Foundations: quire-exact, quire-semantic-value, "
        "quire-walk, quire-canonical, quire-verification-contracts, quire-observation. Language: "
        "quire-specification, quire-spec-language, quire-contract-ir. Backends: "
        "quire-contract-codegen, quire-contract-runtime, quire-analyze, quire-protocol, "
        "quire-mltl. Orchestration: quire-verification, quire-driver, quire-integration.",
    )
    y = 0
    gap = 10
    for band, repos in REPOS:
        f.text(0, y + 12, band, "muted", "mono", SMALL)
        widths = [FONTS["title"].width(name, TITLE) + 24 for name, _ in repos]
        spare = (980 - sum(widths) - gap * (len(repos) - 1)) / len(repos)
        x = 0
        for (name, private), w in zip(repos, widths):
            w = round(w + spare)
            role = "logic" if name == here else "ink"
            f.box(x, y + 20, w, 50, role, name, "private" if private else "public", dashed=private)
            x += w + gap
        y += 88
    f.legend(0, 360, [("logic", "this repository", "box"), ("ink", "private", "dash")])
    return f


def ecosystem_here():
    """The ecosystem figure with this repository highlighted."""
    return ecosystem(here="quire-mltl")


FIGURES = [hero, outcomes, ecosystem_here]
