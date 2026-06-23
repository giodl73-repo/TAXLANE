# Taxpayer Receipt Model Charts

Parent catalog: `docs/charts/README.md`

## Purpose

These chart specs visualize the placeholder FY2025 visibility receipt.

They are static handoff specs for design review. They do not create a
calculator, accept taxpayer inputs, change the model, or claim legal dedication
of ordinary income-tax receipts.

## Specs

| Spec | Data | Use |
|---|---|---|
| `placeholder-lane-bars.vl.json` | Inline rows from the placeholder receipt JSON | Show lane amounts while separating modeled lanes, offsets, financing costs, and dedicated-financing caveats. |
| `placeholder-financing-context.vl.json` | Inline FY2025 context values | Show borrowed share and income-tax coverage beside the receipt. |

## Display Rules

- Title and labels must say `placeholder` or `modeled`.
- Offsets must be shown as adjustments, not service payments.
- Social Security and Medicare need dedicated-financing caveats.
- Borrowed share must appear beside the lane chart.
- Do not place these specs in a taxpayer-calculator flow.

The chart-set role review is recorded at
`reviews/2026-06-22-placeholder-display-spec-role-review.md`.

The static placement rules for pairing these charts with required copy are
recorded at `docs/design/placeholder-receipt-placement-spec.md`.
