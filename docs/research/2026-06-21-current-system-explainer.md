# Current-System Explainer

## Decision supported

This note supports the first public explanation of how today's federal
individual income-tax system works. It creates a descriptive bridge between the
history map, budget accounting explainer, and future taxpayer receipt model.

## Research question

What sequence should TAXLANE use to explain current income tax mechanics without
turning a general-fund tax into a misleading program-linked claim?

## Scope

This is not tax advice, a liability calculator, or a complete guide to the
Internal Revenue Code. It is a civic accounting map. It explains which moving
parts a public reader must understand before TAXLANE models "what your income
tax paid for."

## Sources

- `SRC-IRS-RATES-BRACKETS`
- `SRC-IRS-P17`
- `SRC-IRS-F1040`
- `SRC-IRS-CREDITS-DEDUCTIONS`
- `SRC-IRS-WITHHOLDING-ESTIMATOR`
- `SRC-IRS-1040ES`
- `SRC-IRS-DATA-BOOK`
- `SRC-OMB-HIST-2-1-FY2027`
- `SRC-OMB-HIST-3-1-FY2027`
- `SRC-OMB-AP-6-CONCEPTS-FY2027`
- `SRC-OMB-AP-13-FUNDS-FY2027`

## Current-system chain

The current system should be explained as a chain, not as one number:

1. **Income event**: wages, business income, investment income, retirement
   income, or another taxable or partly taxable source.
2. **Return record**: Form 1040 is the annual individual income-tax return.
3. **Income adjustments**: the return and schedules separate income,
   adjustments, deductions, credits, other taxes, payments, and refundable
   credits.
4. **Taxable income**: deductions reduce the income amount subject to tax.
5. **Rate application**: tax brackets apply in layers; the higher rate applies
   only to the income inside that bracket layer.
6. **Credits and other taxes**: credits reduce tax due; some other taxes may be
   added separately.
7. **Payments**: withholding and estimated payments are payment mechanisms, not
   separate public-purpose taxes.
8. **Refund or amount due**: filing reconciles tax liability with payments and
   refundable credits.
9. **Federal receipt accounting**: individual income-tax payments become
   governmental receipts, mostly general fund receipts unless law dedicates a
   particular stream.
10. **Appropriations and outlays**: Congress and standing law determine spending
    authority and outlays; receipts do not by themselves identify which program
    one taxpayer funded.
11. **Deficit or surplus context**: if outlays exceed receipts, borrowing fills
    the gap; a taxpayer receipt that ignores borrowing is incomplete.

## Findings

### TAXLANE-CUR-01: Tax brackets are marginal layers, not a flat label on all
income

- **Sources**:
  - `SRC-IRS-RATES-BRACKETS`
  - `SRC-IRS-P17`
- **Observation**: IRS explains that taxpayers pay tax as a percentage of income
  in layers. When income reaches a higher bracket, only the income in that new
  layer is taxed at the higher rate.
- **Implication**: TAXLANE should avoid public language such as "you are taxed
  at 24 percent" unless it clearly distinguishes marginal rate from average or
  effective tax rate.
- **Confidence**: High.

### TAXLANE-CUR-02: Deductions and credits affect different parts of the
calculation

- **Sources**:
  - `SRC-IRS-CREDITS-DEDUCTIONS`
  - `SRC-IRS-P17`
  - `SRC-IRS-F1040`
- **Observation**: IRS describes deductions as reducing taxable income and
  credits as reducing tax due. Form 1040 and its schedules organize income,
  deductions, tax, credits, payments, and refunds into distinct return sections.
- **Implication**: A TAXLANE public reader should show the chain in order:
  income, adjustments, deductions, taxable income, tax calculation, credits,
  payments, refund or balance due. It should not treat a deduction and a credit
  as equivalent public subsidies without labeling the mechanism.
- **Confidence**: High.

### TAXLANE-CUR-03: Withholding and estimated tax are payment timing systems,
not spending lanes

- **Sources**:
  - `SRC-IRS-WITHHOLDING-ESTIMATOR`
  - `SRC-IRS-1040ES`
- **Observation**: IRS describes withholding as the amount an employer or
  pension provider should withhold each year, and Form 1040-ES as the method
  used to pay tax on income not subject to withholding.
- **Implication**: TAXLANE should explain withholding as collection and cash
  timing. Withholding does not mean the wage earner's dollars are legally routed
  to a specific program.
- **Confidence**: High.

### TAXLANE-CUR-04: Individual income-tax receipts are one revenue source among
several federal sources

- **Sources**:
  - `SRC-OMB-HIST-2-1-FY2027`
  - `SRC-TREASURY-AFG`
  - `SRC-IRS-DATA-BOOK`
- **Observation**: OMB Table 2.1 separates individual income taxes from
  corporation income taxes, social insurance and retirement receipts, excise
  taxes, and other receipts. Treasury's public guide similarly presents federal
  revenue as coming from multiple sources. IRS Data Book statistics describe tax
  administration volume, gross collections, returns, and refunds, but those are
  administration facts, not program allocation rules.
- **Implication**: TAXLANE should keep individual income tax separate from
  payroll/social-insurance taxes and corporate income taxes. A public receipt
  must identify which receipt source is being allocated.
- **Confidence**: High.

### TAXLANE-CUR-05: Most ordinary income tax flows through general revenue,
then spending is authorized through budget law

- **Sources**:
  - `SRC-OMB-AP-6-CONCEPTS-FY2027`
  - `SRC-OMB-AP-13-FUNDS-FY2027`
  - `docs/research/2026-06-21-budget-accounting-explainer.md`
- **Observation**: OMB budget concepts distinguish governmental receipts,
  budget authority, obligations, outlays, federal funds, trust funds, and
  offsetting collections. The budget accounting explainer records the core rule:
  virtually all income taxes are received by the general fund unless law
  dedicates a collection to another fund.
- **Implication**: "What your income tax paid for" should be labeled as
  proportional allocation, deficit-inclusive allocation, or illustrative civic
  education unless a legal dedication is cited.
- **Confidence**: High.

### TAXLANE-CUR-06: A current-system receipt must show deficits, borrowing, and
debt service

- **Sources**:
  - `SRC-OMB-HIST-1-1-FY2027`
  - `SRC-OMB-HIST-3-1-FY2027`
  - `SRC-OMB-AP-6-CONCEPTS-FY2027`
- **Observation**: OMB's fiscal spine records receipts, outlays, and deficits or
  surpluses. OMB function tables include net interest as a spending function.
- **Implication**: A taxpayer-facing current-system receipt should show both
  allocation of current receipts and the borrowed share when outlays exceed
  receipts. Debt service belongs as its own public-purpose lane because it is a
  cost of prior borrowing, not a direct service category.
- **Confidence**: High for the rule; numbers remain future extraction work.

## Public explainer outline

The first reader-facing TAXLANE page should use this order:

1. **Start with the return**: the taxpayer reports income on Form 1040.
2. **Separate income from taxable income**: deductions reduce the tax base.
3. **Apply marginal brackets**: bracket rates apply by layer.
4. **Apply credits and other taxes**: credits reduce tax due; other taxes may
   add to the bill.
5. **Reconcile payments**: withholding and estimated payments are credited
   against the tax.
6. **Move to federal accounting**: tax payments become federal receipts.
7. **Separate receipt sources**: individual income tax is not payroll tax,
   corporate income tax, excise tax, fees, or borrowing.
8. **Explain the general fund**: most income tax is general revenue.
9. **Show spending by outlays**: public-purpose views should use OMB function
   and subfunction outlays.
10. **Label the allocation method**: actual legal dedication, proportional
    outlay allocation, deficit-inclusive model, or proposed reform lane.

## Data fields for future current-system records

| Field | Meaning |
|---|---|
| `tax_year` | Year for taxpayer-side return and rate rules. |
| `fiscal_year` | Year for federal receipts and outlays. |
| `filing_status` | Filing status used for bracket and deduction context. |
| `income_source_type` | Wage, business, capital, retirement, transfer, or other source category. |
| `taxable_income_rule` | Deduction/adjustment treatment and source. |
| `rate_schedule_source_id` | IRS or statutory source for the rate table. |
| `credit_rule_source_id` | IRS/statutory source for credit treatment. |
| `payment_method` | Withholding, estimated tax, direct payment, refundable credit, or other payment mechanism. |
| `receipt_source` | OMB receipt source category. |
| `fund_group` | General fund, special fund, trust fund, or other fund grouping. |
| `allocation_method` | Legal dedication, proportional outlay share, deficit-inclusive model, or proposed lane. |
| `deficit_context` | Surplus/deficit treatment for the fiscal year. |

## What TAXLANE should say carefully

| Loose phrase | Safer TAXLANE wording |
|---|---|
| "You are in the 24 percent bracket." | "Your next dollar in this layer may be taxed at 24 percent; earlier layers use lower rates." |
| "Deductions lower your tax." | "Deductions lower taxable income; credits reduce tax due." |
| "Withholding is the tax." | "Withholding is a payment toward the tax calculated on the return." |
| "Income tax pays for defense." | "A proportional outlay model can allocate part of income-tax receipts to defense, but ordinary income tax is generally not legally dedicated to defense." |
| "Payroll taxes and income taxes are both income taxes." | "Payroll/social-insurance taxes are separate receipt sources and often have dedicated trust-fund treatment." |
| "Your taxes paid all current spending." | "Current spending may be financed by receipts and borrowing; the receipt must show the deficit gap." |

## Adopt now

- Use the current-system chain as the default reader sequence.
- Keep tax-year mechanics separate from fiscal-year budget accounting.
- Require source IDs for every bracket, deduction, credit, receipt, and outlay
  claim.
- Label withholding and estimated tax as payment methods.
- Show the deficit gap before any taxpayer receipt is treated as complete.

## Prototype next

- A plain-language "from paycheck to Treasury receipt" diagram.
- A crosswalk from Form 1040 line groups to TAXLANE explanation fields.
- A current-year receipt mock that uses placeholder amounts but real allocation
  labels.

## Defer

- Exact rate/bracket extraction for all filing statuses.
- Individual taxpayer calculators.
- Public claims about specific program percentages until OMB tables are parsed.
