# Wave: Placeholder Display Spec Review

## Goal

Apply TAXLANE's role panel to the static placeholder receipt display specs.

## Thesis

The display specs are the first visual receipt handoff. They need review before
any public display packet, app mockup, or calculator-shaped surface builds on
them.

## Pulse Table

| Pulse | Title | Status | Outcome |
|------:|---|---|---|
| 01 | Static display spec review | done | Added role review findings and linked them from the taxpayer receipt chart README. |

## Success Criteria

- Review approves design-review use only.
- Review blocks standalone lane-chart use without financing context.
- Review blocks calculator-shaped surfaces.
- Review records inline value synchronization as a follow-up.

## Non-Goals

- Do not change the display specs in this wave.
- Do not build a public display packet or app.
- Do not add taxpayer inputs.

## Validation

Run:

```powershell
git diff --check
cargo run -p taxlane-tools -- income-tax-outlay validate
```
