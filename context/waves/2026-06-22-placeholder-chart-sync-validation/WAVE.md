# Wave: Placeholder Chart Sync Validation

## Goal

Make `income-tax-outlay validate` check that static placeholder receipt chart
values stay synchronized with the canonical placeholder receipt JSON.

## Thesis

The display-spec review allowed inline chart values only with a synchronization
guard. The chart specs duplicate receipt data, so validation should compare
lane amounts, lane shares, display treatments, borrowed share, and income-tax
coverage against the canonical receipt artifact.

## Pulse Table

| Pulse | Title | Status | Outcome |
|------:|---|---|---|
| 01 | Placeholder chart sync check | done | Added receipt/chart sync validation to the Rust validation command and refreshed the manifest. |

## Success Criteria

- `income-tax-outlay validate` fails if lane chart rows drift from the
  placeholder receipt JSON.
- `income-tax-outlay validate` fails if financing context values drift from the
  placeholder receipt JSON.
- Validation still parses all chart specs and checks the manifest.

## Non-Goals

- Do not generate chart specs in this wave.
- Do not change placeholder receipt amounts.
- Do not build a UI or calculator.

## Validation

Run:

```powershell
cargo run -p taxlane-tools -- income-tax-outlay validate
cargo test
git diff --check
```
