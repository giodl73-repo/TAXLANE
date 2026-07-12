# Spending-Side Scope Decision

## Decision

Do the next spending-side slice inside TAXLANE, not a new repo.

TAXLANE already owns the federal tax-legibility frame: receipts, outlays,
borrowing, lane labels, taxpayer visibility, and accountability guardrails. The
next question, "where is the money actually going?", is the natural outlay-side
counterpart to the rate and receipt work already completed.

## What TAXLANE should answer next

1. What are the largest FY2025 federal outlay categories?
2. Which of those are mandatory benefits, defense, health care, interest,
   income security, veterans, infrastructure, or administration?
3. How much of each category is covered by individual income tax under the
   existing proportional model?
4. How much current spending is financed by total receipts versus borrowing?
5. Which large categories deserve follow-up performance questions, without
   implying fraud or waste from size alone?

The current derived files already support the first slice:

| FY2025 view | Current evidence |
|---|---|
| Total outlays | $7.011T |
| Total receipts | $5.236T |
| Deficit gap | $1.775T |
| Borrowed share of outlays | 25.31% |
| Individual income-tax coverage of outlays | 37.88% |
| Largest subfunctions | Social Security, gross Treasury interest, Medicare, health care services, DOD-Military |

## Why not split now

A separate `SPENDCAT` repo would be premature if it only repeats TAXLANE's
federal outlay tables. Splitting now would create a second place for source
custody, caveats, lane labels, and accountability guardrails.

`SPENDCAT` becomes justified if the product changes from a federal tax/spend
explainer into a reusable classification engine:

- Map any ledger or public budget into spend categories.
- Reconcile agency/account/program/award taxonomies across data systems.
- Compare spend categories across jurisdictions or organizations.
- Offer a domain-neutral CLI/schema other repos can reuse.

## Proposed first reader packet

Create `docs/reading/where-federal-money-goes.md` with:

- A top-line FY2025 funding context.
- The top 15 subfunctions by outlays.
- A grouped explanation of mandatory benefits, health, defense, interest,
  income security, veterans, transportation, education, disasters, and justice.
- A "what this can and cannot prove" box.
- Links to the accountability public brief for performance-question handoff.

## Guardrails

- Use "outlays" when the source is OMB outlays.
- Use "modeled allocation" when showing income-tax shares.
- Use "borrowing financed the gap" only at aggregate context level unless a
  source supports a more specific claim.
- Do not describe obligations, awards, or recipients as final outlays without an
  outlay source.
- Do not call a category wasteful merely because it is large.
