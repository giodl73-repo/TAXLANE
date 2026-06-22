# Wave: Subfunction Dictionary Family

## Goal

Define the generated subfunction model record family in the data dictionary.

## Thesis

The subfunction model is now a tracked derived artifact with reader, chart, and
reconciliation coverage. The central data dictionary should define its fields
and use rules so future taxpayer receipt or lane work does not treat
subfunction rows as legal funding records.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|---|---|---|
| 01 | Subfunction family dictionary entry | done | Added `income_tax_outlay_subfunction_model` to the family map with required fields and public-use rules. |

## Success Criteria

- Add the subfunction model to the record family map.
- Define emitted model fields used by the Rust generator.
- Preserve drilldown-only and modeled-not-legal rules.

## Non-Goals

- Do not change generated model rows.
- Do not change validation code or chart specs.
- Do not approve taxpayer receipt wording.

## Validation

Run:

```powershell
cargo run -p taxlane-tools -- income-tax-outlay validate
cargo fmt --check
cargo test
git diff --check
```
