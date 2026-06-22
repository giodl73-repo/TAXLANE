# Subfunction Deficit Context Note

## Question

What financing caveat has to travel with modeled subfunction allocation charts?

## Finding

Subfunction allocation charts should not be shown alone in public-facing
contexts. They make detailed labels visible, which is useful, but the labels can
sound like program financing claims. A responsible subfunction view needs nearby
financing context:

- individual income-tax receipts are general receipts in this model,
- the allocation is proportional to OMB Table 3.2 outlay shares,
- the allocation is not legal dedication or taxpayer-dollar tracing,
- total federal outlays may exceed income-tax receipts,
- borrowing and other receipt sources help finance the same outlay year.

The broad outlay model already carries borrowed-share and income-tax-coverage
context. Subfunction drilldowns should point back to that context before readers
interpret detailed labels.

## Display Rule

Any public subfunction chart should be paired with one of these:

1. a nearby broad-model financing context chart,
2. a visible sentence giving borrowed share and income-tax coverage for the same
   fiscal year or period, or
3. an explicit link back to `docs/reading/modeled-income-tax-outlays.md`.

If none of those is present, the chart is an analysis artifact only, not a
public taxpayer-facing receipt.

## FY2025 Example

The FY2025 top-subfunction chart can say:

> In this model, FY2025 individual income-tax receipts are allocated by OMB
> Table 3.2 subfunction outlay shares. This is not legal dedication. Read it
> beside the broad financing context because income taxes are only one federal
> receipt source and borrowing also finances outlays.

It should not say:

> Your income taxes paid these programs.

That wording implies tracing and omits deficit context.

## Consequence

The subfunction reader packet may remain public as a drilldown companion, but
future UI work should not make subfunction charts the first or only fiscal
explanation. The broad model and financing context stay first.
