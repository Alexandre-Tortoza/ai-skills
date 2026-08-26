# Rust workspace

Issue [#5](https://github.com/Alexandre-Tortoza/ai-skills/issues/5) establishes this workspace as the initial executable boundary for `ai-skills`.

## Dependency direction

`ai-skills-core` contains framework-independent domain semantics and application ports. It has no crate dependencies. All adapter crates may depend on `ai-skills-core`, but no adapter may be imported by `ai-skills-core`.

| Crate | Responsibility | Direct workspace dependencies |
| --- | --- | --- |
| `ai-skills-core` | Domain types, invariants, and application ports | None |
| `ai-skills-store` | Derived persistence adapters | `ai-skills-core` |
| `ai-skills-library` | Canonical Git/Markdown library adapters | `ai-skills-core` |
| `ai-skills-search` | Lexical and semantic retrieval adapters | `ai-skills-core` |
| `ai-skills-source` | Local, Git, and GitHub source adapters | `ai-skills-core` |
| `ai-skills-sync` | Repository resolution and materialization adapters | `ai-skills-core` |
| `ai-skills-agents` | Agent-specific integration adapters | `ai-skills-core` |
| `ai-skills-llm` | LLM provider adapters | `ai-skills-core` |
| `ai-skills-review` | Evidence, proposal, and review adapters | `ai-skills-core` |
| `ai-skills-mcp` | MCP transport adapters | `ai-skills-core` |
| `ai-skills-web` | HTTP and web adapters | `ai-skills-core` |
| `ai-skills-cli` | Composition root and command-line binary | `ai-skills-mcp`, `ai-skills-sync`, `ai-skills-web` |

The CLI is the top-level composition root. It may select concrete adapters but must not move domain behavior into transports or the binary itself.

The workspace has no external dependencies yet. Subsequent issues introduce them only in the adapter or composition crate that needs them; `ai-skills-core` remains independent of storage, HTTP, MCP, and provider libraries.

## Safety policy

The workspace sets `unsafe_code = "forbid"`. Every crate opts into the workspace lint policy, so introducing `unsafe` requires an explicit future architectural decision.

## Verification

The foundation baseline is verified with:

```sh
cargo check --workspace --all-targets
cargo test --workspace
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
```

The CLI integration test invokes `ai-skills --help`, ensuring the workspace produces the named executable.
