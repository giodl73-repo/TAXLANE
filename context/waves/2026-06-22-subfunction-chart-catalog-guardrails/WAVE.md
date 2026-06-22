# Wave: Subfunction Chart Catalog Guardrails

## Goal

Make the chart catalog's public handoff rules match the subfunction reader
guardrails.

## Thesis

The catalog is the likely entry point for future UI and analysis work. It should
point readers to the correct public wording packet and preserve the financing,
subfunction-label, and partial-decade caveats before charts are embedded
elsewhere.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|---|---|---|
| 01 | Catalog guardrails | done | Updated chart reading order and wording rules for subfunction drilldowns, financing context, and partial decades. |

## Success Criteria

- Link the broad and subfunction reader packets from the chart reading order.
- State the Table 3.2 subfunction wording rule.
- Require financing context for public subfunction chart use.
- Preserve partial-decade caveats for the decade subfunction chart.
- Refresh manifest coverage for the chart catalog.

## Non-Goals

- Do not change chart specs or generated data.
- Do not add UI rendering code.
- Do not approve taxpayer receipt wording.

## Validation

Run:

```powershell
cargo run -p taxlane-tools -- income-tax-outlay validate
cargo fmt --check
cargo test
git diff --check
```
