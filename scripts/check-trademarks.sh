#!/usr/bin/env bash
# Do the public documents use the HL7 word marks the way HL7 asks?
#
# spec/hl7-trademarks-fair-use/ states the fair-use terms this repository works
# under, and ends with "Assurance: create automatic tests to verify this works."
# This is that test. It exists because a trademark rule is exactly the kind of
# obligation that is satisfied once, by hand, and then quietly lost the next
# time someone adds a page.
#
# Two rules are checked, both from the spec:
#
#   1. The registration mark follows the FIRST use of a word mark on each page:
#      HL7 -> HL7(R), FHIR -> FHIR(R), CDA -> CDA(R).
#   2. Any page using a word mark carries the disclaimer verbatim.
#
# A third rule — refer to it as the "HL7(R) FHIR(R) standard" in a place of
# prominence — is editorial rather than mechanical, and is applied in README.md
# and NEWS.md rather than gated here.
#
# What does NOT count as a trademark use, and is masked before checking: fenced
# and inline code, markdown link targets, and URLs. `fhir-sqlite-store` is a
# crate name and FHIR_POSTGRESQL_BENCH is an environment variable; neither is a
# use of the mark in the sense the fair-use rules address.
#
#   scripts/check-trademarks.sh          check
#   scripts/check-trademarks.sh --list   also list compliant files
#
# Exits non-zero if any document is non-compliant.
set -euo pipefail

cd "$(dirname "$0")/.."
LIST="${1:-}"

python3 - "$LIST" <<'PY'
import glob, re, sys

LIST = sys.argv[1] == "--list" if len(sys.argv) > 1 else False

DISCLAIMER = ("HL7®, and FHIR® are the registered trademarks of Health Level Seven "
              "International and their use of these trademarks does not constitute an "
              "endorsement by HL7.")
MARKS = ("HL7", "FHIR", "CDA")

# Scope: the public-facing documents at the repository root, plus help/,
# doc/, the two support crates' top-level pages, the six ports' READMEs
# (markdown widened 2026-08-26), plus the rustdoc of the nine top-level
# crate roots (what docs.rs renders). Still not covered: the fhir/ family's
# own markdown (its fhir.md is a generated transcript that would drown the
# check), the ports' book/ chapters and internal docs, and fhir-loco's
# interior pages — widening these lists further is the way to cover them,
# and is deliberately a visible edit.
FILES = sorted(
    set(glob.glob("*.md"))
    | set(glob.glob("help/**/*.md", recursive=True))
    | set(glob.glob("doc/*.md"))
    | set(glob.glob("fhir-store/*.md"))
    | set(glob.glob("fhir-loco/*.md"))
    | set(glob.glob("fhir-*/README.md"))
)
LIB_RS = ["fhir/src/lib.rs", "fhir-store/src/lib.rs", "fhir-loco/src/lib.rs"] + \
    sorted(glob.glob("fhir-*/crates/fhir-*-store/src/lib.rs"))

def rust_doc_prose(text):
    """Only doc comments, with their code fences removed (er7's approach)."""
    kept, fence = [], False
    for line in text.split("\n"):
        stripped = line.lstrip()
        if not (stripped.startswith("//!") or stripped.startswith("///")):
            kept.append("")
            continue
        body = re.sub(r"^\s*//[!/]", "", line)
        if re.match(r"\s*```", body):
            fence = not fence
            kept.append("")
            continue
        kept.append("" if fence else body)
    return "\n".join(kept)

NOT_PROSE = [
    re.compile(r"```.*?```", re.S),   # fenced code
    re.compile(r"`[^`\n]*`"),         # inline code
    re.compile(r"\]\([^)]*\)"),       # markdown link targets
    re.compile(r"<https?://[^>]*>"),  # autolinks
    re.compile(r"https?://\S+"),      # bare URLs
]

def prose_mask(text):
    mask = [True] * len(text)
    for pat in NOT_PROSE:
        for m in pat.finditer(text):
            mask[m.start():m.end()] = [False] * (m.end() - m.start())
    return mask

def first_prose_use(text, mark):
    mask = prose_mask(text)
    for m in re.finditer(r"\b%s\b" % mark, text):
        if mask[m.start()]:
            return m.span()
    return None

failures = 0
for path in FILES + LIB_RS:
    text = open(path, encoding="utf-8").read()
    if path.endswith(".rs"):
        text = rust_doc_prose(text)
    problems, used = [], []
    for mark in MARKS:
        span = first_prose_use(text, mark)
        if span is None:
            continue
        used.append(mark)
        if text[span[1]:span[1] + 1] != "®":
            line = text[:span[0]].count("\n") + 1
            problems.append("line %d: first use of %s is not %s®" % (line, mark, mark))
    if used and DISCLAIMER not in re.sub(r"\s+", " ", text):
        problems.append("uses %s but omits the required disclaimer" % ", ".join(used))

    if problems:
        failures += 1
        print("FAIL %s" % path)
        for p in problems:
            print("       %s" % p)
    elif used and LIST:
        print("ok   %s (%s)" % (path, ", ".join(used)))
    elif LIST:
        print("--   %s (no word marks in prose)" % path)

if failures:
    print("\n%d document(s) do not meet spec/hl7-trademarks-fair-use/." % failures)
    print("Fix: add ® after the first prose use, and append the disclaimer:\n")
    print("    ## Trademarks\n")
    print("    " + DISCLAIMER)
    sys.exit(1)

print("All documents meet spec/hl7-trademarks-fair-use/.")
PY
