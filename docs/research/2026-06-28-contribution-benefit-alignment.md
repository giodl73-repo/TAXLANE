# Contribution-Benefit Alignment

## Purpose

The next fairness question is not only "where does the money go?" It is also:

> Is each person's visible charge proportional to what goes in, and does the
> lane honestly say who benefits?

TAXLANE should answer that with alignment labels, not with one universal rule.
Some lanes are contributory social insurance. Some are redistribution. Some are
public goods. Some are financing costs. Treating all of them as if each person
gets back exactly what they paid would be false precision.

## Alignment Types

| Type | Applies to | Rule |
|---|---|---|
| `contributory_earned_benefit` | Social Security, Medicare HI, unemployment insurance | Show worker/employer contribution base, legal dedication, solvency gap, and benefit formula. |
| `premium_plus_general_support` | Medicare SMI-style benefits | Show premiums, general revenue subsidy, enrollee basis, and taxpayer support. |
| `redistributive_transfer` | Medicaid-like health, income support, food/housing aid | Show who pays, who receives, eligibility purpose, and outcome floor. |
| `public_good` | defense, courts, science, infrastructure | Show per-person civic cost, not personal benefit matching. |
| `financing_cost` | debt interest | Show per-person burden of past borrowing, not a benefit. |

## Medicare Correction

The current reader shortcut "Medicare = payroll + premiums + general revenue"
is directionally right but not precise enough for alignment.

Medicare must be split:

| Component | Alignment treatment |
|---|---|
| Hospital Insurance / Part A | Contributory payroll-financed social insurance; track HI payroll receipts, trust-fund status, taxable payroll base, and beneficiary hospital benefit commitments. |
| Supplementary Medical Insurance / Parts B and D | Premium plus general support; track enrollee premiums, general revenue transfers, and subsidy share. |
| Medicare-wide efficiency | Health cost-per-outcome pressure; savings require coverage/access/outcome floors. |

So the clean public claim is:

> Medicare is not one aligned payroll lane. Part A is closest to a contributory
> lane; Parts B/D are intentionally subsidized by premiums plus general revenue.

## Per-Person Display Rules

Per-person receipts should show the denominator:

| Display | Denominator |
|---|---|
| Per resident | total population |
| Per taxpayer | tax returns or adult tax units |
| Per worker | covered workers / payroll base |
| Per beneficiary | program beneficiaries |
| Per enrollee | program enrollees |
| Per household | households |

Never display a per-person number without saying which person-count is used.

## Alignment Questions

For every lane, ask:

1. What is the payer base?
2. What is the beneficiary base?
3. Is the lane contributory, redistributive, public-good, or financing-cost?
4. Is there a legal dedication or only a modeled allocation?
5. Is the current receipt flow enough to cover the benefit or public purpose?
6. If not, is the correction a rate change, base change, efficiency gain,
   benefit/scope change, or general-revenue subsidy?

## Direction

- **Social Security:** align by fixing the capped wage base before raising the
  ordinary rate.
- **Medicare:** split Part A from Parts B/D; stop pretending one Medicare tax
  funds the whole program.
- **Health outside Medicare:** treat as redistributive/coverage support and
  efficiency pressure, not a pay-in/pay-out account.
- **Public goods:** show per-person civic cost, not personal matching.
- **Debt interest:** show the per-person cost of past borrowing.
