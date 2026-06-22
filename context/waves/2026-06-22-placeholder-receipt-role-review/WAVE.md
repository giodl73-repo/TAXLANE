# Wave: Placeholder Receipt Role Review

## Goal

Apply TAXLANE's role panel to the placeholder visibility receipt before any
public-display or calculator-shaped work builds on it.

## Thesis

The placeholder receipt is the first artifact that looks like a taxpayer-facing
receipt. It needs a dedicated review so negative offsets, dedicated-financing
caveats, and borrowed-share context are treated as release blockers before UI or
calculator work begins.

## Pulse Table

| Pulse | Title | Status | Outcome |
|------:|---|---|---|
| 01 | Placeholder receipt review | done | Added role review findings and linked them from the receipt model README. |

## Success Criteria

- Review covers all TAXLANE role lenses.
- Review approves only design-review use.
- Review blocks public display without offset wording, dedicated-financing
  caveats, and borrowed-share context.
- Review blocks taxpayer-calculator interpretation.

## Non-Goals

- Do not change receipt amounts in this wave.
- Do not build a public UI or calculator.
- Do not approve taxpayer-facing publication.

## Validation

Run:

```powershell
git diff --check
cargo run -p taxlane-tools -- income-tax-outlay validate
```
