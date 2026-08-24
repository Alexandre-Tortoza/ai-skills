# GitHub planning model

`ai-skills` intentionally does **not** use GitHub Projects. The repository planning model is built from GitHub Issues, Milestones, labels, sub-issues and native dependency relationships.

Issue bodies remain technical specifications. Planning metadata is kept out of prose when GitHub already has a structured representation for it.

## Sources of truth

| Concern | Source of truth |
| --- | --- |
| Epic hierarchy | GitHub parent/sub-issue relationships |
| Dependencies | GitHub `blocked by` / `blocking` relationships |
| Delivery phase | GitHub Milestone |
| Priority | `priority:P0/P1/P2` label |
| Technical area | `area:*` label |
| Relative effort | Fibonacci `weight:*` label |
| Triage / visible blocking state | `status:triage` / `status:blocked` labels |
| Focused engineering-day estimate | `.github/planning.yml` |
| Detailed requirements and acceptance criteria | Issue body |
| Reproducible planning metadata | `.github/planning.yml` |

The GitHub Projects feature is disabled for this repository. No Project token or Project bootstrap workflow is used.

## Native hierarchy and dependencies

- Epic #3 is the parent of the v0 implementation backlog through GitHub sub-issues.
- Nested work may use another managed issue as parent, for example distribution work under #64.
- Prerequisites use GitHub `blocked by` / `blocking` relationships.
- Milestones group work into delivery phases M0-M8.
- `.github/workflows/planning-sync.yml` materializes the relationships declared in `.github/planning.yml`.

Dependency synchronization is additive: manually added dependencies are not silently removed. Parent assignment is authoritative for issues managed by the planning manifest.

## Labels

Planning labels are deliberately small and composable:

- `priority:P0`, `priority:P1`, `priority:P2`;
- `area:core`, `area:library`, `area:storage`, `area:search`, `area:mcp`, `area:api`, `area:cli`, `area:sync`, `area:agents`, `area:llm`, `area:review`, `area:web`, `area:security`, `area:devops`, `area:docs`;
- Fibonacci weights `weight:1`, `weight:2`, `weight:3`, `weight:5`, `weight:8`, `weight:13`, `weight:21`;
- `status:triage` and `status:blocked` only when useful.

**Weight is not elapsed time.** It captures relative complexity, uncertainty, integration surface and delivery risk. Executable work should normally remain at 8 points or below; 13 should trigger decomposition review; 21 is reserved for Epics or intentionally unsliced work.

The focused `estimate_days` value remains in the planning manifest because GitHub Issues do not provide a native numeric estimate field without introducing GitHub Projects. Story weight and time estimate intentionally remain separate concepts.

## Automation

Two permanent workflows maintain the planning model:

1. `.github/workflows/governance.yml` creates and updates the canonical label taxonomy and milestones from `.github/labels.yml` and `.github/milestones.yml`.
2. `.github/workflows/planning-sync.yml` synchronizes parent/sub-issue relationships, dependencies, priority, area and Fibonacci weight from `.github/planning.yml`.

The workflows use the repository-scoped `GITHUB_TOKEN`; no personal access token is required.

## Issue anatomy

Implementation issues should contain only the information needed to understand and verify the engineering work:

- problem/context and measurable objective;
- in-scope/out-of-scope boundaries where useful;
- architecture or implementation guidance;
- deliverables;
- acceptance criteria / Definition of Done;
- test/evaluation strategy;
- security, operational and rollback risks;
- documentation impact and external references.

Do not duplicate parent relationships, dependency relationships, milestone, priority, area or weight in the Markdown body.

## Working with the backlog

Useful GitHub filters include:

- critical path: `is:issue is:open label:priority:P0`;
- security work: `is:issue is:open label:area:security`;
- high uncertainty: `is:issue is:open label:weight:13` or `label:weight:21`;
- blocked items: `is:issue is:open label:status:blocked`;
- milestone-specific work through the native Milestone selector.

This keeps the entire planning system repository-native, searchable, automatable and usable by contributors without requiring a separate board or account-level Project permissions.
