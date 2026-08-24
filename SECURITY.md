# Security Policy

## Reporting vulnerabilities

Do not disclose exploitable vulnerabilities through public issues. Use GitHub Security Advisories for private reporting:

https://github.com/Alexandre-Tortoza/ai-skills/security/advisories/new

## Core security invariants

- Repository-controlled `.ai-skills` configuration is untrusted input.
- Third-party skills are untrusted until allowed by user/global trust policy.
- LLM-generated skill changes never apply silently; they require a reviewable proposal, diff, provenance, and explicit user approval.
- MCP mutation tools must use the same authorization, validation, proposal and audit path as the dashboard and CLI.
- Canonical skill content is versioned and recoverable.
- Secrets must never be persisted into skill content, logs, embeddings, or proposal evidence.
