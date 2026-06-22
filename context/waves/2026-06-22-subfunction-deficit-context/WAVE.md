# Wave: Subfunction Deficit Context

## Goal

Add the deficit-context guardrail for public subfunction allocation views.

## Thesis

Subfunction labels are more concrete than broad outlay categories and therefore
easier to misread as program financing claims. The reader packet and role review
should require borrowing and income-tax coverage context before public
subfunction charts are treated as taxpayer-facing explanations.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|---|---|---|
| 01 | Deficit context guardrail | done | Added a research note, reader financing-context section, and role-review addendum for subfunction chart use. |

## Success Criteria

- Record the deficit-context rule in `docs/research/`.
- Update the subfunction reader packet with financing-context guidance.
- Update the role review so standalone public subfunction charts remain blocked
  without broad financing context.
- Keep model math and chart artifacts unchanged.

## Non-Goals

- Do not create new generated data.
- Do not change chart specs.
- Do not approve taxpayer receipts.

## Validation

Run:

```powershell
cargo run -p taxlane-tools -- income-tax-outlay validate
cargo fmt --check
cargo test
git diff --check
```
