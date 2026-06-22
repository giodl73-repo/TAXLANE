# Wave: Rust Income-Tax Annual Model

## Goal

Port the income-tax outlay annual XLSX model builder from Python to Rust.

## Thesis

The annual model builder is the last Python dependency in the income-tax outlay
artifact pipeline. Moving XLSX parsing and source-profile generation into the
Rust Taxlane CLI makes the pipeline consistently Rust-owned while preserving
canonical outputs.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|---|---|---|
| 01 | Rust annual model builder | done | Moved annual JSONL and source-profile generation into the Rust Taxlane CLI and removed the Python model builder. |

## Success Criteria

- `cargo run -p taxlane-tools -- income-tax-outlay model` writes annual JSONL
  and source profile outputs.
- `cargo run -p taxlane-tools -- income-tax-outlay model --check` detects stale
  annual outputs.
- `cargo run -p taxlane-tools -- income-tax-outlay validate` uses the Rust
  model check.
- Python `build_income_tax_outlay_model.py` is removed.
- Existing annual outputs remain stable.

## Non-Goals

- Do not change allocation math.
- Do not alter source files.
- Do not alter chart specs.

## Validation

Run:

```powershell
cargo run -p taxlane-tools -- income-tax-outlay model --check
cargo run -p taxlane-tools -- income-tax-outlay validate
cargo fmt --check
cargo test
git diff --check
```
