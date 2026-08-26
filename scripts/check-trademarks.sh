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
import glob, os, re, sys

LIST = sys.argv[1] == "--list" if len(sys.argv) > 1 else False

DISCLAIMER = ("HL7®, and FHIR® are the registered trademarks of Health Level Seven "
              "International and their use of these trademarks does not constitute an "
              "endorsement by HL7.")
MARKS = ("HL7", "FHIR", "CDA")

# Scope: every markdown file in the repository (tree-wide since 2026-08-26,
# replacing the allowlist that grew in tranches), plus the rustdoc of the
# nine top-level crate roots (what docs.rs renders), plus — since later on
# 2026-08-26 — the Cargo.toml `description` of every publishable crate (what
# crates.io renders): each must carry the disclaimer verbatim and the ® on
# the first use of each word mark. Exemptions are named,
# not implied, and each has a structural reason — not "too much to fix":
#
#   fhir/fhir.md            a generated 22 MB transcript, regenerated whole;
#                           its ® would be the generator's job, not an edit
#   book/src/SUMMARY.md     mdbook navigation manifests — appending prose
#                           breaks the book build, and their link titles are
#                           navigation, not prose
#   .github/                issue templates become the body of every filed
#                           issue; a footer there would inject itself into
#                           user content
#
# Directories skipped are build products and caches, not documents.
SKIP_DIRS = {"target", "node_modules", ".git", "spec-cache", "corpus"}
EXEMPT = {"fhir/fhir.md"}

def all_markdown():
    out = []
    for dirpath, dirnames, filenames in os.walk("."):
        dirnames[:] = [d for d in dirnames if d not in SKIP_DIRS]
        for name in sorted(filenames):
            if not name.endswith(".md"):
                continue
            path = os.path.normpath(os.path.join(dirpath, name))
            if path in EXEMPT or path.startswith(".github" + os.sep):
                continue
            if name == "SUMMARY.md" and (os.sep + "book" + os.sep) in path:
                continue
            out.append(path)
    return sorted(out)

FILES = all_markdown()
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

# --- Cargo.toml descriptions -------------------------------------------------
# What crates.io renders under each crate name is the `description` string, a
# page of its own in the fair-use sense. Every publishable crate's description
# must carry the disclaimer verbatim and the ® on the first use of each word
# mark (the same check_marks logic as above; a description is plain prose, so
# no masking is needed — crate names like `fhir-sqlite-store` are lowercase
# and never match the word-bounded, case-sensitive marks). Fuzz crates are
# publish = false and have no description; they are skipped by that field,
# not by name.

def package_manifests():
    out = []
    for dirpath, dirnames, filenames in os.walk("."):
        dirnames[:] = [d for d in dirnames if d not in SKIP_DIRS]
        if "Cargo.toml" in filenames:
            out.append(os.path.normpath(os.path.join(dirpath, "Cargo.toml")))
    return sorted(out)

for path in package_manifests():
    text = open(path, encoding="utf-8").read()
    if not re.search(r"(?m)^\[package\]", text):
        continue  # workspace-only manifest
    if re.search(r"(?m)^publish\s*=\s*false", text):
        continue  # not publishable; crates.io never renders it
    m = re.search(r'(?m)^description\s*=\s*"(.*)"\s*$', text)
    problems = []
    if m is None:
        problems.append("publishable crate has no description")
        desc = ""
    else:
        desc = m.group(1)
    used = []
    for mark in MARKS:
        span = None
        for mm in re.finditer(r"\b%s\b" % mark, desc):
            span = mm.span()
            break
        if span is None:
            continue
        used.append(mark)
        if desc[span[1]:span[1] + 1] != "®":
            problems.append("description: first use of %s is not %s®" % (mark, mark))
    if desc and DISCLAIMER not in desc:
        problems.append("description omits the required disclaimer")

    if problems:
        failures += 1
        print("FAIL %s" % path)
        for p in problems:
            print("       %s" % p)
    elif LIST:
        print("ok   %s (description%s)" %
              (path, ": " + ", ".join(used) if used else ", disclaimer only"))

if failures:
    print("\n%d document(s) do not meet spec/hl7-trademarks-fair-use/." % failures)
    print("Fix: add ® after the first prose use, and append the disclaimer:\n")
    print("    ## Trademarks\n")
    print("    " + DISCLAIMER)
    sys.exit(1)

print("All documents meet spec/hl7-trademarks-fair-use/.")
PY
