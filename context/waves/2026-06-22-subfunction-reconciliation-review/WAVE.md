# Wave: Subfunction Reconciliation Review

## Goal

Add a reconciliation review for the generated subfunction model outputs.

## Thesis

The broad modeled-outlay output has a dedicated reconciliation review. The
subfunction output should have the same review artifact so readers can see what
was checked before using annual, decade, or ranked subfunction exports.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|---|---|---|
| 01 | Subfunction reconciliation review | done | Added a review note for subfunction model/export checks and included it in manifest coverage. |

## Success Criteria

- Add a subfunction reconciliation review under the derived subfunction model.
- Include annual, decade, and FY2025 export checks.
- Include the review in manifest coverage.
- Keep modeled-not-legal caveats visible.

## Non-Goals

- Do not change generated model rows.
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
