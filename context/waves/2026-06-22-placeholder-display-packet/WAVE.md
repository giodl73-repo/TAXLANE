# Wave: Placeholder Display Packet

## Goal

Add a static display packet that pairs the placeholder receipt chart specs with
required explanatory copy.

## Thesis

The display-spec review accepted the specs only as design-review handoff assets
and required explanatory copy before public-display work. A static reading
packet is the next safe artifact because it documents the full display contract
without creating an app, calculator, or taxpayer input flow.

## Pulse Table

| Pulse | Title | Status | Outcome |
|------:|---|---|---|
| 01 | Static display packet | done | Added a display packet with required intro, offset, dedicated-financing, and financing-context copy. |

## Success Criteria

- Packet pairs the lane chart with the financing-context chart.
- Packet includes required modeled-not-legal copy.
- Packet explains negative rows as offsets or netting.
- Packet explains Social Security and Medicare dedicated-financing caveats.
- Packet blocks calculator-shaped use.

## Non-Goals

- Do not build a UI or app.
- Do not alter chart specs or model values.
- Do not approve public release.

## Validation

Run:

```powershell
cargo run -p taxlane-tools -- income-tax-outlay validate
git diff --check
```
