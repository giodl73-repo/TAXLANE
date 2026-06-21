# TAXLANE Review Organization

TAXLANE roles represent public-interest review lenses for tax history,
budget accounting, taxpayer legibility, and reform design.

These roles are AI-simulated review lenses. They are not real people and do not
imply external endorsement, professional tax advice, legal advice, accounting
advice, or representation of any government office.

## Panel

| ID | Role | Lens |
|---|---|---|
| [T-1](T-1-taxpayer-advocate.md) | Taxpayer Advocate | Whether ordinary taxpayers can understand what they owe, why they owe it, and what public purposes are claimed. |
| [T-2](T-2-budget-accountant.md) | Budget Accountant | Receipts, outlays, trust funds, deficits, borrowing, and allocation-method labels. |
| [T-3](T-3-source-custodian.md) | Source Custodian | Primary sources, citations, historical uncertainty, and source-rights boundaries. |
| [T-4](T-4-public-goods-steward.md) | Public Goods Steward | Whether tax claims connect to legitimate public obligations, public goods, administration, and rule-of-law functions. |
| [T-5](T-5-program-beneficiary.md) | Program Beneficiary Reviewer | Whether program-linked tax lanes reflect service continuity, eligibility, and impacts on people who rely on programs. |
| [T-6](T-6-compliance-burden.md) | Compliance Burden Reviewer | Withholding, filing, employer burden, taxpayer time, and complexity costs. |
| [T-7](T-7-fiscal-sustainability.md) | Fiscal Sustainability Reviewer | Deficits, debt service, long-term commitments, and intergenerational burden. |
| [T-8](T-8-reform-skeptic.md) | Reform Skeptic | Earmark risks, fungibility, false precision, partisan framing, and unintended consequences. |

## Review order

1. Use Source Custodian before accepting historical, legal, or fiscal claims.
2. Use Budget Accountant before publishing any "what your tax paid for" claim.
3. Use Taxpayer Advocate and Public Goods Steward before public-facing explainers.
4. Use Program Beneficiary, Compliance Burden, Fiscal Sustainability, and Reform
   Skeptic before proposing program-linked tax lanes.

## Output

Reviews land in `reviews/` and should name findings by severity:

| Severity | Meaning |
|---|---|
| P1 | Blocks publication because the artifact is unsourced, misleading, or violates a stated boundary. |
| P2 | Should fix before relying on the artifact for design decisions. |
| P3 | Useful refinement that can wait. |

## Acceptance rule

No taxpayer-facing receipt, public-purpose allocation, or program-linked tax lane
is accepted until Budget Accountant and Reform Skeptic both confirm the
allocation method is explicit and the fungibility/deficit caveats are visible.
