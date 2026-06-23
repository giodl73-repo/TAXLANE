# Rust Core Crate Architecture Review

## Scope

This review covers the first TAXLANE Rust crate split:

| Artifact | Role |
|---|---|
| `Cargo.toml` | Workspace membership for tools and core crate. |
| `crates/taxlane-core/Cargo.toml` | Reusable domain crate manifest. |
| `crates/taxlane-core/src/lib.rs` | Domain contracts for artifact metadata and accountability evidence records. |
| `tools/taxlane/Cargo.toml` | CLI dependency on `taxlane-core`. |
| `tools/taxlane/src/main.rs` | CLI use of core artifact metadata validation during manifest generation/checking. |

## Decision

Accept the split as the first architecture boundary for WP-TAX-003.

The core crate is intentionally small. It owns reusable validation contracts and
typed accountability evidence boundaries; the CLI keeps repository file IO,
source extraction, generated outputs, and command routing.

## Findings

| Role | Result |
|---|---|
| Maintainer | Pass: workspace builds and `cargo test` exercises the new library. |
| Reform Skeptic | Pass: accountability evidence validation blocks public fraud wording without official status. |
| Source Custodian | Pass: source IDs remain required for accountability evidence records. |
| Budget Accountant | Pass: existing model generation stays in the CLI and is not changed by the split. |

## Guardrails

- Keep `taxlane-core` free of repo-relative filesystem assumptions.
- Add record parsers only after example accountability records exist.
- Do not add scoring or fraud inference to the core crate before source-custody examples and role review.

## Validation

- `cargo test`
- `cargo fmt --check`
- `cargo run -p taxlane-tools -- income-tax-outlay validate`
