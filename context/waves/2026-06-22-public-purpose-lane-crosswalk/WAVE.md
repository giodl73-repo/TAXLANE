# Wave: Public-Purpose Lane Crosswalk

## Goal

Create the first explicit crosswalk from OMB outlay functions and subfunctions
to TAXLANE public-purpose lane IDs.

## Thesis

The modeled income-tax outlay views already show broad and subfunction spending
visibility. The next safe step is a lane crosswalk that links OMB categories to
TAXLANE reader labels while preserving the modeled-not-legal and deficit-context
guardrails.

## Pulse Table

| Pulse | Title | Status | Outcome |
|------:|---|---|---|
| 01 | Draft OMB lane crosswalk | done | Added draft lane_crosswalk JSONL, method README, research note, and schema guardrails. |

## Success Criteria

- Draft `lane_crosswalk` records cover the current OMB Table 3.2 actual-year
  function set.
- Each lane lists included functions and subfunctions.
- Net interest and undistributed offsetting receipts remain separate.
- Borrowed share / deficit gap remains required context, not an outlay lane.
- Public wording does not imply legal dedication of ordinary income-tax
  receipts.

## Non-Goals

- Do not build a taxpayer receipt model in this wave.
- Do not set tax rates or statutory lane recommendations.
- Do not use dynamic Treasury, USAspending, or CBO sources.

## Validation

Run:

```powershell
git diff --check
```
