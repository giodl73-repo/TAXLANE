# Wave: Lane Crosswalk Role Review

## Goal

Apply TAXLANE's role panel to the draft public-purpose lane crosswalk before any
taxpayer receipt prototype uses it.

## Thesis

The lane crosswalk is the bridge between OMB source categories and TAXLANE
reader-facing labels. It needs explicit role review so later receipt work cannot
mistake a mapping table for legal dedication or taxpayer-dollar tracing.

## Pulse Table

| Pulse | Title | Status | Outcome |
|------:|---|---|---|
| 01 | Crosswalk role review | done | Added role review findings and linked them from the lane crosswalk README. |

## Success Criteria

- Review covers taxpayer, accounting, source, public-goods, beneficiary,
  compliance, fiscal, and reform-skeptic lenses.
- Review approves the crosswalk only as a draft mapping input.
- Review blocks public receipt use without method labels, legal status, and
  deficit context.

## Non-Goals

- Do not build a taxpayer receipt model in this wave.
- Do not change lane mappings unless the review finds a blocking issue.
- Do not approve statutory program-linked tax lanes.

## Validation

Run:

```powershell
git diff --check
```
