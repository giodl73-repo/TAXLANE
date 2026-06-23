# Wave: Receipt Display Guardrails

## Goal

Make the placeholder visibility receipt carry explicit display guardrails for
negative offsets, dedicated-financing caveats, borrowed-share context, and the
taxpayer-calculator boundary.

## Thesis

The receipt role review approved the prototype only as a design-review artifact
and identified blockers before public display. The next safe pulse is to encode
those blockers in the receipt artifact and reader packet before any mockup or
calculator-shaped work begins.

## Pulse Table

| Pulse | Title | Status | Outcome |
|------:|---|---|---|
| 01 | Receipt display guardrails | done | Added offset wording, Social Security/Medicare caveats, and display guardrail fields to the receipt prototype. |

## Success Criteria

- Negative rows are described as offsets or netting, not negative service
  payments.
- Social Security and Medicare rows distinguish modeled ordinary income-tax
  allocation from dedicated financing.
- Borrowed-share context remains beside the receipt.
- The prototype remains static and not calculator-shaped.

## Non-Goals

- Do not change modeled amounts.
- Do not build a UI mockup.
- Do not add taxpayer input fields.

## Validation

Run:

```powershell
git diff --check
cargo run -p taxlane-tools -- income-tax-outlay validate
```
