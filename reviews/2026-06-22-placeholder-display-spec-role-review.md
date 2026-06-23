# Placeholder Receipt Display Spec Role Review

## Scope

This review covers the static chart handoff for the placeholder visibility
receipt:

| Artifact | Role |
|---|---|
| `docs/charts/taxpayer-receipt-model/placeholder-lane-bars.vl.json` | Static lane display spec. |
| `docs/charts/taxpayer-receipt-model/placeholder-financing-context.vl.json` | Companion financing context spec. |
| `docs/charts/taxpayer-receipt-model/README.md` | Display rules and chart-set boundary. |
| `data/derived/taxpayer_receipt_model/taxpayer_receipt_model.placeholder-1000.fy2025.omb-fy2027-v1.draft.json` | Canonical receipt data source. |

## Decision

The static display specs are acceptable as design-review handoff assets.

This review does not approve a public taxpayer receipt, interactive calculator,
taxpayer input flow, legal dedication claim, or final visual design.

## Findings

| Role | Result |
|---|---|
| Taxpayer Advocate | Pass with caution: the chart titles and axis labels say placeholder/modeled, but a final public display needs plain text near the chart, not only tooltips. |
| Budget Accountant | Pass: offsets, financing cost, and dedicated-financing caveat lanes are visually separated from ordinary modeled lanes. |
| Source Custodian | Pass with P2 follow-up: the specs use inline values, so they must stay synchronized with the canonical receipt JSON. |
| Public Goods Steward | Pass: public-purpose labels remain understandable and do not promote statutory lane conversion. |
| Program Beneficiary | Pass with caution: Social Security and Medicare are visually flagged as caveat-sensitive, but future public copy must spell out payroll/social-insurance financing. |
| Compliance Burden | Pass: the specs are static and do not ask for taxpayer data. |
| Fiscal Sustainability | Pass: borrowed-share and income-tax coverage appear in a companion context spec. |
| Reform Skeptic | Pass: the display treatment labels reduce false precision and do not claim legal tracing. |

## Required Guardrails

### P1: Do not ship the lane chart without the financing companion

- **Roles**: Fiscal Sustainability Reviewer, Budget Accountant.
- **Evidence**: The lane chart shows a full `$1,000` placeholder allocation,
  while the context chart says individual income tax covered 37.88 percent of
  FY2025 outlays and borrowed share was 25.31 percent.
- **Risk**: The lane chart alone can make the public system look fully funded by
  current income-tax receipts.
- **Decision**: Public display work must present the financing context beside
  the lane chart or block the lane chart from standalone use.

### P1: Keep the specs out of calculator-shaped surfaces

- **Roles**: Taxpayer Advocate, Compliance Burden Reviewer.
- **Evidence**: The specs use a placeholder amount and inline values; they do
  not compute from taxpayer data.
- **Risk**: A future UI could invite users to enter tax information before the
  repo has tax-advice and calculator-boundary review.
- **Decision**: These specs remain static design-review handoff assets.

### P2: Synchronize inline chart values with the canonical receipt

- **Roles**: Source Custodian, Budget Accountant.
- **Evidence**: The chart specs duplicate values from the canonical JSON rather
  than loading the JSON directly.
- **Risk**: A later receipt update could leave chart values stale.
- **Decision**: Before public release, either generate the chart values from the
  canonical receipt artifact or add a validation check that compares inline
  values against it.

### P2: Add in-chart explanatory copy before public release

- **Roles**: Taxpayer Advocate, Reform Skeptic.
- **Evidence**: Caveats currently appear in chart treatment labels and the chart
  README, not as explicit visible text in the chart specs.
- **Risk**: Screenshots or embedded charts could lose the caveat context.
- **Decision**: A public display should include visible subtitle or annotation
  text for modeled-not-legal, offset, and dedicated-financing caveats.

## Follow-Up

- Add a static display packet that pairs both specs with required explanatory
  copy before any app or web mockup.
- Add sync validation if the inline chart-value approach continues.
