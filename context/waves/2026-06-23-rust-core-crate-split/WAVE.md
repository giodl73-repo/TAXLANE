# Wave: Rust Core Crate Split

## Goal

Create the first Rust domain crate boundary so future accountability checks have typed validation outside CLI orchestration.

## Thesis

TAXLANE needs reusable domain contracts before it encodes fraud, waste, abuse, or performance review logic. The first split should be small: artifact metadata validation and accountability evidence boundary types in `taxlane-core`, with repository IO and report generation staying in `taxlane-tools`.

## Pulse Table

| Pulse | Title | Status | Outcome |
|------:|---|---|---|
| 01 | Core crate boundary | done | Added `taxlane-core`, wired CLI manifest validation through it, added tests for accountability evidence guardrails, and closed VTRACE WP-TAX-003. |

## Success Criteria

- Workspace includes a reusable core crate.
- CLI depends on the core crate for at least one existing validation path.
- Accountability evidence guardrails have typed Rust coverage.
- Cargo tests and TAXLANE validation pass.

## Non-Goals

- Do not migrate all CLI structs in this slice.
- Do not add public fraud detection or scoring.
- Do not parse accountability record files until source-custody examples exist.

## Validation

Run:

```powershell
cargo test
cargo fmt --check
cargo run -p taxlane-tools -- income-tax-outlay validate
cargo run --manifest-path ..\..\standards-protocols\vtrace\Cargo.toml -- validate .
git diff --check
```
