# Wave: Subfunction Derived View Dictionary

## Goal

Document the derived CSV view fields for the subfunction model.

## Thesis

The canonical subfunction model rows are defined in the data dictionary, but
chart-ready exports add view-specific fields such as decade coverage and rank.
Those fields should be described as derived views so they are not mistaken for
new canonical model families.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|---|---|---|
| 01 | Derived view notes | done | Added annual long, decade long, and ranked current-year view notes to the subfunction dictionary section. |

## Success Criteria

- Describe export-only fields for annual, decade, and ranked subfunction views.
- Keep canonical rows distinct from chart-ready views.
- Preserve modeled-not-legal and drilldown-only rules.

## Non-Goals

- Do not change generated data.
- Do not add new record families.
- Do not approve taxpayer receipt wording.

## Validation

Run:

```powershell
cargo run -p taxlane-tools -- income-tax-outlay validate
cargo fmt --check
cargo test
git diff --check
```
