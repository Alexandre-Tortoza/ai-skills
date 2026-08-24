# Contributing to ai-skills

Thank you for considering a contribution. `ai-skills` is currently pre-alpha, so issue scope and architecture contracts matter more than preserving accidental implementation details.

## Before starting

- Read `README.md` and `docs/index.md`.
- Search existing issues before opening new work.
- For non-trivial changes, start from or create an issue with objective, scope, acceptance criteria, validation strategy and risks.
- Use the Issue Forms in `.github/ISSUE_TEMPLATE/` rather than free-form planning when possible.

Priority, area, Fibonacci weight, delivery status, target release and dates belong in GitHub Projects v2. Parent/sub-issue and blocked-by relationships belong in GitHub's native issue relationships. Do not duplicate these planning fields in the issue body.

## Development workflow

1. Create a focused branch such as `feat/...`, `fix/...`, `refactor/...`, `docs/...` or `chore/...`.
2. Keep one pull request centered on one reviewable outcome.
3. Link the issue with `Closes #<number>` when the PR fully resolves it.
4. Add or update automated tests for changed behavior and failure cases.
5. Update documentation and `CHANGELOG.md` when behavior visible to users changes.
6. Record durable architectural decisions as ADRs under `docs/adr/` once the ADR process is established.

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
