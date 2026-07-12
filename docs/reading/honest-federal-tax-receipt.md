# An Honest Federal Tax Receipt

## The Point

A conventional taxpayer receipt turns one tax payment into a tidy list of
programs. That presentation is easy to read, but it can imply that a person's
income-tax dollars were legally traced to those programs or that every program
works like a personal pay-in/pay-out account.

An honest receipt shows the relationship instead of hiding it:

> What goes in, who benefits, how the program is financed, which denominator is
> used, and whether the displayed amount is legally dedicated, modeled, or only
> an illustration.

This prototype is a federal financing receipt, not an individual tax bill. It
uses current TAXLANE source records to demonstrate the labels that should travel
with public tax-and-spending numbers.

## Before And After

| Conventional receipt | Honest receipt |
|---|---|
| "You paid $X for Medicare." | Medicare must be split: Part A is primarily payroll-financed; Parts B and D combine premiums with large general-revenue contributions. A displayed amount must identify the part, source, year, and denominator. |
| "You paid $X for defense." | Defense is a collective public good. A per-resident amount is civic-cost context, not equal tax liability, personal benefit, or legal dedication of income-tax dollars. |
| "You paid $X in interest." | Interest is the financing cost of past borrowing. It is not a current program benefit. |

The second version is less tidy. It is also much harder to misunderstand.

## Prototype Receipt

| Public-purpose lane | Financing relationship | Who benefits | Honest display | Status |
|---|---|---|---|---|
| Social Security | Contributory social insurance financed mainly through dedicated payroll taxes; the wage cap and solvency gap remain material context. | Covered workers, eligible beneficiaries, and dependents. | Show per-covered-worker and per-beneficiary views with the basis and benefit rules visible. Do not describe benefits as a personal account balance. | Denominators sourced; a combined personal-dollar card is not asserted here. |
| Medicare Part A / HI | Primarily payroll-financed social insurance. | Part A enrollees and eligible beneficiaries. | CY2025 Part A expenditures were about **$6,428 per Part A enrollee**. This is program context, not what an enrollee personally paid or received. | Source-basis context: 2026 Medicare Trustees Report, CY2025 expenditure and enrollment values. |
| Medicare Parts B and D / SMI | Premiums plus general-revenue contributions; not payroll-funded pay-in/pay-out lanes. | Part B and Part D enrollees, with broad public subsidy. | CY2025 government support was about **$6,654 per Part B enrollee** and **$2,623 per Part D enrollee**. Keep premiums, government contributions, and state payments separate. | Source-basis context: 2026 Medicare Trustees Report, CY2025 financing and enrollment values. |
| Defense-Military | General public financing for a collective public good; ordinary income-tax dollars are not legally traced to this lane. | The public collectively. | FY2025 outlays equal about **$2,541 per CY2025 resident** as a civic-cost illustration. | Cross-basis illustration: FY2025 OMB Table 3.2 divided by Census July 1, 2025 population. |
| Health and income-support programs outside Medicare | General-revenue and other financing supporting eligible populations; the relationship is redistributive rather than personal matching. | Eligible beneficiaries and the broader public through coverage and risk protection. | Use program-specific enrollee or beneficiary denominators, funding sources, and outcome floors. Do not infer waste or personal return from aggregate spending. | Lane-specific cards remain gated by program-level evidence and denominators. |
| Gross Treasury interest | General financing cost created by accumulated borrowing. | Creditors are paid; taxpayers and residents bear the fiscal constraint. | FY2025 outlays equal about **$3,557 per CY2025 resident** as a civic financing-cost illustration. | Cross-basis illustration: FY2025 OMB Table 3.2 divided by Census July 1, 2025 population. |

## The Five Labels

No public amount should appear without these labels:

1. **Payer base:** the tax, premium, borrowing, or other receipt source.
2. **Beneficiary base:** the direct beneficiary group or the public collectively.
3. **Relationship:** contributory, premium-plus-general-support,
   redistributive, public-good, or financing-cost.
4. **Denominator:** taxpayer, return, worker, beneficiary, enrollee, resident,
   or household, with its source year.
5. **Allocation status:** legally dedicated, source-basis context, modeled
   allocation, cross-basis illustration, or proposed reform.

If one of those labels is missing, the amount is not ready to function as a
taxpayer receipt.

## What This Prototype Does Not Claim

- It does not trace an individual's income-tax payment through the Treasury.
- It does not claim equal tax liability or equal personal benefit.
- It does not turn aggregate outlays into findings of waste or effectiveness.
- It does not prove that program-linked tax lanes would improve trust.
- It does not replace a tax return, benefits statement, legal appropriation, or
  individualized calculator.

The immediate proposal is narrower: public tax receipts should disclose their
financing and allocation logic. Replacing the single income-tax label with
formal program-linked taxes remains a design hypothesis to test separately.

## Evidence Trail

- `docs/reading/aligned-contribution-receipt.md`
- `docs/reading/per-unit-receipt-cards.md`
- `data/derived/denominator_requirements/per_unit_receipt_cards.v1.draft.jsonl`
- `data/derived/contribution_alignment/contribution_alignment.fy2025.v1.draft.jsonl`
- `docs/sources/source-version-ledger.md`

