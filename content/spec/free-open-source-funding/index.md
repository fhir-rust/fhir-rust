# Free open source funding

- Set up GitHub Sponsors.
- Set up Open Collective.
- Add .github/FUNDING.yml
- Update CONTRIBUTING.md to match
- Update NEWS.md to match

## Status in this repository

Implemented 2026-08-28, checked against each platform's own API rather than
assumed:

- **GitHub Sponsors: done.** Verified live via the GitHub GraphQL API
  (`user(login: "joelparkerhenderson") { sponsorsListing { isPublic } }`
  returns `true`) — this predates this pass, not created by it.
- **Open Collective: not done, and not something an agent should do on the
  maintainer's behalf.** Checked against Open Collective's own GraphQL API:
  no collective exists at `joelparkerhenderson` or `fhir-rust` (both queries
  return `Collective Not Found`). Creating one needs the maintainer's own
  sign-in and a fiscal-host choice — left as a real open item rather than
  worked around or silently dropped from the list.
- **`.github/FUNDING.yml`: done.** `github`, `patreon`, `ko_fi`, and a
  `custom` PayPal link — the four that are real. No `open_collective:` entry,
  because adding one before the collective exists would put a broken button
  on the repository page.
- **`CONTRIBUTING.md` and `NEWS.md`: done**, and both say Open Collective is
  outstanding rather than staying silent about it — matching this
  repository's rule that a document must not claim more than the evidence
  supports (`C0.11`).

The Open Collective item stays open until the maintainer does it. This
document is not a place to invent a workaround.
