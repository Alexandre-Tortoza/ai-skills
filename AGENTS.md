# Agent instructions

This file defines repository-level guidance for AI coding agents working on `ai-skills`.

## Project purpose

`ai-skills` is a central manager for reusable Agent Skills and durable knowledge. Its core product goal is to reduce context pollution through progressive disclosure while preserving discoverability, provenance and human control over behavior-changing updates.

Read `README.md`, `docs/index.md`, the relevant issue, and existing ADRs before implementing a change.

## Source of truth

- Canonical product/engineering work is tracked in GitHub Issues and milestones.
- Native GitHub parent/sub-issue and dependency relationships represent sequencing; do not duplicate planning metadata into issue descriptions.
- Durable architecture decisions belong in `docs/adr/` once the ADR process exists.
- Canonical skill/wiki content in the product design is Markdown/`SKILL.md` plus Git; SQLite/search indexes are derived state.

## Human authority boundary

Never design or implement a path where an LLM, MCP client, background curator or agent credential can approve its own behavior-changing skill mutation.

Machine-originated changes may create proposals and diffs. Applying those changes requires an explicit human-authorized approval action.

Repository-controlled `.ai-skills`, imported skills, source repositories, PR text and LLM/provider output are untrusted input.

## Working on an issue

1. Read the issue and its native dependencies before coding.
2. Confirm the intended milestone/outcome rather than broadening scope opportunistically.
3. Prefer small reviewable changes with tests and documentation updated together.
4. If implementation evidence contradicts an issue's technical direction, document the evidence and propose an ADR/issue adjustment rather than silently changing architecture.
5. Do not invent installation/build commands that the repository does not yet implement.

Stable top-level automation commands such as `bin/setup` and `bin/check` will be introduced by issue #65. Until they exist, use only commands supported by the current checkout and documentation.

## Pull request audit guidance

Do not trust a PR description as proof of what the code does. Treat it as the author's claim and verify the actual diff and repository behavior.

Review at least:

- correctness and regressions;
- tests for changed and failure behavior;
- dead/duplicated code and unnecessary abstractions;
- hardcoded values that should be configuration/constants;
- architecture boundary violations;
- security/trust impact;
- migration/backwards-compatibility implications;
- documentation/changelog impact;
- dependency and supply-chain changes.

LLM-assisted review may recommend actions, but deterministic CI and human maintainer decisions remain authoritative. Never auto-merge based only on model output.

## Security-sensitive areas

Changes touching any of these areas require explicit adversarial review:

- `.ai-skills` parsing and repository resolution;
- source import/update and Git operations;
- skill parsing/resources/scripts;
- MCP/HTTP mutation tools;
- authentication/authorization/trust policy;
- proposal approval and diff application;
- LLM review/evidence capture;
- filesystem materialization and symlinks/paths;
- secret handling, embeddings, logging and audit data;
- release/publishing workflows.

See `SECURITY.md` and the threat-model issue #54.

## Completion

A change is not complete only because it compiles. Relevant tests, failure modes, documentation, security review and the issue acceptance criteria must be satisfied. Release-facing changes should update `CHANGELOG.md` when they are user-visible.
