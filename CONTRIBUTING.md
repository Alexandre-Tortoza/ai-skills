# Contributing to ai-skills

## Workflow

1. Start from an issue when the change is non-trivial.
2. Keep issues scoped to one reviewable outcome.
3. Use conventional branches such as `feat/...`, `fix/...`, `refactor/...`, `docs/...`, or `chore/...`.
4. Open a pull request and link the issue with `Closes #<number>` when appropriate.
5. Keep durable architectural decisions in ADRs under `docs/adr/`.

## Definition of Done

A change is complete when its acceptance criteria are met, automated tests cover critical behavior, relevant documentation is updated, and security/trust implications have been reviewed.

## Security-sensitive areas

Changes involving `.ai-skills`, third-party skill sources, MCP write tools, LLM-generated proposals, authentication, authorization, filesystem materialization, Git synchronization, or executable skill assets require explicit review of trust boundaries and untrusted input handling.
