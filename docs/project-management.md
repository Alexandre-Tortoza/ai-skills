# GitHub project model

The canonical delivery board is the single GitHub Project **ai-skills Roadmap**.

Issue bodies are technical specifications. Planning data belongs to native GitHub fields and relationships instead of being repeated in Markdown.

## Native hierarchy and dependencies

- Epic #3 is the parent of the v0 implementation issues through GitHub sub-issues.
- Prerequisites use GitHub `blocked by` / `blocking` relationships.
- Milestones are assigned through the issue's native Milestone field.
- The reproducible relationship input is `.github/project/planning.yml`.
- `.github/workflows/planning-sync.yml` materializes declared parent/dependency relationships using the GitHub Issues API.

The relationship workflow is additive for dependencies so a maintainer-added relationship is not silently removed. Parent assignment is authoritative for managed roadmap issues.

## Project fields

| Field | Type | Values / semantics |
| --- | --- | --- |
| Status | Built-in | Backlog, Ready, In progress, In review, Blocked, Done |
| Priority | Single select | P0, P1, P2 |
| Area | Single select | Core, Library, Storage, Search, MCP, API, CLI, Sync, Agents, LLM, Review, Web, Security, DevOps, Docs |
| Weight | Number | Fibonacci story points: 1, 2, 3, 5, 8, 13; 21 for Epics or unsliced work |
| Estimate (days) | Number | Focused implementation-day estimate, separate from story weight |
| Target release | Single select | v0, Post-v0 |
| Start date | Date | Scheduled start when planning becomes concrete |
| Target date | Date | Scheduled completion when planning becomes concrete |
| Milestone | Built-in issue field | Delivery phase M0-M8 |

**Weight is not elapsed time.** It represents relative complexity, uncertainty, integration surface and delivery risk. Executable issues should normally stay at 8 points or below; 13 requires decomposition review; 21 is reserved for Epics or intentionally unsliced discovery work.

## Issue anatomy

Implementation issue bodies should contain only information needed to understand and verify the engineering work:

- problem/context and measurable objective;
- in-scope/out-of-scope boundaries where useful;
- architecture/implementation guidance;
- deliverables;
- acceptance criteria / Definition of Done;
- test/evaluation strategy;
- security, operational and rollback risks;
- documentation impact and external references.

Do **not** repeat parent, dependencies, priority, area, weight, milestone, release or scheduling metadata in issue prose once the native fields/relationships exist.

## Recommended Project views

1. **Roadmap**, grouped by Milestone and Target release.
2. **Delivery board**, grouped by Status and filtered to open work.
3. **Architecture**, grouped by Area.
4. **Critical path**, filtered to Priority = P0.
5. **Security**, filtered to Area = Security.
6. **Release v0**, filtered to Target release = v0.
7. **High uncertainty**, filtered to Weight >= 13.
8. **Timeline**, using Start date / Target date once milestone scheduling is committed.

GitHub does not expose every view customization through `gh project`; create/refine visual views in the Project UI while keeping fields/items reproducible through the bootstrap workflow.

## Project bootstrap

`.github/workflows/project.yml` creates/links the single **ai-skills Roadmap**, creates the planning fields, adds all roadmap issues and writes Status/Priority/Area/Weight/Estimate/Target release from `.github/project/planning.yml`.

Because the Project belongs to the maintainer's personal GitHub account, bootstrap uses a temporary classic PAT stored as repository secret `PROJECTS_TOKEN` with the `project` scope. Run **Bootstrap and sync GitHub Project** once, verify the Project, then remove the token. Fine-grained PATs do not currently support user-owned Projects.

After a successful Project migration, the workflow removes legacy `priority:*`, `area:*`, `weight:*` and `status:*` labels. `.github/labels.yml` intentionally retains only issue classification labels so planning has one source in the Project rather than competing representations.
