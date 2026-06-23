# Wave: Placeholder Placement Spec

## Goal

Add static placement rules for the placeholder receipt display.

## Thesis

The display packet role review accepted the copy contract but left viewport,
screenshot, and placement behavior as a follow-up. A placement spec is the next
safe artifact because it keeps the work static while making future mockups
harder to separate charts from required caveats.

## Pulse Table

| Pulse | Title | Status | Outcome |
|------:|---|---|---|
| 01 | Static placement spec | done | Added placement, mobile, screenshot, and export rules for the placeholder receipt display. |

## Success Criteria

- Spec keeps the lane chart, financing context, and required caveats together.
- Spec blocks calculator-shaped controls and lane-only screenshots.
- Spec is manifest-tracked and linked from chart/display handoff docs.

## Non-Goals

- Do not build a UI or mockup.
- Do not alter chart specs, receipt values, or model outputs.
- Do not approve public release.

## Validation

Run:

```powershell
cargo run -p taxlane-tools -- income-tax-outlay manifest
cargo run -p taxlane-tools -- income-tax-outlay validate
git diff --check
```
