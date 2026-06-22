# Wave: Rust Income-Tax Chart Export

## Goal

Port the income-tax outlay chart-ready CSV exporter from Python to Rust.

## Thesis

After the Rust manifest port, the next lowest-risk generator is the chart CSV
exporter. It transforms canonical JSONL into wide CSV views, so Rust can take
ownership with direct parity checks against the existing generated files.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|---|---|---|
| 01 | Rust chart CSV exporter | done | Moved chart CSV export/check behavior into the Rust Taxlane CLI and removed the Python exporter. |

## Success Criteria

- `cargo run -p taxlane-tools -- income-tax-outlay export` writes both wide CSVs.
- `cargo run -p taxlane-tools -- income-tax-outlay export --check` detects stale CSVs.
- `cargo run -p taxlane-tools -- income-tax-outlay validate` uses the Rust exporter check.
- Python `export_chart_views.py` is removed.
- Existing CSV content remains stable.

## Non-Goals

- Do not port annual or decade JSONL generators in this pulse.
- Do not change allocation math.
- Do not alter chart specs.

## Validation

Run:

```powershell
cargo run -p taxlane-tools -- income-tax-outlay export --check
cargo run -p taxlane-tools -- income-tax-outlay validate
cargo fmt --check
cargo test
git diff --check
```
