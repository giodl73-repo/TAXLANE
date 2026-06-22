# Wave: Rust Income-Tax Manifest

## Goal

Port the income-tax outlay artifact manifest generator from Python to Rust.

## Thesis

TAXLANE tooling should move to Rust. The manifest generator is self-contained,
so it is the safest first generator to port while preserving output checks and
the existing generated data.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|---|---|---|
| 01 | Rust manifest generator | done | Moved manifest build/check behavior into the Rust Taxlane CLI and removed the Python manifest runner. |

## Success Criteria

- `cargo run -p taxlane-tools -- income-tax-outlay manifest` writes the manifest.
- `cargo run -p taxlane-tools -- income-tax-outlay validate` checks manifest freshness.
- Python `build_manifest.py` and `validate_all.py` are removed.
- Generated data values are unchanged.

## Non-Goals

- Do not port XLSX extraction in this pulse.
- Do not change allocation math.
- Do not alter chart specs.

## Validation

Run:

```powershell
cargo run -p taxlane-tools -- income-tax-outlay manifest
cargo run -p taxlane-tools -- income-tax-outlay validate
cargo fmt --check
cargo test
git diff --check
```
