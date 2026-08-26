# ai-skills documentation

This directory contains the detailed engineering and operational documentation for `ai-skills`. The project README stays focused on the problem, product model and quickest path for users; implementation detail belongs here.

## Start here

- [Open-source readiness](open-source-readiness.md), minimum bar before v0 is presented as ready.
- [Planning model](planning.md), GitHub Issues, labels, native relationships and milestones without GitHub Projects.
- [Branching and promotion](branching-strategy.md), issue branches merge into `dev`, then completed milestones promote from `dev` to `main`.
- [Release policy](releasing.md), tag-driven releases and build-once/repackage-many rules.
- [Security policy](../SECURITY.md), current trust and mutation invariants.
- [Contributing](../CONTRIBUTING.md), contributor workflow.
- [Agent instructions](../AGENTS.md), rules for AI coding agents working on the repository.

## Architecture

Architecture documents and ADRs will be added alongside implementation. They should describe verified behavior and durable decisions rather than speculative code structure.

- [Rust workspace](architecture/workspace.md), crate responsibilities, dependency direction, and safety lint policy.

Planned areas include:

- domain and workspace boundaries;
- canonical Git/Markdown library and derived SQLite index;
- search and embeddings;
- MCP and HTTP contracts;
- `.ai-skills`, trust and materialization;
- agent adapters;
- proposal/approval and third-party overlays;
- Web dashboard;
- security, operations and recovery.

The implementation roadmap is tracked in Epic [#3](https://github.com/Alexandre-Tortoza/ai-skills/issues/3).
