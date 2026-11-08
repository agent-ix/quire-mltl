# SPDX-License-Identifier: AGPL-3.0-or-later
# Copyright (C) 2026 Agent-IX
"""quire-mltl documentation figures, rendered by the docs-figures skill.

Regenerate from the repository root with the agent-ix/dev-tools skill:

    python3 -m pip install fonttools
    python3 <dev-tools>/skills/docs-figures/scripts/render.py docs/images/figures.py

Add --check to confirm the committed SVGs are current. The outcome rows follow
the order of checks in src/contract_ir.rs (`outcome`).

The block between the two SHARED markers is identical in every TL repository
(tl-syntax, tl-parse, tl-rewrite, tl-mltl, tl-oracle, quire-mltl). Copy it
verbatim and call tl_stack(here="<crate>") and stack_pipeline(here="<crate>"). The block
between the two SHARED SCENARIO markers is also identical in every TL repository; call
scenario_sequence(here="<crate>").
"""

from docs_figures import FONTS, SMALL, TITLE, Figure  # noqa: F401

STYLE = "ix-docs"

# One role per kind of thing, fixed across the TL and Quire repositories.
ROLES = [
    ("code", "authored source", "box"),
    ("logic", "processing step", "box"),
    ("model", "checked document / view", "box"),
    ("evidence", "outcome", "box"),
    ("ink", "input", "box"),
]


# ==== SHARED: copy verbatim into sibling figures.py (begin) ===================

TL_CRATES = [
    # (column, row, name, one-line role, dependency note)
    (0, 1, "tl-parse", "text to formula graph"),
    (1, 1, "tl-oracle", "independent reference"),
    (2, 1, "tl-mltl", "evaluates traces"),
    (3, 1, "quire-mltl", "the bridge to Quire"),
    (2, 2, "tl-rewrite", "equivalent rewrites"),
    (3, 2, "quire-observation", "Quire observations"),
]


def tl_stack(here="tl-syntax"):
    """The TL crate family as a dependency graph; `here` is drawn highlighted."""
    f = Figure(
        "tl-stack",
        980,
        356,
        "The TL crates. tl-syntax defines the formula graph, profiles and signal catalog, and "
        "tl-parse, tl-oracle, tl-mltl, tl-rewrite and quire-mltl all depend on it. tl-rewrite "
        "depends on tl-mltl. quire-mltl depends on tl-mltl and quire-observation, and is the "
        "single bridge to Quire. Dashed lines are development-only: tl-mltl and tl-rewrite "
        "test against tl-oracle, and tl-rewrite tests against tl-parse.",
    )
    w, h, step = 192, 56, 262
    rows = {0: 24, 1: 140, 2: 256}
    f.text(3 * step, 244, "QUIRE", "muted", "mono", SMALL)
    f.box(0, rows[0], 980, h, "logic" if here == "tl-syntax" else "ink", "tl-syntax",
          "formula graph · semantic profiles · signal catalog · strict documents")
    where = {}
    for col, row, name, sub in TL_CRATES:
        x, y = col * step, rows[row]
        where[name] = (x, y)
        role = "logic" if name == here else "ink"
        f.box(x, y, w, h, role, name, sub, dashed=(name == "quire-observation"))
    for name in ("tl-parse", "tl-oracle", "tl-mltl", "quire-mltl"):
        x, y = where[name]
        f.line([(x + w / 2, rows[0] + h), (x + w / 2, y - 3)], "model")
    f.note(where["tl-parse"][0] + w / 2 + 10, 104, "types", "model")
    mx, my = where["tl-mltl"]
    rx, ry = where["tl-rewrite"]
    f.line([(mx + w / 2, my + h), (rx + w / 2, ry - 3)], "model")
    f.note(mx + w / 2 + 10, my + h + 30, "documents", "model")
    qx, qy = where["quire-mltl"]
    f.line([(mx + w, my + h / 2), (qx - 3, qy + h / 2)], "model")
    f.note(mx + w + 8, my + h / 2 - 8, "results", "model")
    ox, oy = where["quire-observation"]
    f.line([(ox + w / 2, oy - 3), (ox + w / 2, qy + h)], "model")
    f.note(ox + w / 2 + 10, qy + h + 30, "observations", "model")
    # development-only edges: tl-oracle and tl-parse feed tests of tl-mltl and tl-rewrite
    px, py = where["tl-parse"]
    zx, zy = where["tl-oracle"]
    f.line([(zx + w, zy + h / 2), (mx - 3, my + h / 2)], "model", dashed=True)
    f.line([(zx + w / 2, zy + h), (zx + w / 2, ry + 18), (rx - 3, ry + 18)], "model", dashed=True)
    f.line([(px + w / 2, py + h), (px + w / 2, ry + 40), (rx - 3, ry + 40)], "model", dashed=True)
    f.legend(0, 350, [("logic", "this repository", "box"), ("ink", "other crate", "box"),
                      ("model", "depends on", "solid"), ("model", "development only", "dash")])
    return f


PIPELINE_STAGES = [
    ("MLTL text", "your formula", "code", None),
    ("tl-parse", "parse", "logic", "tl-parse"),
    ("tl-rewrite", "rewrite", "logic", "tl-rewrite"),
    ("tl-mltl", "evaluate", "logic", "tl-mltl"),
    ("tl-oracle", "tests only", "logic", "tl-oracle"),
    ("quire-mltl", "map to Quire", "logic", "quire-mltl"),
]


def stack_pipeline(here="tl-syntax"):
    """End-to-end flow: text, parse, rewrite, evaluate with an oracle cross-check, Quire view."""
    f = Figure(
        "pipeline",
        980,
        392,
        "A formula moves from text to a Quire outcome. tl-parse turns MLTL text into a formula "
        "document, tl-rewrite returns a rewritten document, and tl-mltl evaluates it against a "
        "trace. In tests, tl-oracle evaluates the same formula and trace independently and the "
        "verdicts are compared. quire-mltl maps the result to a Boolean Value or a typed "
        "NonValue reason. All documents use the vocabulary of tl-syntax.",
    )
    w, h = 140, 56
    step = (980 - w) / (len(PIPELINE_STAGES) - 1)
    cx = [round(i * step + w / 2) for i in range(len(PIPELINE_STAGES))]
    for i, (name, sub, role, crate) in enumerate(PIPELINE_STAGES):
        if crate is not None:
            role = "logic" if crate == here else "ink"
        f.box(cx[i] - w // 2, 0, w, h, role, name, sub)
        f.line([(cx[i], h + 2), (cx[i], 312)], "faint", dashed=True, arrow=False, weight=1.0)
    msgs = [
        (0, 1, 96, "MLTL text", "code", False),
        (1, 2, 140, "formula document", "model", False),
        (2, 3, 184, "rewritten formula", "model", False),
        (3, 4, 228, "formula + trace", "model", True),
        (4, 3, 262, "reference verdict", "evidence", True),
        (3, 5, 306, "result", "model", False),
    ]
    for a, b, y, label, role, dev in msgs:
        sign = 1 if b > a else -1
        f.line([(cx[a] + 3 * sign, y), (cx[b] - 3 * sign, y)], role, dashed=dev,
               weight=2.4 if not dev else 1.5)
        left = min(cx[a], cx[b]) + 10
        f.note(left, y - 8, label, role)
    f.box(980 - 190, 332, 190, 50, "evidence", "Value or NonValue", "true / false or reason")
    f.line([(cx[5], 312), (cx[5], 329)], "evidence")
    f.box(0, 332, 520, 50, "logic" if here == "tl-syntax" else "ink", "tl-syntax",
          "the formula, profile, signal and document types every step uses")
    return f


# ==== SHARED (end) ============================================================


# ==== SHARED SCENARIO: copy verbatim into sibling figures.py (begin) ==========

SCENARIO_LANES = [
    # (name, one-word role, step number whose look-here callout sits on this lane)
    ("engineer", "you", None),
    ("tl-parse", "text", 1),
    ("tl-syntax", "signals", 2),
    ("tl-rewrite", "normalize", 3),
    ("tl-mltl", "evaluate", 4),
    ("R2U2", "outside", None),
    ("tl-oracle", "reference", 5),
    ("quire-mltl", "assurance", 6),
]

SCENARIO_LOOK = [
    # (step, crate, what to look at) for the case r2u2-future-deadline-v1
    (1, "tl-parse", "the tree is one F node, interval [0,3], over p2"),
    (2, "tl-syntax", "p2 is bound to signal s2, the signal the requirement waits for"),
    (3, "tl-rewrite", "status Unchanged under closed-trace: no rule applies, no step recorded; online-prefix has no rule set"),
    (4, "tl-mltl", "s2 is absent at instants 0 to 3, so false at time 0; R2U2 agrees"),
    (5, "tl-oracle", "a second evaluator reads the same formula and trace and returns false"),
    (6, "quire-mltl", "false becomes Value false only if the observation views allow a value"),
]


def scenario_sequence(here=None):
    """The debugging walkthrough as a sequence diagram; the lane named `here` is highlighted."""
    f = Figure(
        "scenario-sequence",
        980,
        684,
        "A deadline requirement fails on a recorded trace, and the engineer asks why. The engineer "
        "sends F[0,3] p2 to tl-parse and gets a formula document, binds p2 to signal s2 with tl-syntax, "
        "and sends the document to tl-rewrite, which reports Unchanged under the closed-trace profile. tl-mltl evaluates the formula "
        "and trace and returns false at time 0 with a buffer of 4 instants, and R2U2, which runs outside "
        "the crates, returns false at time 0 for the C2PO expression F[0,3](p2). tl-oracle evaluates the "
        "same formula and trace independently and returns false. quire-mltl maps the result to Value false. "
        "Numbered rings mark where to look at each step.",
    )
    w, h = 112, 50
    step = (980 - w) / (len(SCENARIO_LANES) - 1)
    cx = [round(i * step + w / 2) for i in range(len(SCENARIO_LANES))]
    lane = {name: i for i, (name, _, _) in enumerate(SCENARIO_LANES)}
    bottom = 492
    for i, (name, sub, _) in enumerate(SCENARIO_LANES):
        if name == "engineer":
            role = "code"
        elif name == here:
            role = "logic"
        else:
            role = "ink"
        f.box(cx[i] - w // 2, 0, w, h, role, name, sub, dashed=(name == "R2U2"))
        f.line([(cx[i], h + 2), (cx[i], bottom)], "faint", dashed=True, arrow=False, weight=1.0)
    y = 82
    pairs = [
        # (from, to, go label, go role, back label, back role, dashed, ring step)
        ("engineer", "tl-parse", "F[0,3] p2", "code", "formula doc", "model", False, 1),
        ("engineer", "tl-syntax", "signal catalog", "ink", "p2 bound to s2", "model", False, 2),
        ("engineer", "tl-rewrite", "formula doc", "model", "Unchanged", "evidence", False, 3),
        ("engineer", "tl-mltl", "formula + trace", "model", "false at 0, buffer 4", "evidence", False, 4),
        ("tl-mltl", "R2U2", "F[0,3](p2)", "model", "false at 0", "evidence", True, None),
        ("engineer", "tl-oracle", "formula + trace", "model", "false", "evidence", True, 5),
        ("engineer", "quire-mltl", "result", "model", "Value false", "evidence", False, 6),
    ]
    for a, b, go, go_role, back, back_role, dev, ring in pairs:
        ia, ib = lane[a], lane[b]
        f.line([(cx[ia] + 3, y), (cx[ib] - 3, y)], go_role, dashed=dev, weight=1.5 if dev else 2.4)
        f.text(cx[ia] + 12, y - 7, go, go_role, "mono", SMALL)
        yb = y + 26
        f.line([(cx[ib] - 3, yb), (cx[ia] + 3, yb)], back_role, dashed=dev, weight=1.5 if dev else 2.4)
        f.text(cx[ia] + 26, yb - 7, back, back_role, "mono", SMALL)
        if ring:
            rx, ry = cx[ib] + 22, y + 13
            f.dot(rx, ry, 9, "evidence", filled=False)
            f.text(rx, ry + 4, str(ring), "evidence", "mono", SMALL, "middle")
        y += 62
    f.text(0, bottom + 32, "Look here", "muted", "title", TITLE)
    ly = bottom + 56
    for n, crate, what in SCENARIO_LOOK:
        f.dot(9, ly - 4, 9, "evidence", filled=False)
        f.text(9, ly, str(n), "evidence", "mono", SMALL, "middle")
        f.text(30, ly, crate, "logic" if crate == here else "muted", "mono", SMALL)
        f.text(140, ly, what, "ink", "mono", SMALL)
        ly += 22
    return f


# ==== SHARED SCENARIO (end) ===================================================


def scenario_sequence_here():
    return scenario_sequence(here="quire-mltl")


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
    (1, 0, "quire-driver", "the quire command", False),
    (2, 0, "quire-integration", "tests the whole chain", False),
    (3, 0, "quire-verification", "plans verification", False),
    (0, 1, "quire-specification", "the standard", False),
    (1, 1, "quire-spec-language", "compiler · evaluator", False),
    (2, 1, "quire-contract-ir", "contract model", False),
    (3, 1, "quire-contract-codegen", "code · tests · proofs", False),
    (4, 1, "quire-contract-runtime", "runs generated checks", False),
    (0, 2, "quire-observation", "qualified observations", False),
    (1, 2, "quire-protocol", "protocol conformance", False),
    (3, 2, "quire-eval", "evaluation core", False),
    (2, 2, "quire-analyze", "SMT consistency", False),
    (0, 3, "quire-mltl", "MLTL over observations", False),
]
FEEDS = [
    ("quire-specification", "quire-spec-language"),
    ("quire-spec-language", "quire-contract-ir"),
    ("quire-contract-ir", "quire-contract-codegen"),
    ("quire-contract-codegen", "quire-contract-runtime"),
    ("quire-spec-language", "quire-protocol"),
    ("quire-contract-ir", "quire-analyze"),
    ("quire-observation", "quire-protocol"),
    ("quire-observation", "quire-mltl"),
]
SHARED = ["quire-exact", "quire-semantic-value", "quire-walk", "quire-canonical",
          "quire-verification-contracts"]


def ecosystem(here="quire-spec-language"):
    """The Quire repositories: the main chain, its branches, and what runs and shares it."""
    f = Figure(
        "ecosystem",
        980,
        476,
        "The Quire repositories. The main chain runs from the standard, quire-specification, to "
        "the compiler, quire-spec-language, then quire-contract-ir, quire-contract-codegen and "
        "quire-contract-runtime. quire-spec-language also feeds quire-protocol, and "
        "quire-contract-ir feeds quire-analyze. quire-observation feeds quire-protocol and "
        "quire-mltl. Above the chain, quire-driver runs it, quire-integration tests it and "
        "quire-verification plans verification. Below it, shared libraries used throughout: "
        "quire-exact, quire-semantic-value, quire-walk, quire-canonical and "
        "quire-verification-contracts.",
    )
    w, h, step = 176, 50, 201
    rows = {0: 20, 1: 120, 2: 210, 3: 290}
    where = {}
    for col, row, name, sub, private in REPOS:
        x, y = col * step, rows[row]
        where[name] = (x, y)
        role = "logic" if name == here else "ink"
        f.box(x, y, w, h, role, name, sub, dashed=private)
    for a, b in FEEDS:
        (ax, ay), (bx, by) = where[a], where[b]
        if ay == by:
            f.line([(ax + w, ay + h / 2), (bx - 3, by + h / 2)], "model")
        else:
            f.line([(ax + w / 2, ay + h), (bx + w / 2, by - 3)], "model")
    f.text(0, 12, "ORCHESTRATION", "muted", "mono", SMALL)
    f.text(0, 112, "THE CHAIN", "muted", "mono", SMALL)
    f.text(0, 382, "SHARED LIBRARIES · USED THROUGHOUT", "muted", "mono", SMALL)
    gap = 10
    widths = [FONTS["title"].width(name, TITLE) + 24 for name in SHARED]
    spare = (980 - sum(widths) - gap * (len(SHARED) - 1)) / len(SHARED)
    x = 0
    for name, w0 in zip(SHARED, widths):
        w0 = round(w0 + spare)
        f.box(x, 390, w0, h, "logic" if name == here else "ink", name)
        x += w0 + gap
    f.legend(0, 472, [("logic", "this repository", "box"), ("model", "feeds", "fill")])
    return f


def ecosystem_here():
    """The ecosystem figure with this repository highlighted."""
    return ecosystem(here="quire-mltl")


def tl_stack_here():
    return tl_stack(here="quire-mltl")


def pipeline_here():
    return stack_pipeline(here="quire-mltl")


FIGURES = [hero, tl_stack_here, pipeline_here, scenario_sequence_here, outcomes, ecosystem_here]
