# Placeholder Visibility Receipt Role Review

## Scope

This review covers the first draft visibility receipt prototype:

| Artifact | Role |
|---|---|
| `data/derived/taxpayer_receipt_model/taxpayer_receipt_model.placeholder-1000.fy2025.omb-fy2027-v1.draft.json` | Canonical placeholder receipt scenario. |
| `data/derived/taxpayer_receipt_model/README.md` | Source chain and public-use boundary. |
| `docs/reading/placeholder-visibility-receipt.md` | Reader-facing prototype packet. |
| `reviews/2026-06-22-lane-crosswalk-role-review.md` | Prior crosswalk guardrail review. |

## Decision

The placeholder receipt is acceptable as a design-review prototype.

This review does not approve a public taxpayer receipt, taxpayer calculator,
legal dedication claim, program-level tracing claim, statutory tax lane, or
tax-rate recommendation.

## Findings

| Role | Result |
|---|---|
| Taxpayer Advocate | Pass with caution: the packet is understandable and says the `$1,000` amount is a placeholder, but any public display still needs stronger visual separation between allocation lanes, offsets, and financing context. |
| Budget Accountant | Pass: the prototype labels proportional outlay-share allocation, modeled-not-legal status, fiscal-year basis, net interest, and offsetting receipts. |
| Source Custodian | Pass: the receipt points to recorded OMB FY2027 source IDs and repo-local derived artifacts. |
| Public Goods Steward | Pass: public-purpose lanes are legible and do not imply every lane should become a statutory tax. |
| Program Beneficiary | Pass with caution: Social Security and Medicare are visible, but the packet correctly avoids saying ordinary income tax legally funds those benefits. |
| Compliance Burden | Pass: the prototype is not a filing tool and adds no taxpayer, employer, preparer, or agency calculation requirement. |
| Fiscal Sustainability | Pass: borrowed-share context and net interest are visible. |
| Reform Skeptic | Pass: the packet blocks dollar-tracing wording and preserves the distinction between visibility and dedication. |

## Required Guardrails

### P1: Keep this prototype out of taxpayer-calculator flows

- **Roles**: Taxpayer Advocate, Compliance Burden Reviewer.
- **Evidence**: The scenario uses a placeholder `$1,000` ordinary income-tax
  amount and no taxpayer return data.
- **Risk**: A user could treat the prototype as a liability calculator or a
  personalized tax statement if it appears near filing mechanics.
- **Decision**: Keep it labeled as a placeholder design prototype until a
  separate calculator boundary and tax-advice review exist.

### P1: Do not publish without stronger offset wording

- **Roles**: Budget Accountant, Reform Skeptic.
- **Evidence**: Commerce, Housing, and Market Stability is negative in FY2025,
  and undistributed offsetting receipts are also negative.
- **Risk**: Negative dollar rows can confuse readers if presented as "you paid
  negative dollars" rather than an OMB netting or offset treatment.
- **Decision**: Any public visual receipt must explain negative rows as offsets
  or budget netting before release.

### P2: Add dedicated-financing caveats before public use

- **Roles**: Program Beneficiary Reviewer, Budget Accountant, Reform Skeptic.
- **Evidence**: Social Security and Medicare have distinct financing structures
  and legal statuses in the crosswalk, while the placeholder payment is ordinary
  individual income tax.
- **Risk**: A reader may infer that ordinary income-tax dollars are legally
  dedicated to Social Security or Medicare.
- **Decision**: Public receipt views must separate ordinary income-tax modeled
  allocation from payroll/social-insurance tax financing.

### P2: Keep borrowed share beside the receipt

- **Roles**: Fiscal Sustainability Reviewer, Taxpayer Advocate.
- **Evidence**: The prototype reports FY2025 borrowed share of outlays as
  25.31 percent and income-tax coverage of outlays as 37.88 percent.
- **Risk**: The lane table alone makes current outlays look fully covered by
  current income-tax receipts.
- **Decision**: Borrowed-share and income-tax-coverage context must sit beside
  any public receipt view, not only in a footnote.

## Follow-Up

- Build a public-display mockup only after offset wording and dedicated-financing
  caveats are visible in the artifact.
- Keep the next prototype static and scenario-based; do not add taxpayer input
  fields yet.
