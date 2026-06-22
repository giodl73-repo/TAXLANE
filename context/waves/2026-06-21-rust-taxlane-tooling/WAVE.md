# Wave: Rust Taxlane Tooling

## Goal

Start moving TAXLANE tooling to Rust, beginning with the income-tax outlay
validation path.

## Thesis

Rust should become the preferred implementation language for TAXLANE tooling.
The lowest-risk first step is a Rust CLI that owns validation orchestration
while existing generators remain stable. Later pulses can port the generators
one at a time with output parity checks.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|---|---|---|
| 01 | Rust validation runner | done | Added a Rust CLI command that validates the income-tax outlay artifact chain. |

## Success Criteria

- Add a Cargo workspace and Taxlane tools crate.
- Provide `income-tax-outlay validate` as a Rust command.
- Validate existing model checks and chart specs.
- Update docs to prefer the Rust command.

## Non-Goals

- Do not rewrite XLSX extraction in this pulse.
- Do not change generated data values.
- Do not remove Python generators until Rust replacements have parity checks.

## Validation

Run:

```powershell
cargo run -p taxlane-tools -- income-tax-outlay validate
cargo fmt --check
cargo test
git diff --check
```
