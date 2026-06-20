# Agent Passport

This repository provides a comprehensive specification for the **Agent Passport** format, detailing its structure, components, and intended use cases. The **Agent Passport** aims to standardize the representation of agent-related data across various systems, ensuring consistency and interoperability. This RFC outlines the core elements of the format, guidelines for implementation, and examples to facilitate adoption and integration.

## Drafts

- [Agent Passport RFC](https://github.com/0al-spec/agent-passport/blob/main/drafts/agent-passport.md)

## CLI validator

This repository includes a Rust CLI for validating Agent Passport YAML documents
against the current experimental RFC shape.

Build and run:

```bash
cargo run -- validate examples/log-processor.passport.yaml
cargo run -- validate examples/specauthor-behavior-policy.passport.yaml
```

Install locally:

```bash
cargo install --path .
agent-passport validate examples/log-processor.passport.yaml
```

Useful options:

```bash
# Emit JSON for automation.
agent-passport validate --json examples/log-processor.passport.yaml

# Also verify agentIntegrity.codeHashes against files on disk.
agent-passport validate --check-integrity --integrity-root ./agent-root passport.yaml
```

The validator checks:

- YAML parseability and the required top-level `passport` object
- required RFC fields such as `apiVersion`, `kind`, `metadata`, `spec`,
  `capabilities`, and `signature`
- RFC 3339 lifecycle timestamps and expiration
- capability signatures, resource declarations, network entries, and security
  policy risk warnings
- signature field presence and base64 syntax
- optional SHA-256/SHA-512 file integrity checks via
  `agentIntegrity.codeHashes`

The v1alpha1 validator is intentionally forward-compatible: unknown fields are
ignored by the core validator. Experimental `x-*` fields such as
`x-behaviorPolicies` may be used for semantic or LLM behavior declarations, but
they are report-only extension data unless a consuming system explicitly
implements their meaning. They are not runtime security enforcement by default.

Full cryptographic signature verification is intentionally not implemented yet:
the RFC still needs a canonicalization profile and trust-store model so that
independent implementations verify exactly the same signed bytes.

## Quality gates

The GitHub Actions CI pipeline runs:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo llvm-cov --summary-only --fail-under-lines 68
cargo audit
cargo deny check
cargo run -- validate examples/log-processor.passport.yaml
```

## Releases

Releases are created from version tags. The tag must match the package version
in `Cargo.toml`:

```bash
# First bump Cargo.toml, then tag the same version.
git tag v<version>
git push origin v<version>
```

The release workflow validates the tag, runs locked tests, builds release
binaries for Linux and macOS, smoke-tests the binary, publishes SHA-256
checksums, and creates the GitHub Release.

## License

- 🧠 Specifications & Documents (in `drafts/`, `schema/`, `generated`, and `docs/`) are licensed under the Creative Commons Attribution 4.0 International License (CC BY 4.0).
- 💻 Source Code (in `src/`, `tools/`, etc.) is licensed under the MIT License.

See [LICENSE](./LICENSE) and [LICENSE-CC-BY-4.0](./LICENSE-CC-BY-4.0) for details.
