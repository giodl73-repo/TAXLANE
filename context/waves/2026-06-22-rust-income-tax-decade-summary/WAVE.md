# Wave: Rust Income-Tax Decade Summary

## Goal

Port the income-tax outlay decade summary generator from Python to Rust.

## Thesis

The decade summary is the next dependency after the annual model builder. It
can be generated from canonical annual JSONL with stable output checks, reducing
the remaining Python surface to the source workbook extraction step.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|---|---|---|
| 01 | Rust decade summary generator | done | Moved decade JSONL and Markdown generation into the Rust Taxlane CLI and removed the Python summary generator. |

## Success Criteria

- `cargo run -p taxlane-tools -- income-tax-outlay summary` writes decade JSONL
  and Markdown.
- `cargo run -p taxlane-tools -- income-tax-outlay summary --check` detects
  stale decade outputs.
- `cargo run -p taxlane-tools -- income-tax-outlay validate` uses the Rust
  summary check.
- Python `build_decade_summary.py` is removed.
- Existing decade outputs remain stable.

## Non-Goals

- Do not port XLSX extraction in this pulse.
- Do not change allocation math.
- Do not alter chart specs.

## Validation

Run:

```powershell
cargo run -p taxlane-tools -- income-tax-outlay summary --check
cargo run -p taxlane-tools -- income-tax-outlay validate
cargo fmt --check
cargo test
git diff --check
```
