# Trusted Publishing

Trusted Publishing is a secure way to publish your Rust crates from CI/CD platforms like GitHub Actions and GitLab CI/CD without manually managing API tokens. It uses OpenID Connect (OIDC) to verify that your workflow is running from your repository, then provides a short-lived token for publishing.

Instead of storing long-lived API tokens in your repository secrets, Trusted Publishing allows your CI/CD platform to authenticate directly with crates.io using cryptographically signed tokens that prove the workflow's identity.

We intend to add "Trusted Publishing" when it is production-ready across all our code forges (GitHub.com, GitLab.com, Codeberg.org, etc.) and across all our target destinations (Rust crates.io, NPM npmjs.com, etc.).

## Status in this repository

Checked 2026-08-28, against each registry's own documentation rather than
assumed, and reconciled against a decision this repository had already made.

**External readiness — the condition this file states.**

| Forge | crates.io Trusted Publishing | Source |
| --- | --- | --- |
| GitHub Actions | **Generally available.** First platform supported, per [RFC 3691](https://rust-lang.github.io/rfcs/3691-trusted-publishing-cratesio.html) | [crates.io Trusted Publishing docs](https://crates.io/docs/trusted-publishing) |
| GitLab CI/CD | **Available — GitLab.com only.** Self-hosted GitLab instances are explicitly not supported yet | [crates.io development update, 2026-01-21](https://blog.rust-lang.org/2026/01/21/crates-io-development-update) |
| Codeberg.org (Forgejo) | **Not available.** crates.io's implementation is refactored to support more providers, but Forgejo itself is still building the OIDC short-lived-token issuance Trusted Publishing needs; nothing has shipped | [Forgejo issue #9939](https://codeberg.org/forgejo/forgejo/issues/9939) |

This repository mirrors to exactly the three forges named in this file's
opening paragraph — GitHub (canonical), GitLab.com, and Codeberg.org — so the
condition as stated (*all* forges) is not met: Codeberg/Forgejo support does
not exist yet, upstream, regardless of anything this repository could do.

npm is not applicable here: `fhir-rust` publishes no npm packages, so that
half of the stated condition is vacuous for this repository specifically
(it may still matter for a sibling project that does).

**The condition that actually governs this repository is a different, more
specific one, and it was already decided before this file was read closely
against it.** [`spec/publishing.md`](../publishing.md) and
[`MAINTAINERS.md`](../../MAINTAINERS.md) record that the owner ruled out
publishing from CI **entirely, on 2026-08-26** — not on Trusted Publishing's
own readiness, but on GitHub Actions' reliability, a judgment made hours
after a GitHub Actions major outage stalled every hosted run. Trusted
Publishing is structurally a CI-workflow-authentication mechanism: there is
no "Trusted Publishing from a laptop." Ruling out CI as the publish path
therefore rules out Trusted Publishing as a mechanism for this repository,
independent of whether crates.io supports the forge — the GitHub Actions
support in the table above is real and does not change this repository's
answer, because this repository is not publishing from GitHub Actions at
all, by decision, not by gap.

**So: this file's stated intent is not wrong, and it is not this
repository's current plan.** It describes a reasonable default for a project
that publishes from CI. This one does not, on a decision recorded elsewhere
and dated later than this file's generic wording. Nothing here is deleted,
because the intent may still be correct for a sibling project; but a reader
of this repository should follow [`spec/publishing.md`](../publishing.md) for
what actually happens, not this page's stated intent. That document already
notes the route back: `RFC.md` §10 solicits exactly the evidence that would
reopen the question — for either reason, forge readiness or a change in
GitHub Actions' reliability track record.
