# Wave: Placeholder Visibility Receipt

## Goal

Create the first TAXLANE visibility receipt prototype using a placeholder
ordinary income-tax payment and FY2025 lane shares.

## Thesis

After the lane crosswalk and role-review guardrails, the next safe product is a
non-taxpayer-specific receipt scenario. It can show what a labeled lane receipt
would look like while preserving modeled-not-legal wording, offsetting rows,
debt service, and borrowed-share context.

## Pulse Table

| Pulse | Title | Status | Outcome |
|------:|---|---|---|
| 01 | Placeholder FY2025 receipt | done | Added draft placeholder receipt JSON, reader packet, and data-directory index entry. |

## Success Criteria

- Use a placeholder amount, not a calculated taxpayer liability.
- Use FY2025 actual-year OMB-derived lane shares.
- Include allocation method, legal status, and deficit context.
- Keep net interest and offsetting receipts visible.
- Block legal dedication and taxpayer-dollar tracing language.

## Non-Goals

- Do not build an interactive calculator.
- Do not use real taxpayer information.
- Do not make statutory tax-lane recommendations.
- Do not use dynamic Treasury, USAspending, or CBO sources.

## Validation

Run:

```powershell
git diff --check
cargo run -p taxlane-tools -- income-tax-outlay validate
```
