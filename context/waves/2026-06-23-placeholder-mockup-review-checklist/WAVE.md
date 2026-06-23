# Wave: Placeholder Mockup Review Checklist

## Goal

Add a review checklist for any future static placeholder receipt mockup.

## Thesis

The placement-spec review requires a static mockup review before implementation.
A checklist is the next safe artifact because it defines acceptance and blocking
findings without creating a UI, calculator, or public-release asset.

## Pulse Table

| Pulse | Title | Status | Outcome |
|------:|---|---|---|
| 01 | Mockup review checklist | done | Added a static mockup acceptance checklist and included it in manifest coverage. |

## Success Criteria

- Checklist maps required source, chart, copy, and placement artifacts.
- Checklist blocks taxpayer-input, lane-only export, tooltip-only caveat, and
  misleading headline patterns.
- Checklist is linked from design/review docs and included in manifest coverage.

## Non-Goals

- Do not build a mockup or UI.
- Do not change chart specs, receipt values, or model outputs.
- Do not approve public release.

## Validation

Run:

```powershell
cargo run -p taxlane-tools -- income-tax-outlay manifest
cargo run -p taxlane-tools -- income-tax-outlay validate
git diff --check
```
