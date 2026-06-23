# Placeholder Receipt Display Packet

## Purpose

This packet is a static display handoff for the placeholder FY2025 visibility
receipt. It pairs the lane chart spec with the financing-context chart spec and
the explanatory copy that must travel with them.

It is not a public taxpayer receipt, not a calculator, and not a source of tax,
legal, or accounting advice.

## Required Display Pair

These two specs must be shown together:

| Spec | Role |
|---|---|
| `docs/charts/taxpayer-receipt-model/placeholder-lane-bars.vl.json` | Shows the placeholder `$1,000` modeled lane amounts. |
| `docs/charts/taxpayer-receipt-model/placeholder-financing-context.vl.json` | Shows FY2025 borrowed-share and income-tax-coverage context. |

The lane chart should not be displayed by itself. Without the financing context,
the receipt can make current outlays look fully covered by ordinary income-tax
receipts.

## Required Intro Copy

Use this copy near the display:

> This is a placeholder visibility receipt. It models how a `$1,000` ordinary
> individual income-tax payment would look if allocated by FY2025 federal
> outlay shares. It is not legal dedication, taxpayer-dollar tracing, tax
> advice, or a taxpayer liability calculation.

## Required Offset Copy

Use this copy near the negative rows:

> Negative rows are budget offsets or netting effects in the OMB outlay data.
> They are shown as adjustments, not as claims that a taxpayer paid a negative
> amount for a public service.

## Required Dedicated-Financing Copy

Use this copy near Social Security and Medicare:

> Social Security and Medicare have distinct financing structures involving
> payroll or social-insurance taxes, trust funds, premiums, general financing,
> or other rules. These rows are modeled ordinary income-tax visibility rows,
> not claims that ordinary income-tax dollars are legally dedicated to those
> programs.

## Required Financing Context Copy

Use this copy beside the financing-context chart:

> In FY2025, the model reports borrowed share of outlays at 25.31 percent and
> individual income-tax coverage of outlays at 37.88 percent. Borrowing and
> debt service are part of the fiscal picture and must not be hidden behind the
> lane allocation.

## Blocked Uses

- Do not add taxpayer input fields.
- Do not describe the result as a personalized receipt.
- Do not say a lane was legally funded by ordinary income tax.
- Do not crop out the financing-context chart.
- Do not present negative rows as negative service payments.

## Source Artifacts

| Artifact | Role |
|---|---|
| `data/derived/taxpayer_receipt_model/taxpayer_receipt_model.placeholder-1000.fy2025.omb-fy2027-v1.draft.json` | Canonical placeholder receipt scenario. |
| `docs/reading/placeholder-visibility-receipt.md` | Reader packet explaining the model. |
| `reviews/2026-06-22-placeholder-display-spec-role-review.md` | Role review for the display specs. |
| `reviews/2026-06-23-placeholder-display-packet-role-review.md` | Role review for this static display packet. |
| `docs/design/placeholder-receipt-placement-spec.md` | Static placement rules for keeping chart context and caveats together. |
