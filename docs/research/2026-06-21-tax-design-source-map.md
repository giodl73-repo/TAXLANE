# Tax Design Source Map and Legibility Principles

## Decision supported

This note supports the first design decision for INCOME-TAX: before renaming or
locking the repo down, define the evidence architecture needed to explain income
tax rates, receipts, uses, accounting, and a taxpayer-facing standard for knowing
what a taxpayer is paying for.

## Research question

What source-backed records and public principles should INCOME-TAX establish so
it can explain the income tax from inception to today and evaluate whether the
income tax should be presented as distinct program-linked taxes?

## Findings

### INCOME-TAX-01: Treat rates as a historical schedule, not a single number

- **Sources**:
  - IRS SOI Historical Table 23:
    <https://www.irs.gov/statistics/soi-tax-stats-historical-table-23>
  - Revenue Act of 1913, 38 Stat. 114:
    <https://www.govinfo.gov/app/details/STATUTE-38/STATUTE-38-Pg114>
  - Sixteenth Amendment, National Archives:
    <https://www.archives.gov/milestone-documents/16th-amendment>
- **Observation**: The income tax begins as a constitutional and statutory
  authority question, then becomes a rate-schedule question. A useful history
  needs the legal authority, the statutory starting point, and the year-by-year
  brackets/rates.
- **Implication**: The first data product should be a rates timeline with columns
  for tax year, legal event, lowest rate, highest rate, bracket thresholds,
  exemption/standard deduction context, tax base notes, and source URL.
- **Confidence**: High for source selection; details must be verified from the
  downloaded tables and statutes before publication.

### INCOME-TAX-02: Receipts require source and fund-group separation

- **Sources**:
  - OMB Historical Tables:
    <https://www.whitehouse.gov/omb/information-resources/budget/historical-tables/>
  - OMB Table 1.1, summary of receipts, outlays, and deficits, 1789-2025:
    <https://www.whitehouse.gov/wp-content/uploads/2026/04/hist01z1_fy2027.xlsx>
  - OMB Table 1.4, receipts, outlays, and deficits by fund group, 1934-2025:
    <https://www.whitehouse.gov/wp-content/uploads/2026/04/hist01z4_fy2027.xlsx>
  - OMB Table 2.1, receipts by source, 1934-2031:
    <https://www.whitehouse.gov/wp-content/uploads/2026/04/hist02z1_fy2027.xlsx>
  - OMB Table 2.2, percentage composition of receipts by source, 1934-2031:
    <https://www.whitehouse.gov/wp-content/uploads/2026/04/hist02z2_fy2027.xlsx>
  - OMB Table 2.4, social insurance, retirement receipts, and excise tax
    composition, 1940-2031:
    <https://www.whitehouse.gov/wp-content/uploads/2026/04/hist02z4_fy2027.xlsx>
- **Observation**: "Income tax receipts" are not enough. The repo needs to
  distinguish individual income taxes from corporation income taxes, social
  insurance/payroll taxes, excise taxes, other receipts, borrowing, and fund
  groups.
- **Implication**: The receipts record should not ask "what did my income tax pay
  for?" until it also records what revenue sources existed that year and whether
  the budget was in deficit or surplus.
- **Confidence**: High for source selection; table parsing remains future work.

### INCOME-TAX-03: "What it paid for" must be modeled as budget accounting, not
literal dollar tracing

- **Sources**:
  - OMB Analytical Perspectives:
    <https://www.whitehouse.gov/omb/information-resources/budget/analytical-perspectives/>
  - OMB Budget Concepts chapter:
    <https://www.whitehouse.gov/wp-content/uploads/2026/04/ap_6_concepts_fy2027.pdf>
  - OMB Governmental Receipts chapter:
    <https://www.whitehouse.gov/wp-content/uploads/2026/04/ap_8_receipts_fy2027.pdf>
  - OMB Offsetting Collections and Offsetting Receipts chapter:
    <https://www.whitehouse.gov/wp-content/uploads/2026/04/ap_9_offsetting_fy2027.pdf>
  - OMB Trust Funds and Federal Funds chapter:
    <https://www.whitehouse.gov/wp-content/uploads/2026/04/ap_13_funds_fy2027.pdf>
- **Observation**: Federal taxes are accounted for through budget concepts such
  as receipts, offsetting collections, outlays, federal funds, trust funds,
  budget authority, obligations, and cash outlays. A taxpayer-facing explanation
  should not imply that one person's dollars are physically routed to one agency
  unless law creates a dedicated fund or formula.
- **Implication**: INCOME-TAX needs an accounting explainer before it creates a
  taxpayer receipt. The standard should say whether an allocation is statutory,
  proportional, modeled, or illustrative.
- **Confidence**: High for source selection; exact definitions should be quoted
  only after reviewing the PDF text.

### INCOME-TAX-04: Uses should be shown by function, subfunction, agency, and
program

- **Sources**:
  - OMB Table 3.1, outlays by superfunction and function, 1940-2031:
    <https://www.whitehouse.gov/wp-content/uploads/2026/04/hist03z1_fy2027.xlsx>
  - OMB Table 3.2, outlays by function and subfunction, 1962-2031:
    <https://www.whitehouse.gov/wp-content/uploads/2026/04/hist03z2_fy2027.xlsx>
  - OMB Table 4.1, outlays by agency, 1962-2031:
    <https://www.whitehouse.gov/wp-content/uploads/2026/04/hist04z1_fy2027.xlsx>
  - OMB Table 8.5, outlays for mandatory and related programs, 1962-2031:
    <https://www.whitehouse.gov/wp-content/uploads/2026/04/hist08z5_fy2027.xlsx>
  - OMB Table 8.7, outlays for discretionary programs, 1962-2031:
    <https://www.whitehouse.gov/wp-content/uploads/2026/04/hist08z7_fy2027.xlsx>
  - OMB Table 11.3, payments for individuals by category and major program,
    1940-2031:
    <https://www.whitehouse.gov/wp-content/uploads/2026/04/hist11z3_fy2027.xlsx>
  - USAspending.gov:
    <https://www.usaspending.gov/>
- **Observation**: "What taxes paid for" can be shown at multiple levels:
  national defense, Social Security, Medicare, net interest, courts,
  infrastructure, agency outlays, and program/account detail. Each level answers
  a different public question.
- **Implication**: The public reader should display a layered explanation:
  function first, then subfunction/program, then agency/account when detail is
  needed. Program-linked taxes should be evaluated at the level where Congress
  can actually define and audit the funding lane.
- **Confidence**: High for source selection; historical coverage differs by
  table and must be recorded.

### INCOME-TAX-05: A taxpayer receipt is useful only if its allocation rule is
explicit

- **Sources**:
  - U.S. Treasury Fiscal Data, Your Guide to America's Finances:
    <https://fiscaldata.treasury.gov/americas-finance-guide/>
  - USAspending.gov:
    <https://www.usaspending.gov/>
  - OMB Historical Tables:
    <https://www.whitehouse.gov/omb/information-resources/budget/historical-tables/>
- **Observation**: A taxpayer-facing receipt can educate people about the scale
  and composition of government. It can also mislead if it hides deficits,
  borrowing, trust funds, or the fact that most general revenues are appropriated
  through the budget rather than tagged to a taxpayer's own payment.
- **Implication**: INCOME-TAX should use receipts as a design output, but every
  receipt must label its method: actual legal dedication, proportional share of
  outlays, marginal-policy model, or illustrative civic education.
- **Confidence**: Medium-high; future work should compare existing official and
  nonprofit taxpayer-receipt methods.

## Proposed records

| Record | Purpose | First source family |
|---|---|---|
| `rates_timeline` | Show legal authority, rates, brackets, exemptions, and tax-base changes from inception. | IRS SOI Table 23; Revenue Act of 1913; National Archives. |
| `receipts_by_source` | Show how much revenue came from individual income taxes versus other sources. | OMB Historical Tables 1.1, 2.1, 2.2. |
| `receipts_by_fund` | Separate general fund, trust fund, and other fund-group accounting. | OMB Historical Tables 1.4; OMB Analytical Perspectives. |
| `outlays_by_function` | Show what the federal government spent money on by public purpose. | OMB Historical Tables 3.1, 3.2. |
| `outlays_by_program` | Show mandatory/discretionary/program detail for taxpayer-facing explanations. | OMB Historical Tables 8.5, 8.7, 11.3; USAspending.gov. |
| `taxpayer_receipt_model` | Convert a person's tax payment into a clearly labeled allocation view. | OMB, Treasury Fiscal Data, USAspending.gov. |
| `program_tax_lane_model` | Evaluate whether income tax should become explicit public-purpose lanes. | Derived from the records above. |

## Legibility principles

1. **People should know the public purpose of taxation.** A tax is more
   legitimate when citizens can connect compulsory payment to lawful public
   obligations and public goods.
2. **People should know the accounting status of the claim.** "This paid for
   X" must identify whether X is legally earmarked, proportionally allocated,
   debt-financed, or a civic-education approximation.
3. **People should see both benefits and obligations.** Public costs include
   services, insurance-like transfers, enforcement, administration, debt service,
   and long-lived capital.
4. **Rates are not enough.** A fair explanation must show the tax base,
   deductions, credits, bracket thresholds, receipts, inflation/GDP context, and
   household incidence where sourced.
5. **Program-linked taxes should be auditable.** If a tax lane claims to fund a
   program, the lane needs a statutory definition, receipt source, outlay target,
   shortfall/surplus rule, and public report.
6. **Fungibility must be disclosed.** Earmarks and trust funds can improve
   visibility, but they do not by themselves prove that total spending changed or
   that the rest of the budget was unaffected.
7. **Deficits belong on the receipt.** A taxpayer receipt that allocates only
   taxes while ignoring borrowing hides part of what current policy consumes.
8. **The standard should be neutral before it is persuasive.** INCOME-TAX should
   first make the current system legible, then compare reform designs.

## Adopt now

- Use the current repo name as a working title only; defer final naming until the
  source model and public reader shape are clearer.
- Build source-backed records before writing advocacy language.
- Make "allocation method" a required field for every taxpayer-facing receipt or
  program-linked tax claim.
- Use OMB historical tables as the default receipts/outlays spine and IRS SOI
  Historical Table 23 as the default rates spine.

## Prototype behind a boundary

- A taxpayer receipt that allocates individual income taxes proportionally across
  outlay functions.
- A second receipt view that includes borrowing/deficit share.
- Program tax lanes for defense, justice/courts, infrastructure, health,
  retirement/social insurance, debt service, and general administration.

## Reject or defer

- Do not claim a specific taxpayer's dollars literally paid a specific program
  without statutory dedication.
- Do not choose final tax rates, bracket designs, or program-lane percentages in
  this foundation wave.
- Do not treat payroll taxes and individual income taxes as interchangeable; they
  need separate source and fund records.

## Next validation steps

1. Download and parse the IRS and OMB spreadsheets into a local, cited data
   dictionary.
2. Draft the budget accounting explainer from OMB Analytical Perspectives.
3. Build the first taxpayer-receipt method note with explicit allocation rules.
4. Compare at least two existing taxpayer-receipt designs before choosing the
   repo's public standard.

