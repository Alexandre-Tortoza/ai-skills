# Contributing to ai-skills

Thank you for considering a contribution. `ai-skills` is currently pre-alpha, so issue scope and architecture contracts matter more than preserving accidental implementation details.

## Before starting

- Read `README.md` and `docs/index.md`.
- Search existing issues before opening new work.
- For non-trivial changes, start from or create an issue with objective, scope, acceptance criteria, validation strategy and risks.
- Use the Issue Forms in `.github/ISSUE_TEMPLATE/` rather than free-form planning when possible.

Priority, area, Fibonacci weight, delivery status, target release and dates belong in the single GitHub Project **ai-skills Roadmap**. Parent/sub-issue and blocked-by relationships belong in GitHub's native issue relationships. Do not duplicate these planning fields in the issue body.

## Development workflow

The repository uses the promotion flow documented in [`docs/branching-strategy.md`](docs/branching-strategy.md):

```text
feature/<name> | fix/<name> | hotfix/<name> -> dev -> main
```

`main` is production/stable and `dev` is the normal integration target.

1. Start from the current `dev` branch for normal work.
2. Create one focused `feature/<name>`, `fix/<name>`, or `hotfix/<name>` branch for each issue.
3. Open the pull request against `dev`. External fork pull requests also target `dev`.
4. Keep one pull request centered on that issue's reviewable outcome.
5. Link the issue with `Closes #<number>` when the PR fully resolves it.
6. Add or update automated tests for changed behavior and failure cases.
7. Update documentation and `CHANGELOG.md` when behavior visible to users changes.
8. Record durable architectural decisions as ADRs under `docs/adr/` once the ADR process is established.
9. Merge into `dev` may be automated once required checks pass; it requires no approving review.
10. GitHub deletes the merged issue branch; remove its local branch after fetching `dev`.
11. After every issue in a milestone is merged into `dev`, open a `dev -> main` promotion pull request.
12. A human must review and manually merge the `dev -> main` promotion pull request.

Do not open issue pull requests directly against `main`.

Standard repository commands such as `bin/setup` and `bin/check` are tracked by #65. Until they exist, follow commands supported by the current checkout; do not introduce undocumented personal scripts as required contributor steps.

## Pull requests

The PR description should explain intent and validation, but reviewers must verify the code itself. A description is not evidence that the implementation is correct.

Before requesting review, verify:

- relevant tests pass;
- formatting/lint/static analysis pass when available;
- public configuration/API/schema changes are documented;
- migration and compatibility behavior is explicit;
- security/trust boundaries are considered;
- no secrets, local paths or maintainer-specific deployment values are committed;
- user-visible changes are reflected in the changelog when appropriate.

## Definition of Done

A change is complete when the issue's acceptance criteria are objectively met, critical behavior and failure modes are covered, relevant documentation is updated, and security/trust implications have been reviewed.

Passing CI is necessary but does not replace code review.

## Security-sensitive changes

Changes involving `.ai-skills`, third-party skill sources, MCP write tools, LLM-generated proposals, authentication, authorization, filesystem materialization, Git synchronization, executable skill assets or release credentials require explicit review of trust boundaries and untrusted input handling.

Do not post vulnerabilities in public issues. Follow [SECURITY.md](SECURITY.md).

## Contribution licensing

Unless explicitly stated otherwise, contributions submitted to this repository are licensed under the same MIT license as the project.

## Conduct

Participation is governed by [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).
