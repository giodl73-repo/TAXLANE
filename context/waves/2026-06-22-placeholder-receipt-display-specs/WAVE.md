# Wave: Placeholder Receipt Display Specs

## Goal

Add static chart specs for the placeholder visibility receipt without creating a
taxpayer calculator.

## Thesis

The receipt now carries display guardrails for offsets, dedicated financing, and
borrowed-share context. The next safe UI-facing artifact is a static chart
handoff that preserves those separations before any interactive display work.

## Pulse Table

| Pulse | Title | Status | Outcome |
|------:|---|---|---|
| 01 | Static receipt display specs | done | Added placeholder lane-bar and financing-context chart specs plus catalog entries and validation coverage. |

## Success Criteria

- Specs are static and do not accept taxpayer input.
- Lane chart separates modeled lanes, dedicated-financing caveats, financing
  cost, and offsets.
- Financing context appears as a companion spec.
- Chart catalog points readers back to the placeholder receipt packet.

## Non-Goals

- Do not build an app, public UI, or calculator.
- Do not alter modeled receipt values.
- Do not publish a final taxpayer-facing receipt.

## Validation

Run:

```powershell
cargo run -p taxlane-tools -- income-tax-outlay validate
git diff --check
```
