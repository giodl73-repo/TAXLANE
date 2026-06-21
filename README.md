# INCOME-TAX

INCOME-TAX is a public-policy knowledge repo about the income tax: how it works
today, how it came to exist, what taxes are supposed to fund, and whether the
single income-tax label should be replaced or explained as a set of distinct,
program-linked taxes.

## Thesis

The income tax is usually presented as one tax on income, but public trust and
fiscal legibility improve when taxpayers can see which public purposes a tax is
meant to cover. INCOME-TAX studies three layers:

1. The historical layer: why the modern income tax was passed, which problems it
   was meant to solve, and how its legal and political role changed over time.
2. The current layer: how taxable income, rates, deductions, credits,
   withholding, compliance, appropriations, and general revenue interact today.
3. The reform layer: whether an income-tax system should expose distinct
   funding lanes for programs such as defense, courts, infrastructure, health,
   retirement, debt service, and social insurance.

The repo starts from the practical question: if a tax is supposed to pay for
public goods and lawful public obligations, should income tax be treated as one
general revenue stream or as a transparent portfolio of program taxes?

## Working distinction

A tax is a compulsory public charge used to fund legitimate public obligations.
It is not the same as a user fee, fine, loan, or insurance premium, even when a
tax is politically described that way. Dedicated or earmarked taxes can make
program costs visible, but they can also create false precision because money is
fungible and legislatures still control appropriations.

INCOME-TAX therefore treats program-linked taxation as a design hypothesis, not
as an assumed answer.

## First questions

- What did supporters and opponents say the income tax was supposed to cover
  when it was created?
- Which parts of today's government are actually funded by income-tax receipts,
  payroll taxes, excise taxes, borrowing, fees, and other revenue?
- Where does the current system hide program costs from taxpayers?
- Which program-linked taxes already exist, such as payroll taxes for social
  insurance?
- What would be clearer, fairer, or riskier about splitting income tax into
  visible funding lanes?

## Repository shape

- `PRODUCT_PLAN.md` records the product thesis, non-goals, and wave plan.
- `context/waves/` records implementation waves and pulses.
- `docs/research/` holds cited research notes and decision records.
- `.claude/skills/` contains repo-local wave, pulse, and research workflows.

## Validation

This is a docs-first repo. The foundation validation command is:

```powershell
git diff --check
```
