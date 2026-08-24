# Open-source readiness

`ai-skills` should not be presented as a ready open-source tool merely because its core implementation works. The release bar is deliberately user-facing: a new person must be able to understand the problem, install the tool with little friction, trust contributions through automated validation, and find the design/operations documentation needed to work on it.

This policy is influenced by Fabio Akita's article [Boas práticas de projetos de código aberto com LLM - O Mínimo](https://akitaonrails.com/2026/05/30/boas-praticas-projetos-codigo-aberto-llm-o-minimo/) and the engineering patterns visible in projects such as `ai-memory` and `ai-jail`.

## Minimum release pillars

### 1. Installation surface

A release must not require a curious user to reconstruct the build toolchain just to try the product.

Required for the first supported release:

- prebuilt direct-download artifacts for advertised platforms;
- clear one-command package-manager paths where supported;
- checksums and release provenance;
- container/runtime installation for server use;
- smoke-tested instructions;
- explicit support matrix instead of unverified platform claims.

The distribution pipeline follows **build once, repackage many**: one verified binary per target becomes the input to tarballs, containers and package-manager packaging rather than recompiling independently for every surface.

Tracked by #59, #60 and #64.

### 2. Tests and CI

Pull requests need a common deterministic floor before subjective review.

The target CI baseline includes:

- formatting;
- lint/static analysis with warnings treated deliberately;
- unit/integration/contract/E2E tests as applicable;
- dependency/license/advisory checks;
- secret scanning;
- frontend validation once the dashboard exists;
- security and protocol regression tests;
- release-only longer fuzz/load/evaluation gates where appropriate.

Tracked primarily by #8, #27, #46, #54 and #58.

LLM review is additive, never a replacement for deterministic CI or human merge authority. Optional LLM-assisted PR auditing is tracked by #67.

### 3. Documentation that starts with the problem

The README should answer what the tool solves, who benefits and how to try it before discussing internal stack choices.

Detailed architecture, protocols, migrations, security and operations belong under `docs/`.

Final v0.1 documentation consolidation is tracked by #62.

## Predictable automation for humans and agents

Stable repository commands reduce both contributor friction and agent guesswork. Once the implementation exists, `bin/setup`, `bin/check` and release/development entry points should become documented interfaces rather than every agent inventing command sequences independently.

Tracked by #65.

## Release discipline

Stable releases are tag-driven. A release tag should trigger the publication pipeline; local maintainer commands may prepare/version/tag but should not contain hidden one-off publishing logic.

`CHANGELOG.md` follows Keep a Changelog and SemVer. GitHub Releases should explain what changed, how to install and how to verify artifacts.

See [releasing.md](releasing.md).

## GitHub-native governance

Use the platform rather than encoding planning into prose:

- milestones represent delivery phases;
- parent/sub-issue relationships represent Epic hierarchy;
- blocked-by/blocking relationships represent dependencies;
- Projects v2 fields hold priority, area, Fibonacci weight, estimate, dates/status and release targeting;
- labels are reserved for lightweight classification rather than duplicating Project fields.

Repository settings that cannot be represented entirely in Git are tracked by #66.

## Readiness rule

`v0.1.0` is not ready while any mandatory release gate in #63 remains unsatisfied. A polished core algorithm is not an exception to installation, CI, documentation, security or release integrity requirements.
