# Release policy

`ai-skills` releases are intended to be automated, tag-driven and reproducible.

The implementation is tracked by #59, #60 and #64. This document defines the contract those issues must satisfy.

## Versioning

Use Semantic Versioning. During pre-1.0 development, incompatible changes are still documented explicitly because users and agent integrations depend on configuration/protocol stability.

The initial product roadmap target is **v0**. Concrete software releases within that phase may use normal SemVer tags such as `v0.1.0`, `v0.2.0`, and so on.

User-visible changes should be recorded under `[Unreleased]` in `CHANGELOG.md` as they land.

## Trigger

Stable publication is triggered from a reviewed SemVer tag, for example `v0.1.0`.

A maintainer/agent command may prepare version files, changelog and the tag, but the GitHub Actions release workflow is the publication authority.

## Build once, repackage many

For each supported platform/architecture:

1. Build and test the canonical binary on the appropriate trusted runner/toolchain.
2. Archive it with deterministic naming and publish its SHA-256 checksum as a workflow artifact.
3. Reuse that artifact downstream for direct downloads, container images and package-manager formats.
4. Do not independently rebuild application source inside Homebrew/AUR/Docker packaging jobs unless a package's explicit source-build variant requires it.

This reduces CI cost and ensures package surfaces distribute the same reviewed bytes.

## Release contents

A supported release should include, as applicable:

- platform binary archives;
- checksums;
- SBOM;
- build provenance/attestation where supported;
- container images;
- package-manager updates;
- release notes extracted/derived from the changelog;
- installation and verification instructions;
- known limitations/support matrix.

## Safety

Publishing jobs run only after mandatory tests/security/evals pass and receive the smallest permissions/credentials necessary. Untrusted pull-request code must not execute with release credentials.

Release reruns should be idempotent where registries support it and produce explicit recovery instructions where publication is immutable/non-repeatable.

## Release candidate gate

Issue #63 is the final v0 roadmap release gate. Any code/config change after a candidate's validation invalidates the affected evidence and requires rerunning the relevant checks.
