# Placeholder Receipt Mockup Review Checklist

## Purpose

Use this checklist before accepting any static mockup of the placeholder FY2025
visibility receipt. It turns the placement-spec review follow-up into a concrete
review gate without creating a UI, app, or calculator.

This checklist does not approve public release. It is a pre-implementation
review aid.

## Required Inputs

| Input | Required artifact |
|---|---|
| Canonical scenario | `data/derived/taxpayer_receipt_model/taxpayer_receipt_model.placeholder-1000.fy2025.omb-fy2027-v1.draft.json` |
| Lane chart spec | `docs/charts/taxpayer-receipt-model/placeholder-lane-bars.vl.json` |
| Financing chart spec | `docs/charts/taxpayer-receipt-model/placeholder-financing-context.vl.json` |
| Required copy | `docs/reading/placeholder-receipt-display-packet.md` |
| Placement rules | `docs/design/placeholder-receipt-placement-spec.md` |

## Acceptance Checks

| Check | Pass condition |
|---|---|
| Static scenario | Mockup uses the placeholder `$1,000` scenario and does not ask for taxpayer data. |
| Chart pair | Lane chart and financing-context chart appear together in the same display flow. |
| Modeled label | Title or lead copy says `placeholder` or `modeled`. |
| Legal boundary | Visible copy says the display is not legal dedication or taxpayer-dollar tracing. |
| Offset caveat | Negative rows appear with offset or netting copy. |
| Dedicated-financing caveat | Social Security and Medicare appear with dedicated-financing copy. |
| Financing context | Borrowed share and income-tax coverage are visible before any share/export boundary. |
| Mobile order | Mobile mockup stacks intro, lane chart, financing context, and caveats in that order. |
| Screenshot completeness | Example screenshots that include the lane chart also include financing context or a visible financing summary. |
| Alt text | Exported or shareable alt text says modeled allocation and not legal funding. |

## Blocking Findings

Any of these findings blocks acceptance:

- Taxpayer input fields, filing-status controls, income fields, withholding
  fields, refund labels, credit labels, or liability labels.
- A lane-only chart export or social card.
- Tooltip-only caveats.
- "Where your taxes went" wording without modeled-allocation context.
- Social Security or Medicare rows without dedicated-financing caveats.
- Negative rows shown without offset or netting copy.
- Financing context placed after a screenshot, embed, or export boundary.

## Review Output

Record the review under `reviews/` before implementation. The review should
state:

- whether the mockup remains static and scenario-based,
- whether desktop and mobile placements pass,
- whether screenshots and exports remain context-complete,
- which artifacts were checked,
- and whether public release remains blocked.
