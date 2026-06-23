# Placeholder Receipt Placement Spec

## Purpose

This spec records placement rules for a static display of the placeholder
FY2025 visibility receipt. It closes the visual-placement follow-up from
`reviews/2026-06-23-placeholder-display-packet-role-review.md`.

It is not a UI implementation, not a public taxpayer receipt, not a calculator,
and not approval for public release.

## Required Artifact Group

The display must keep these artifacts together as one static group:

| Artifact | Placement role |
|---|---|
| `docs/charts/taxpayer-receipt-model/placeholder-lane-bars.vl.json` | Primary lane chart. |
| `docs/charts/taxpayer-receipt-model/placeholder-financing-context.vl.json` | Companion financing context. |
| `docs/reading/placeholder-receipt-display-packet.md` | Required intro, offset, dedicated-financing, and financing-context copy. |

The lane chart must not be placed in a standalone card, screenshot, export, or
embed without the financing context and required caveats.

## Desktop Placement

- Show the required intro copy above the chart group.
- Place the lane chart and financing-context chart in the same viewport when
  screen width allows.
- Keep the financing-context chart visually adjacent to the lane chart, not in a
  later section.
- Place offset copy near negative rows or directly below the lane chart.
- Place dedicated-financing copy close to Social Security and Medicare labels or
  directly below the lane chart if row-level annotations are not available.

## Mobile Placement

- Stack the required intro copy, lane chart, financing-context chart, and caveat
  copy in that order.
- Do not insert taxpayer input fields, filing-status controls, income fields, or
  refund/liability labels between the charts.
- Keep the financing-context chart before any shareable screenshot boundary or
  export boundary.
- If the lane chart is too tall for one screen, repeat or pin a short
  modeled-not-legal label near the chart title.

## Screenshot And Export Rules

- Any screenshot or export that includes the lane chart must also include the
  financing-context chart or a visible financing-context summary.
- Any screenshot or export that includes negative rows must include offset copy.
- Any screenshot or export that includes Social Security or Medicare rows must
  include dedicated-financing copy.
- The display title must include `placeholder` or `modeled`.
- Exported alt text must say the display is a modeled allocation, not legal
  dedication or taxpayer-dollar tracing.

## Blocked Patterns

- A lane-only social card.
- A calculator-style form with income, filing status, withholding, refund,
  credit, or liability fields.
- A chart image that crops out borrowed-share or income-tax-coverage context.
- Tooltip-only caveats.
- "Where your taxes went" headlines without modeled-allocation wording.

## Review Status

This spec is a static design handoff. A future visual mockup or UI still needs
separate role review before public release.

Role review for this spec is recorded at
`reviews/2026-06-23-placeholder-placement-spec-role-review.md`.
