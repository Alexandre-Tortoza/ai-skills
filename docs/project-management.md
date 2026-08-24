# GitHub project model

The canonical delivery board is **ai-skills Roadmap** using GitHub Projects v2.

Recommended fields:

| Field | Type | Values |
| --- | --- | --- |
| Status | Single select | Backlog, Ready, In progress, In review, Blocked, Done |
| Priority | Single select | P0, P1, P2 |
| Area | Single select | Core, Library, Storage, Search, MCP, API, CLI, Sync, Agents, LLM, Web, Security, DevOps |
| Effort | Single select | XS, S, M, L, XL |
| Target release | Single select | v0.1, Post-v0.1 |
| Start date | Date | planning |
| Target date | Date | planning |

Recommended views:

1. **Roadmap**, grouped by milestone/target release.
2. **Delivery board**, grouped by Status and filtered to open work.
3. **Architecture**, grouped by Area.
4. **Critical path**, filtered to Priority = P0.
5. **Security**, filtered to Area = Security or Type = security.
6. **Release v0.1**, filtered to Target release = v0.1.

The repository workflow `.github/workflows/project-v2.yml` can create the project when a `PROJECTS_TOKEN` secret with GitHub Projects v2 authorization is configured. The repository `GITHUB_TOKEN` is intentionally not treated as authority over the owner's user-level project.
