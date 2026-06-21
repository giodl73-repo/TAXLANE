# Budget Accounting Explainer for Tax Legibility

## Decision supported

This note closes the first budget-accounting gate for TAXLANE foundation work:
define the terms that control whether a taxpayer-facing "what you paid for"
claim is accurate, modeled, or misleading.

## Research question

What does official federal budget accounting say about receipts, outlays, budget
authority, obligations, trust funds, federal funds, and offsetting collections,
and what does that imply for TAXLANE legibility?

## Sources

- OMB, Analytical Perspectives FY2027, Budget Concepts:
  <https://www.whitehouse.gov/wp-content/uploads/2026/04/ap_6_concepts_fy2027.pdf>
- OMB, Analytical Perspectives FY2027, Governmental Receipts:
  <https://www.whitehouse.gov/wp-content/uploads/2026/04/ap_8_receipts_fy2027.pdf>
- OMB, Analytical Perspectives FY2027, Offsetting Collections and Offsetting
  Receipts:
  <https://www.whitehouse.gov/wp-content/uploads/2026/04/ap_9_offsetting_fy2027.pdf>
- OMB, Analytical Perspectives FY2027, Trust Funds and Federal Funds:
  <https://www.whitehouse.gov/wp-content/uploads/2026/04/ap_13_funds_fy2027.pdf>
- Source ledger: `docs/sources/source-version-ledger.md`

## Core definitions

### Governmental receipts

OMB defines governmental receipts as collections from sovereign power:

> Governmental receipts are collections that result from the Government's
> exercise of its sovereign power to tax or otherwise compel payment.

The Governmental Receipts chapter similarly says:

> Governmental receipts are taxes and other collections from the public that
> result from the exercise of the Federal Government's sovereign or governmental
> powers. The difference between governmental receipts and outlays is the surplus
> or deficit.

**Implication**: individual income taxes, corporation income taxes, and social
insurance taxes are receipt sources. A taxpayer-facing view must first identify
which receipt source is being discussed.

### Offsetting collections and offsetting receipts

OMB says money collected by the government is recorded either as governmental
receipts or as offsets to spending:

> The Government records money collected in one of two ways. It is either
> recorded as a governmental receipt and included in the amount reported on the
> receipts side of the budget or it is recorded as an offsetting collection or
> offsetting receipt, which reduces (or "offsets") the amount reported on the
> outlay side of the budget.

OMB distinguishes the two offset types by where the money is credited:

> They are offsetting collections when the collections are authorized to be
> credited to expenditure accounts. Otherwise, they are deposited in receipt
> accounts and called offsetting receipts.

**Implication**: not every public payment is an income tax or even a receipt.
User charges, premiums, fees, rents, royalties, and intragovernmental flows may
appear as offsets to spending. A public receipt must not collapse these into
TAXLANE funding.

### Budget authority

OMB defines budget authority as legal authority to incur obligations:

> Budget authority is the authority provided in law to enter into legal
> obligations that will result in immediate or future outlays of the Government.
> In other words, it is the amount of money that agencies are allowed to commit
> to be spent in current or future years.

**Implication**: Congress can authorize spending before cash outlays occur. A
taxpayer explanation must distinguish authorized spending from actual payments.

### Obligations

OMB describes obligations as legal liabilities that lead to outlays:

> Agencies must record obligations when they incur a legal liability that will
> result in immediate or future outlays.

**Implication**: an obligation is a commitment, not the final payment. A program
tax lane should say whether it funds budget authority, obligations, or outlays.

### Outlays

OMB defines outlays as the spending measure:

> Outlays are the measure of Government spending. They are payments that
> liquidate obligations (other than most exchanges of financial instruments, of
> which the repayment of debt is the prime example). The budget records outlays
> when obligations are paid, in the amount that is paid.

**Implication**: a "what you paid for" view should allocate against outlays only
if it says it is using an outlay-share method. It should not imply that the tax
payment itself legally caused those outlays.

### Federal funds and the general fund

OMB says the Federal funds group covers transactions not required by law to be
recorded in trust funds. The general fund is the key fund for income tax:

> The Federal funds group includes the "general fund," which is used for the
> general purposes of Government rather than being restricted by law to a
> specific program. The general fund is the largest fund in the Government and it
> receives all collections not dedicated for some other fund, including virtually
> all income taxes and many excise taxes. The general fund is used for all
> programs that are not supported by trust, special, or revolving funds.

**Implication**: this is the central fact for TAXLANE. The ordinary income tax
is mostly a general fund revenue source. A program-linked receipt for income tax
is usually a modeled civic allocation unless law creates a dedication.

### Special funds and dedicated collections

OMB says some Federal fund collections are legally dedicated:

> Where the law requires that Federal fund collections be dedicated to a
> particular program, the collections and associated disbursements are recorded
> in special fund receipt and expenditure accounts.

OMB also says money in special fund receipt accounts still needs appropriation:

> Money in special fund receipt accounts must be appropriated before it can be
> obligated and spent.

**Implication**: legal dedication is not the same as automatic spending. A
program tax lane needs both a receipt rule and an outlay/appropriation rule.

### Trust funds

OMB says trust funds are designated by law and often receive dedicated
collections:

> The trust funds group consists of funds that are designated by law as trust
> funds. Like special funds and revolving funds, trust funds receive collections
> that are dedicated by law for specific purposes.

OMB also warns that federal trust funds are not private trusts:

> The Federal Government uses the term "trust fund" differently than the way in
> which it is commonly used.

and:

> In contrast, the Federal Government owns and manages the assets and the
> earnings of most Federal trust funds, and can unilaterally change the law to
> raise or lower future trust fund collections and payments or change the purpose
> for which the collections are used.

**Implication**: trust-fund language can improve legibility, but it can mislead
if readers assume private-trust rights. Program-linked taxes must disclose that
Congress can change the rules unless a binding constitutional or statutory limit
actually applies.

## What we are learning

### 1. "What did my income tax pay for?" has multiple valid answers

The answer depends on the allocation method:

| Method | What it can honestly say | What it cannot say |
|---|---|---|
| Legal dedication | A receipt source is legally credited to a fund or program. | That total spending increased by the same amount. |
| Proportional outlay share | A taxpayer's payment is shown as a share of federal outlays. | That the taxpayer's dollars literally funded each category. |
| Deficit-inclusive share | A taxpayer sees both tax-funded and borrowed shares of spending. | That current receipts covered all current commitments. |
| Program-lane model | A proposed tax base/rate could finance a defined public purpose. | That the lane is self-enforcing without shortfall/surplus rules. |

### 2. General fund status is the default for income tax

OMB's general fund definition says it receives collections not dedicated to other
funds, "including virtually all income taxes." That means the public receipt
standard should start from general revenue and only depart from it when a source
has a legal dedication.

### 3. Public trust requires labels, not just charts

A spending chart can be numerically correct and still misleading. Every
taxpayer-facing allocation needs a visible label:

- statutory dedication,
- proportional outlay allocation,
- deficit-inclusive model,
- illustrative civic education, or
- proposed program lane.

### 4. Trust-fund language needs a warning label

Trust funds are real budget accounts, but OMB explicitly says the federal use of
"trust fund" differs from common private-trust usage. Public docs should explain
that dedicated collections do not necessarily create private ownership,
unchangeable benefits, or automatic spending.

### 5. Program-linked taxes are an accounting design, not just a moral slogan

A serious lane needs at least:

- receipt source,
- tax base,
- rate rule,
- fund/account destination,
- appropriation rule,
- outlay target,
- shortfall rule,
- surplus/reserve rule,
- borrowing rule,
- legislative override rule,
- beneficiary-impact statement,
- compliance-burden statement,
- public audit report.

## Standard for TAXLANE going forward

TAXLANE should not ask "what did income tax pay for?" as a single unqualified
question. It should ask:

1. Which tax or receipt source?
2. Which fiscal year or tax year?
3. Which fund group or account?
4. Which spending measure: budget authority, obligations, or outlays?
5. Which allocation method?
6. Are deficits and borrowing included?
7. Is the claim descriptive, modeled, or proposed reform?

## Adopt now

- Treat individual income tax as a general fund receipt unless a specific legal
  dedication is cited.
- Use outlays for "what government spent money on" views.
- Use allocation-method labels on every taxpayer receipt.
- Put deficit and borrowing context next to any spending allocation.
- Require the Budget Accountant and Reform Skeptic roles before public receipt
  publication.

## Prototype next

- A small taxpayer-receipt method note with two views:
  1. proportional outlay allocation of TAXLANE payment;
  2. deficit-inclusive allocation showing the borrowed share of federal outlays.
- A program-lane schema that includes shortfall, surplus, reserve, override,
  beneficiary, and compliance-burden fields.
- A public-purpose lane crosswalk that maps taxpayer-friendly lane names to OMB
  functions and mandatory/discretionary spending-control labels.

## Defer

- Final repo name.
- Any proposal for actual rates or brackets.
- Any claim that the current income tax literally funds one program category.
