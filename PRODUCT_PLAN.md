# INCOME-TAX Product Plan

## Product thesis

INCOME-TAX should become a clear, cited, public explainer and reform-design
workspace for income taxation. It should help a reader understand what the
income tax is, why it was adopted, how it functions today, and what would change
if the system were represented as distinct taxes that fund distinct public
programs.

## Audience

- Citizens who want a plain-language map of how income tax works.
- Researchers comparing general-revenue taxation with earmarked or
  program-linked taxation.
- Policy designers exploring more legible fiscal systems.
- Portfolio consumers that may later need source-backed civic or public-finance
  knowledge packets.

## Core model

The repo organizes the topic around four connected records:

1. **History record**: major legal, constitutional, political, and fiscal
   milestones in income-tax adoption and expansion.
2. **Current-system record**: taxable income, rates, deductions, credits,
   withholding, enforcement, Treasury receipts, appropriations, and debt.
3. **Public-purpose record**: what a tax is supposed to cover, including public
   goods, constitutional obligations, administration, insurance-like transfers,
   infrastructure, defense, courts, and debt service.
4. **Reform-design record**: options for program-linked taxes, earmarks,
   taxpayer receipts, budget transparency, and safeguards against misleading
   labels.

## Design principles

- Separate descriptive claims from reform proposals.
- Cite primary sources before relying on summaries.
- Distinguish taxes, fees, fines, borrowing, premiums, and transfers.
- Show funding flows without pretending money is not fungible.
- Treat "one income tax vs many program taxes" as a design question with tradeoffs.
- Require every taxpayer-facing allocation claim to identify its method: legal
  dedication, proportional allocation, modeled marginal effect, or illustrative
  civic education.
- Include deficits and borrowing in public explanations so receipts do not make
  current spending look fully tax-funded when it is not.
- Make program-linked tax lanes auditable before treating them as reform
  proposals.
- Run role review before accepting taxpayer-facing receipts or program-linked
  tax lanes.

## Initial waves

| Wave | Purpose |
|---|---|
| Tax civics foundation | Establish repo docs, wave protocol, and first research agenda. |
| History and authority map | Build a cited timeline of income-tax authorization and expansion. |
| Current funding map | Explain how revenue and appropriations work today. |
| Program-linked reform model | Prototype a legible set of tax lanes and taxpayer-facing receipts. |

## Working name

INCOME-TAX is a temporary literal name. Keep the final repo name open until the
research spine shows whether the repo is mainly an income-tax history, a tax
civics explainer, a public-finance transparency standard, or a reform-design
workspace.

## Non-goals

- Do not provide tax, legal, investment, or accounting advice.
- Do not calculate individual taxpayer liability in the foundation wave.
- Do not advocate a final tax rate schedule before the current system and history
  are sourced.
- Do not claim a program-linked tax fixes budget discipline by itself.
- Do not import partisan messaging as fact; record policy arguments as claims
  with sources.

## Dependency placement

INCOME-TAX starts as a Knowledge Systems research/explainer repo. It has no
runtime dependency on portfolio systems during the foundation wave. Later waves
should consider PROOF for markdown/source validation, CROP for corpus indexing,
PEBBLE for portable research packets, and FLETCH for fetchable source registries
after source custody is defined. ROLES is required now as a repo-local artifact
because tax allocation claims need explicit stakeholder and accounting review.

## Foundation validation

```powershell
git diff --check
```
