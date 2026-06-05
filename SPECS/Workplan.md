# Agent Passport Workplan

Status: Draft
Created: 2026-06-05
Updated: 2026-06-05
Input: `drafts/agent-passport.md`, current Rust validator implementation, PR #4 quality-gate discussion

## Working Rules

- Treat passport YAML as untrusted input.
- Keep validation deterministic, local-first, and side-effect free unless an
  explicit command such as `--check-integrity` asks to read agent files.
- Separate RFC conformance errors from advisory security warnings.
- Do not invent cryptographic semantics that are not specified in the RFC.
- Prefer structured parsing and typed models over string matching.
- Keep generated or downloaded build artifacts out of the repository.
- Every implementation task must document and run its quality gates.
- Raise CI thresholds only after the matching test evidence exists.

## Phase 0. Rust Validator and CI Baseline

### ✅ AP-P0-T1 — Add Rust Agent Passport validator CLI

- **Description:** Add an importable Rust validator library and an
  `agent-passport validate` CLI for RFC-shaped YAML passports.
- **Priority:** P0
- **Dependencies:** none
- **Parallelizable:** no
- **Status:** Complete
- **Outputs / Artifacts:** `Cargo.toml`, `src/`, `tests/`, sample passport,
  README usage.
- **Acceptance Criteria:**
  - Complete: CLI validates required RFC fields, lifecycle timestamps,
    capabilities, resources, security policies, signature base64 syntax, and
    optional SHA-256/SHA-512 integrity hashes.
  - Complete: CLI supports human-readable and JSON output.
  - Complete: Unit and integration tests pass.

### ✅ AP-P0-T2 — Add Rust quality gates to CI

- **Description:** Configure GitHub Actions for local-equivalent quality gates.
- **Priority:** P0
- **Dependencies:** AP-P0-T1
- **Parallelizable:** yes
- **Status:** Complete
- **Outputs / Artifacts:** `.github/workflows/ci.yml`, `deny.toml`,
  `rust-toolchain.toml`, README quality-gate section.
- **Acceptance Criteria:**
  - Complete: CI runs `cargo fmt --check`.
  - Complete: CI runs `cargo clippy --all-targets -- -D warnings`.
  - Complete: CI runs `cargo test`.
  - Complete: CI runs `cargo llvm-cov --summary-only --fail-under-lines 60`.
  - Complete: CI runs `cargo audit` and `cargo deny check`.
  - Complete: CI runs a validator CLI smoke test.

## Phase 1. RFC Fidelity and Schema Contract

### ☐ AP-P1-T1 — Materialize Agent Passport JSON Schema

- **Description:** Add a machine-readable JSON Schema or YAML Schema that
  mirrors the RFC v1alpha1 structure and can validate examples independently
  from the Rust implementation.
- **Priority:** P0
- **Dependencies:** AP-P0-T1
- **Parallelizable:** yes
- **Status:** Planned
- **Outputs / Artifacts:** `schema/agent-passport.v1alpha1.schema.json`,
  schema validation tests, README/schema documentation.
- **Acceptance Criteria:**
  - Schema requires `passport.apiVersion`, `kind`, `metadata`, `spec.entity`,
    `spec.capabilities`, and `signature`.
  - Schema validates all committed valid examples.
  - Schema rejects invalid fixtures for missing required fields and wrong
    scalar/container types.
  - Rust model and schema are documented as two views of the same RFC contract.

### ☐ AP-P1-T2 — Add Valid and Invalid Fixture Matrix

- **Description:** Build a deterministic fixture suite for common RFC and
  security-policy cases.
- **Priority:** P0
- **Dependencies:** AP-P1-T1
- **Parallelizable:** yes
- **Status:** Planned
- **Outputs / Artifacts:** `fixtures/valid/`, `fixtures/invalid/`,
  fixture-driven tests.
- **Acceptance Criteria:**
  - Valid fixtures cover LLM agent, system agent, proxy agent, no-expiry
    passport, SHA-256 integrity, and SHA-512 integrity.
  - Invalid fixtures cover malformed YAML, missing top-level `passport`, wrong
    `kind`, expired passports, future `issueDate`, invalid hash encoding,
    invalid signature base64, invalid network ports, and missing capability
    signatures.
  - Fixture tests assert expected error paths, not only failure status.

### ☐ AP-P1-T3 — Define Strict, Lenient, and Advisory Validation Modes

- **Description:** Make validator policy explicit so implementers can choose
  RFC-compatible forward tolerance or strict issuer/runtime enforcement.
- **Priority:** P1
- **Dependencies:** AP-P1-T2
- **Parallelizable:** yes
- **Status:** Planned
- **Outputs / Artifacts:** CLI flags, validator options, tests, documentation.
- **Acceptance Criteria:**
  - `--mode lenient` preserves RFC extensibility and reports unknown fields only
    when useful.
  - `--mode strict` rejects unknown top-level and known-section fields for
    issuer/runtime use.
  - Advisory warnings remain distinguishable from hard validation errors in
    human and JSON output.

### ☐ AP-P1-T4 — Replace Bespoke Utility Checks with Vetted Crates

- **Description:** Audit hand-written utility validation code and replace it
  with mature Rust libraries where doing so improves correctness,
  maintainability, and auditability without expanding the trusted computing
  base unnecessarily.
- **Priority:** P1
- **Dependencies:** AP-P1-T2, AP-P1-T3
- **Parallelizable:** yes
- **Status:** Planned
- **Outputs / Artifacts:** dependency decision notes, focused refactor PR,
  regression tests.
- **Acceptance Criteria:**
  - UUID validation uses `uuid` instead of a custom UUID-like parser.
  - Semantic version validation uses `semver` instead of split-and-parse logic.
  - Hex encoding/decoding uses `hex` instead of local nibble parsing.
  - Path handling for integrity roots evaluates `camino` or a documented
    standard-library-only policy.
  - Diagnostic/error modeling evaluates `thiserror` and `miette` before adding
    stable diagnostic codes.
  - Enum/string parsing for protocols, data types, algorithms, and validation
    modes evaluates `strum` or typed enums with manual `FromStr`.
  - Any new dependency passes `cargo audit`, `cargo deny check`, license review,
    MSRV review, and feature-surface review.

## Phase 2. Test Quality and Regression Signal

### ☐ AP-P2-T1 — Raise Coverage Gate with Targeted Tests

- **Description:** Expand tests before raising coverage from the current 60%
  gate toward a production baseline.
- **Priority:** P0
- **Dependencies:** AP-P1-T2
- **Parallelizable:** yes
- **Status:** Planned
- **Outputs / Artifacts:** additional unit/integration tests, updated CI
  threshold.
- **Acceptance Criteria:**
  - Line coverage reaches at least 80%.
  - Region coverage reaches at least 80%.
  - CI coverage gate is raised only after local `cargo llvm-cov` confirms the
    new threshold.
  - Tests cover all public CLI exit-code classes: valid, invalid, and runtime
    usage/read errors.

### ☐ AP-P2-T2 — Add Golden CLI Output Snapshots

- **Description:** Lock human and JSON output contracts for common validation
  paths.
- **Priority:** P1
- **Dependencies:** AP-P1-T2
- **Parallelizable:** yes
- **Status:** Planned
- **Outputs / Artifacts:** snapshot fixtures, CLI output tests.
- **Acceptance Criteria:**
  - Golden snapshots cover valid, invalid, advisory-only, multi-file, and JSON
    output.
  - Snapshot updates require intentional review.
  - Error path formatting remains stable for downstream automation.

### ☐ AP-P2-T3 — Add Property Tests for Parser and Validator Invariants

- **Description:** Use property-based tests to check validator invariants over
  generated passport-like structures and malformed scalar values.
- **Priority:** P1
- **Dependencies:** AP-P1-T3
- **Parallelizable:** yes
- **Status:** Planned
- **Outputs / Artifacts:** `proptest` tests and reduced regression cases.
- **Acceptance Criteria:**
  - Validator never panics on arbitrary YAML-like input accepted by the parser.
  - Valid timestamp ordering is monotonic: moving `expiryDate` before
    `issueDate` always produces an error.
  - Hash decoding accepts only valid hex/base64 encodings with correct digest
    lengths.

### ☐ AP-P2-T4 — Add Fuzzing Harness for Untrusted YAML Input

- **Description:** Add `cargo fuzz` targets for YAML parsing and validation.
- **Priority:** P1
- **Dependencies:** AP-P2-T3
- **Parallelizable:** yes
- **Status:** Planned
- **Outputs / Artifacts:** fuzz target, seed corpus, fuzzing instructions.
- **Acceptance Criteria:**
  - Fuzz target exercises `validate_str` without filesystem access.
  - Seed corpus includes valid examples and invalid fixtures.
  - Crashes are minimized into regression fixtures before fixes are accepted.

### ☐ AP-P2-T5 — Add Mutation Testing Review Gate

- **Description:** Use mutation testing to identify weak assertions in validator
  behavior.
- **Priority:** P2
- **Dependencies:** AP-P2-T1
- **Parallelizable:** yes
- **Status:** Planned
- **Outputs / Artifacts:** `cargo-mutants` report and follow-up test tasks.
- **Acceptance Criteria:**
  - Mutation test run is documented and locally reproducible.
  - Surviving mutants in core validation branches are triaged.
  - High-value surviving mutants become test additions or explicit non-goals.

## Phase 3. Cryptographic Verification and Trust

### ☐ AP-P3-T1 — Specify Canonical Signed Payload Profile

- **Description:** Define the exact canonical byte representation used for
  passport signatures, including exclusion of `signature` and YAML/JSON
  normalization rules.
- **Priority:** P0
- **Dependencies:** AP-P1-T1
- **Parallelizable:** no
- **Status:** Planned
- **Outputs / Artifacts:** RFC section update, canonicalization test vectors.
- **Acceptance Criteria:**
  - Canonicalization is deterministic across independent implementations.
  - Test vectors include original document, canonical bytes, digest, and
    signature input.
  - RFC explicitly states whether YAML source formatting is signed or whether a
    normalized representation is signed.

### ☐ AP-P3-T2 — Implement Signature Verification

- **Description:** Add cryptographic verification for RFC-approved algorithms
  after canonicalization is specified.
- **Priority:** P0
- **Dependencies:** AP-P3-T1
- **Parallelizable:** no
- **Status:** Planned
- **Outputs / Artifacts:** verifier module, CLI flags, tests, signed examples.
- **Acceptance Criteria:**
  - Ed25519/EdDSA verification is supported with committed test vectors.
  - RSASSA-PSS-SHA256 verification is supported or explicitly deferred with an
    RFC-compatible reason.
  - Invalid signatures fail with clear diagnostics.
  - Verification never accepts a key from an untrusted source implicitly.

### ☐ AP-P3-T3 — Add Trust Store and Public Key Resolution Model

- **Description:** Define and implement a verifier trust-store model for PEM,
  JWK/JWKS, and DID-style public key references.
- **Priority:** P1
- **Dependencies:** AP-P3-T2
- **Parallelizable:** yes
- **Status:** Planned
- **Outputs / Artifacts:** trust-store config, resolver interface, tests.
- **Acceptance Criteria:**
  - CLI can verify against an explicit local trust store.
  - Network public-key resolution is disabled by default.
  - DID/JWKS references are parsed as identifiers unless a resolver is
    explicitly configured.
  - Diagnostics distinguish unknown issuer, missing key, unsupported key type,
    and signature mismatch.

### ☐ AP-P3-T4 — Add Revocation and Expiry Policy Hooks

- **Description:** Prepare lifecycle validation for future issuer revocation
  sources without hardcoding one revocation mechanism.
- **Priority:** P2
- **Dependencies:** AP-P3-T3
- **Parallelizable:** yes
- **Status:** Planned
- **Outputs / Artifacts:** revocation status interface, offline fixture tests.
- **Acceptance Criteria:**
  - Expiry validation is separated from revocation validation.
  - Revocation checks can run in offline fixture mode.
  - CLI reports unknown revocation status separately from revoked and valid.

## Phase 4. Security Policy and Runtime Semantics

### ☐ AP-P4-T1 — Validate Security Policy Consistency

- **Description:** Add semantic checks that compare declared resources against
  security policies for obvious contradictions or least-privilege risks.
- **Priority:** P1
- **Dependencies:** AP-P1-T3
- **Parallelizable:** yes
- **Status:** Planned
- **Outputs / Artifacts:** semantic validator rules and tests.
- **Acceptance Criteria:**
  - `resources.network` outbound entries are compared against
    `networkRestrictions.allowList` and `denyList`.
  - `privileged: true` remains a warning by default and can be promoted to an
    error in strict runtime mode.
  - Filesystem paths and executable paths are checked for absolute-path
    enforceability.

### ☐ AP-P4-T2 — Add Seccomp and Chroot Profile Validation Hooks

- **Description:** Validate referenced seccomp and chroot policy artifacts when
  a runtime-oriented mode is requested.
- **Priority:** P1
- **Dependencies:** AP-P4-T1
- **Parallelizable:** yes
- **Status:** Planned
- **Outputs / Artifacts:** CLI flags, filesystem validation tests.
- **Acceptance Criteria:**
  - Seccomp profile references can be checked for file existence and parseable
    JSON/YAML shape.
  - Chroot paths can be checked for existence and directory type.
  - Runtime artifact checks are opt-in and never run in ordinary structure-only
    validation.

### ☐ AP-P4-T3 — Add Agent Integrity Verification Profiles

- **Description:** Harden file integrity checking for source-file roots,
  absolute paths, symlink handling, and reproducible diagnostics.
- **Priority:** P1
- **Dependencies:** AP-P1-T2
- **Parallelizable:** yes
- **Status:** Planned
- **Outputs / Artifacts:** integrity-root policy docs, tests.
- **Acceptance Criteria:**
  - Relative `sourceFile` paths are resolved only under the declared integrity
    root.
  - Symlink behavior is documented and tested.
  - Missing files, unreadable files, and hash mismatches produce distinct
    diagnostics.

### ☐ AP-P4-T4 — Define `agentifyd` Validation Contract

- **Description:** Specify the exact validator API and expected diagnostics
  that `agentifyd` or a reference runtime will consume before launch.
- **Priority:** P1
- **Dependencies:** AP-P4-T1, AP-P4-T3
- **Parallelizable:** no
- **Status:** Planned
- **Outputs / Artifacts:** runtime contract document, integration fixtures.
- **Acceptance Criteria:**
  - Runtime-facing validation mode is documented separately from issuer-facing
    validation.
  - Diagnostics include stable machine-readable codes.
  - Integration fixtures show pass/fail behavior for a restricted legacy binary.

## Phase 5. Supply Chain, Release, and CI Hardening

### ☐ AP-P5-T1 — Add CI Caching and Tool Installation Strategy

- **Description:** Reduce CI runtime while preserving deterministic tool
  versions.
- **Priority:** P1
- **Dependencies:** AP-P0-T2
- **Parallelizable:** yes
- **Status:** Planned
- **Outputs / Artifacts:** updated GitHub Actions workflow.
- **Acceptance Criteria:**
  - Cargo registry and build caches are configured safely.
  - Tool versions for `cargo-audit`, `cargo-deny`, and `cargo-llvm-cov` are
    pinned or documented.
  - CI remains green after cache misses and cache hits.

### ☐ AP-P5-T2 — Add Cross-Platform CI Matrix

- **Description:** Validate the CLI on Linux and macOS, with Windows evaluated
  once path semantics are specified.
- **Priority:** P1
- **Dependencies:** AP-P1-T2
- **Parallelizable:** yes
- **Status:** Planned
- **Outputs / Artifacts:** CI matrix and platform-specific fixture notes.
- **Acceptance Criteria:**
  - Linux and macOS CI jobs pass.
  - Path-sensitive integrity tests are platform-aware.
  - Windows support is either implemented or explicitly documented as deferred.

### ☐ AP-P5-T3 — Add SBOM and Release Artifact Workflow

- **Description:** Prepare signed release artifacts and supply-chain metadata
  for the CLI.
- **Priority:** P2
- **Dependencies:** AP-P5-T1
- **Parallelizable:** yes
- **Status:** Planned
- **Outputs / Artifacts:** release workflow, checksums, SBOM.
- **Acceptance Criteria:**
  - Release builds produce reproducible checksums for supported targets.
  - SBOM generation is documented and attached to releases.
  - Release artifacts do not include private keys or machine-local paths.

### ☐ AP-P5-T4 — Add Minimum Supported Rust Version Policy

- **Description:** Define and test the minimum Rust version supported by the
  CLI and library.
- **Priority:** P2
- **Dependencies:** AP-P0-T1
- **Parallelizable:** yes
- **Status:** Planned
- **Outputs / Artifacts:** MSRV policy in README and CI.
- **Acceptance Criteria:**
  - MSRV is documented.
  - CI verifies the MSRV or documents why stable-only is currently required.
  - Dependency updates cannot silently raise MSRV without review.

## Phase 6. Developer Experience and Documentation

### ☐ AP-P6-T1 — Add Contributor Workflow Documentation

- **Description:** Document how to run the validator, quality gates, coverage,
  dependency policy, and fixture tests locally.
- **Priority:** P1
- **Dependencies:** AP-P0-T2
- **Parallelizable:** yes
- **Status:** Planned
- **Outputs / Artifacts:** README and/or `CONTRIBUTING.md` updates.
- **Acceptance Criteria:**
  - Contributor docs explain every CI gate.
  - Docs include expected exit codes for `agent-passport validate`.
  - Docs explain how to add valid/invalid fixtures.

### ☐ AP-P6-T2 — Improve Diagnostic Codes and Help Text

- **Description:** Add stable diagnostic codes and richer CLI help without
  making output noisy.
- **Priority:** P1
- **Dependencies:** AP-P1-T3
- **Parallelizable:** yes
- **Status:** Planned
- **Outputs / Artifacts:** diagnostic code registry, tests, CLI help snapshots.
- **Acceptance Criteria:**
  - Every hard error has a stable diagnostic code.
  - JSON output includes code, severity, path, and message.
  - Human output remains concise and readable.

### ☐ AP-P6-T3 — Add Shell Completion and Manpage Generation

- **Description:** Provide standard CLI distribution ergonomics.
- **Priority:** P2
- **Dependencies:** AP-P6-T2
- **Parallelizable:** yes
- **Status:** Planned
- **Outputs / Artifacts:** generated shell completions and manpage docs.
- **Acceptance Criteria:**
  - Completion generation supports bash, zsh, and fish.
  - Manpage/help text documents validation modes and integrity checks.
  - Generated artifacts are reproducible or generated only during release.

## Phase 7. Standardization Readiness

### ☐ AP-P7-T1 — Add RFC Test Vector Appendix

- **Description:** Add normative or informative examples that implementers can
  use to compare validators and verifiers.
- **Priority:** P1
- **Dependencies:** AP-P1-T2, AP-P3-T1
- **Parallelizable:** no
- **Status:** Planned
- **Outputs / Artifacts:** RFC appendix and fixtures.
- **Acceptance Criteria:**
  - Test vectors include valid, invalid, signed, expired, and integrity-mismatch
    passports.
  - Expected validation results are machine-readable.
  - Appendix avoids private keys and uses clearly marked test keys only.

### ☐ AP-P7-T2 — Define Compatibility and Versioning Rules

- **Description:** Specify how `apiVersion` and `metadata.version` changes are
  handled by validators, issuers, and runtimes.
- **Priority:** P1
- **Dependencies:** AP-P1-T3
- **Parallelizable:** yes
- **Status:** Planned
- **Outputs / Artifacts:** RFC section update and validator tests.
- **Acceptance Criteria:**
  - Validator behavior for unknown future `apiVersion` values is explicit.
  - Backward-compatible and breaking schema changes are classified.
  - CLI output tells relying parties when a passport was accepted leniently.

### ☐ AP-P7-T3 — Add External Implementation Conformance Checklist

- **Description:** Create a checklist for independent Agent Passport
  implementations.
- **Priority:** P2
- **Dependencies:** AP-P7-T1
- **Parallelizable:** yes
- **Status:** Planned
- **Outputs / Artifacts:** conformance checklist document.
- **Acceptance Criteria:**
  - Checklist covers parsing, schema validation, lifecycle, integrity,
    signature verification, trust store, and diagnostics.
  - Checklist references concrete fixtures and RFC sections.
  - Checklist distinguishes MUST, SHOULD, and optional implementation areas.
