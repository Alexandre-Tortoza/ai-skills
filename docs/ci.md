# CI policy

Every pull request and push to `dev` or `main` runs the Rust quality/test and dependency-advisory jobs. The workflow uses read-only GitHub permissions, Cargo caches only registries, Git dependencies, and build output, and cancels superseded runs for the same ref.

Required checks for protected branches are `Validate promotion path` and the stable CI jobs once the rulesets are updated through #66:

- Rust quality and tests;
- Dependency advisories;
- Secret scan.

The frontend has no implementation yet. Its validation job will be added when `web/` exists rather than creating a non-running required check.

`cargo audit` reports known RustSec advisories. License and deny-list policy will be added with the first non-trivial third-party runtime dependency if `cargo-deny` supplies additional value beyond advisory scanning.
