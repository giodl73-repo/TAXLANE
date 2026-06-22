# Wave: Subfunction Decade Chart Spec

## Goal

Add a Vega-Lite chart specification for the subfunction decade rollup.

## Thesis

The decade rollup needs a compact visualization contract before it can be used
in reader or UI work. A top-five-by-decade chart shows long-run subfunction
change without pretending the modeled allocation is legal dedication.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|---|---|---|
| 01 | Decade top-subfunction chart spec | done | Added a Vega-Lite chart spec for top modeled subfunction allocations by decade and included it in validation and manifest coverage. |

## Success Criteria

- Add a chart spec backed by the decade-long subfunction CSV.
- Validate the spec through `income-tax-outlay validate`.
- Include the spec in manifest coverage and the chart catalog.
- Keep the model caveat visible through tooltip legal status.

## Non-Goals

- Do not add new data exports.
- Do not change allocation math.
- Do not approve taxpayer receipt wording.

## Validation

Run:

```powershell
cargo run -p taxlane-tools -- income-tax-outlay validate
cargo fmt --check
cargo test
git diff --check
```
