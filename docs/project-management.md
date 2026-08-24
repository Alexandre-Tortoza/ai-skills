# GitHub project model

The canonical delivery board is **ai-skills Roadmap** using GitHub Projects v2.

## Planning fields

| Field | Type | Values / semantics |
| --- | --- | --- |
| Status | Single select | Backlog, Ready, In progress, In review, Blocked, Done |
| Priority | Single select | P0, P1, P2 |
| Area | Single select | Core, Library, Storage, Search, MCP, API, CLI, Sync, Agents, LLM, Web, Security, DevOps |
| Weight | Number | Fibonacci story points: 1, 2, 3, 5, 8, 13; 21 for Epics or work requiring decomposition |
| Target release | Single select | v0.1, Post-v0.1 |
| Start date | Date | Planned start |
| Target date | Date | Planned completion |
| Milestone | Built-in | Delivery phase M0-M8 |

**Weight is not elapsed time.** It represents relative complexity, uncertainty, integration surface and delivery risk. Every issue also carries a separate time estimate in its body. Executable issues should normally be <= 8 points; 13 requires decomposition review; 21 is reserved for Epics or deliberately unsliced discovery work.

## Required issue anatomy

Implementation issues should contain: context/problem, measurable objective, in-scope and out-of-scope boundaries, architecture/implementation guidance, deliverables, acceptance criteria, dependencies/blocked-by/blocks, Fibonacci weight, time estimate, test strategy, security/risk analysis, documentation impact and definition of done.

Use GitHub task lists and cross-references (`#123`, `Closes #123`, `Blocked by #123`) so relationships remain navigable even outside Projects. The Epic maintains a checklist of its child issues and milestones provide phase-level completion tracking.

## Recommended views

1. **Roadmap**, grouped by milestone/target release.
2. **Delivery board**, grouped by Status and filtered to open work.
3. **Architecture**, grouped by Area.
4. **Critical path**, filtered to Priority = P0.
5. **Security**, filtered to Area = Security or Type = security.
6. **Release v0.1**, filtered to Target release = v0.1.
7. **High uncertainty**, filtered to Weight >= 13.

The repository workflow `.github/workflows/project-v2.yml` can create/link the Project when a `PROJECTS_TOKEN` secret with GitHub Projects v2 authorization is configured. The repository `GITHUB_TOKEN` is intentionally not treated as authority over the owner's user-level project.
