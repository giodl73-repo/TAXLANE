# Wave: Income-Tax Subfunction Chart Exports

## Goal

Create chart-ready CSV exports for the income-tax subfunction allocation model.

## Thesis

The subfunction JSONL is the canonical model output, but reader and UI work need
bounded CSV views. The first export should keep one complete annual long view
and one compact FY2025 ranked view while preserving the modeled-not-legal
allocation caveat.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|---|---|---|
| 01 | Subfunction CSV exports | done | Added Rust generation for annual-long and FY2025 top-subfunction CSV views and included them in aggregate validation and manifest coverage. |

## Success Criteria

- `cargo run -p taxlane-tools -- income-tax-outlay subfunction-export` writes
  annual long and FY2025 top-subfunction CSV outputs.
- `cargo run -p taxlane-tools -- income-tax-outlay subfunction-export --check`
  detects stale outputs.
- `cargo run -p taxlane-tools -- income-tax-outlay validate` includes the
  subfunction export check.
- Manifest coverage includes subfunction JSONL, CSV exports, README, and source
  profile.
- Public wording stays explicitly modeled and non-legal.

## Non-Goals

- Do not add public reader copy for subfunction allocation.
- Do not add chart specifications in this wave.
- Do not change allocation math or source extraction.

## Validation

Run:

```powershell
cargo run -p taxlane-tools -- income-tax-outlay subfunction-model --check
cargo run -p taxlane-tools -- income-tax-outlay subfunction-export --check
cargo run -p taxlane-tools -- income-tax-outlay validate
cargo fmt --check
cargo test
git diff --check
```
