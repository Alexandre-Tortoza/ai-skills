# ADR 0001: Canonical content and human approval

## Status

Accepted

## Context

`ai-skills` stores reusable agent behavior that may originate from users, organizations, third parties, and machine review. The system needs a durable source of truth that is inspectable outside the application, while derived search and persistence state must remain rebuildable.

Machine-generated changes can alter agent behavior. A machine, MCP client, repository configuration, or background review process cannot be an approval authority for its own mutation.

## Decision

- Canonical skills and wiki content are Markdown/`SKILL.md` files versioned in Git. SQLite, FTS, embeddings, and caches are derived state.
- Third-party skills retain ownership and source provenance independently from trust policy.
- Machine-originated changes produce a `ChangeProposal` with evidence and remain pending until a `HumanActorId` explicitly approves it.
- Applying a proposal requires an `ApprovedProposal` proof created by that human approval. Adapter ports must reject proposals without that proof.
- Repository-controlled `.ai-skills` configuration, imported content, provider output, and review evidence remain untrusted inputs. They cannot create human approval authority.

## Consequences

- Adapters can rebuild indexes from canonical Git content after storage loss or drift.
- Search rank, provider trust, and source ownership remain separate from authorization to activate or mutate content.
- Mutation flows need proposal, approval, and audit surfaces before machine-originated changes can reach the canonical library.
- Human-authored direct edits remain possible through the canonical library workflow; automated mutation paths must use the proposal gate.

## Rejected alternatives

- Using SQLite or embeddings as the canonical durable store would make recovery and human review dependent on derived state.
- Allowing an LLM or MCP tool to apply a proposal it created would collapse the authority boundary and permit silent behavior changes.
- Treating a repository request or third-party provenance as trust would let untrusted content authorize itself.
