# Wave: Subfunction Decade Rollup

## Goal

Add a generated decade rollup for the income-tax subfunction allocation model.

## Thesis

Annual subfunction rows are too detailed for long-run reading. A decade rollup
keeps the same modeled-not-legal caveat while giving readers and future chart
work a compact way to compare detailed budget subfunctions over time.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|---|---|---|
| 01 | Decade subfunction CSV export | done | Extended the Rust subfunction export command with a decade-long CSV rollup and manifest coverage. |

## Success Criteria

- `income-tax-outlay subfunction-export` writes a decade-long CSV.
- `income-tax-outlay subfunction-export --check` detects stale decade output.
- Decade percentages sum back to 100 percent per decade.
- Manifest coverage includes the decade-long export.
- Model math remains proportional allocation, not legal dedication.

## Non-Goals

- Do not add new chart specs.
- Do not change annual model rows.
- Do not add public receipt claims.

## Validation

Run:

```powershell
cargo run -p taxlane-tools -- income-tax-outlay subfunction-export --check
cargo run -p taxlane-tools -- income-tax-outlay validate
cargo fmt --check
cargo test
git diff --check
```
