# Peer-informed savings and reinvestment profiles

Taxlane can now answer a more useful hypothetical question:

> If a concrete package actually produced net savings after protecting and
> expanding selected priorities, what would happen to the scoped ordinary-
> income rate schedule?

The four profiles are goal-seeking envelopes. Their spending amounts are not
forecasts, scores, recommendations, or admitted Taxlane savings. Their rate
implications are model results conditional on achieving the full stated net
savings without double counting.

## The profile ladder

| Profile | Gross savings goal | Reinvestment | Net savings goal | Remaining target | Central schedule |
|---|---:|---:|---:|---:|---|
| Focused | $225B | $75B | **$150B** | $663.727B | **18.9/20.9/30.9/32.9/40.9/43.9/45.9** |
| Balanced | $450B | $150B | **$300B** | $513.727B | **16.8/18.8/28.8/30.8/38.8/41.8/43.8** |
| Ambitious | $700B | $250B | **$450B** | $363.727B | **14.8/16.8/26.8/28.8/36.8/39.8/41.8** |
| Transformative | $950B | $350B | **$600B** | $213.727B | **12.8/14.8/24.8/26.8/34.8/37.8/39.8** |

Each schedule is the smallest tested tenth-point uniform uplift that clears its
remaining central target after Taxlane's $0.077B administration ceiling and
first-order debt sensitivity. The immediately lower tested candidate fails in
every profile. These are central behavioral results, not behavior-robust or
official scores.

## Taxpayer illustrations

The same six taxable-ordinary-income examples make the conditional rate dividend
visible. Values are ordinary-bracket tax only.

| Profile input | Current law | No-savings Taxlane | Focused | Balanced | Ambitious | Transformative |
|---|---:|---:|---:|---:|---:|---:|
| Single, $30,000 taxable | $3,352 | $6,652 | $6,022 | $5,392 | $4,792 | $4,192 |
| Single, $75,000 | $11,212 | $19,462 | $17,887 | $16,312 | $14,812 | $13,312 |
| Single, $150,000 | $28,598 | $45,098 | $41,948 | $38,798 | $35,798 | $32,798 |
| Joint, $100,000 | $11,504 | $22,504 | $20,404 | $18,304 | $16,304 | $14,304 |
| Joint, $250,000 | $45,196 | $72,696 | $67,446 | $62,196 | $57,196 | $52,196 |
| Joint, $600,000 | $147,538.50 | $213,538.50 | $200,938.50 | $188,338.50 | $176,338.50 | $164,338.50 |

Because thresholds stay unchanged and each profile uses a uniform uplift, the
narrow difference from current law equals taxable ordinary income multiplied by
8.9%, 6.8%, 4.8%, or 2.8%. It is not a complete return, effective tax rate, or
benefit-incidence calculation.

## What changes

The profiles direct gross savings goals toward three initiative families:

1. health provider, pharmaceutical, and administrative cost convergence;
2. strategy-led defense procurement and force-posture reform; and
3. owner-attributed payment, procurement, subsidy, and operating reforms.

They hold reinvestment in an adaptive needs reserve rather than pre-committing
it to family, education, transportation, or any other lane. In each budget
cycle, eligible service lanes compete on current pain, floor risk, urgency,
expected marginal outcome, evidence, equity, delivery capacity, and whole-
system cost. A lane—including education—may receive no increase.

The reserve is $75B, $150B, $250B, or $350B across the four profiles. Its total
is fixed for each hypothetical, so this allocation change does not alter net
savings or the conditional rate schedules. See the
[adaptive needs and balance cycle](adaptive-needs-and-balance-cycle.md).

## Why this is peer-informed, not peer-copied

The captured COFOG evidence places U.S. health and defense composition above
the displayed peers and social protection below them. Current OECD material
likewise describes U.S. composition as skewed toward health and defense and
away from social insurance. That supports the direction of inquiry, not the
dollar values in these profiles.

Cross-country statutory tax rates are not directly comparable without bases,
deductions, credits, payroll and consumption taxes, benefits, household type,
and government perimeter. These profiles therefore move the Taxlane schedule
toward current law as net savings rise; they do not claim to reproduce another
country's tax system.

## Admission boundary

All four profiles still record **$0.000B admitted savings**. Before an amount
can enter the canonical Taxlane package, the initiative needs a same-year
crosswalk, concrete policy instruments, current-law score, service and
beneficiary floors, cost-shift analysis, implementation and distribution work,
owner attribution, overlap removal, endogenous NET recomputation, and a fresh
REV run.

Machine record:
[`peer_informed_savings_and_reinvestment_profiles.ty2026.v1.draft.json`](../../data/derived/breadth_benchmark_matrix/peer_informed_savings_and_reinvestment_profiles.ty2026.v1.draft.json)

Generator:
[`build_peer_savings_profiles.py`](../../experiments/rev-level-3-taxcalc/build_peer_savings_profiles.py)
