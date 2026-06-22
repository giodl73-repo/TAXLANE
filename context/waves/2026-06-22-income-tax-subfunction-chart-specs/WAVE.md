# Wave: Income-Tax Subfunction Chart Specs

## Goal

Add first Vega-Lite chart specifications for the subfunction allocation exports.

## Thesis

The subfunction CSV exports are useful only if downstream readers and UI work
have stable visualization contracts. The first chart specs should stay compact:
a FY2025 ranked view for current-year drilldown and a selected-subfunction trend
view for long-run comparison.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|---|---|---|
| 01 | Subfunction chart specs | done | Added FY2025 top-subfunction and selected-subfunction trend Vega-Lite specs and included them in chart validation and manifest coverage. |

## Success Criteria

- Add a FY2025 top-subfunction chart spec backed by the ranked CSV export.
- Add a selected-subfunction trend chart spec backed by the annual-long CSV
  export.
- `income-tax-outlay validate` parses both new chart specs.
- Manifest coverage includes both new chart specs.
- Wording remains modeled and non-legal.

## Non-Goals

- Do not add a public reader packet.
- Do not change CSV exports or model math.
- Do not claim program tracing or legal dedication.

## Validation

Run:

```powershell
cargo run -p taxlane-tools -- income-tax-outlay validate
cargo fmt --check
cargo test
git diff --check
```
