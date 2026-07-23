# How Taxlane Chooses A Fair International Benchmark

Machine record:
`data/derived/breadth_benchmark_matrix/international_comparator_target_rubric.v1.draft.json`.

## The Rule

Taxlane uses three reference levels:

| Question | Reference | Meaning |
|---|---|---|
| What is typical? | Equal-country peer median and interquartile range | Descriptive eligible-OECD-peer context |
| What looks attainable? | Favorable quartile | Conditional ambition after outcome, floor, stability, and transferability tests |
| Who should we study? | Sustained high performers | Named policy examples, not automatic numeric targets |

The OECD mean is secondary context, not an optimum. The best country is a case
study, not a target. The 85th percentile is not used on the current core panel:
with roughly ten peer observations it is effectively a fragile second-best
value. P85 requires at least 20 peers, 80 percent universe coverage, three
comparable periods, and sensitivity analysis.

## Who Counts As A Peer

The broad reference panel uses every OECD country with a definition-matched
official observation unless a substantive restriction was declared before
viewing results. The United States is shown but excluded from the peer statistic.
Every exclusion and missing value must be disclosed.

Taxlane's named European, Asian, Canadian, and Australian panel makes the
display readable. A separate structural panel tests transferability using
predeclared characteristics such as demographics, federalism, geography,
financing model, hazard exposure, threat exposure, or system design.

Countries described as high performers must qualify on preregistered outcomes,
not merely low spending. Equal-country weighting is the default. Population or
GDP weighting answers a different question and must be shown separately.

## Minimum Evidence

| Claim | Minimum |
|---|---|
| Display only | 3 countries, with Europe and Asia where matched data exist |
| Median comparison | 8 peers and at least 70% of the eligible universe |
| Favorable-quartile comparison | 10 peers plus outcome and floor tests |
| P85 or frontier analysis | 20 peers, 80% coverage, at least 3 periods, and sensitivity tests |

Every result must show N, countries, years, missingness, source, scope, unit,
weighting, quantile method, and whether the United States was excluded from the
benchmark. Missing countries are never imputed into ranks. If a conclusion
changes under the alternate quantile method or leave-one-country-out test, the
result is labeled indeterminate.

## Direction Matters

Higher is not always better and lower is not always better. Spending, taxes,
debt, defense, pension replacement, staffing, and reserve capacity usually need
an outcome dashboard or a policy band. Poverty, avoidable mortality, fatalities,
and delay may have a favorable lower quartile, but only while access, quality,
equity, adequacy, and resilience floors hold.

Payment integrity and veterans use structured cases because their definitions
do not support a fair percentile ranking. Science, energy, environment,
agriculture, and international affairs retain component scorecards rather than
an opaque composite champion.

## Claim Ladder

```text
design
  -> observed comparison
  -> robust favorable-quartile pattern
  -> noncausal performance pattern
  -> transferable exemplar
  -> explicit policy scenario
  -> separate causal and fiscal evidence before efficiency or savings
```

Country comparisons can inform the first five steps. They cannot by themselves
open an efficiency, waste, fraud, or savings claim.

## First Application

The first metric-specific application is
[Family Support Target Rubric Application](family-support-target-rubric-application.md).
It keeps family support at display-only descriptive status because the current
bounded spending panel has six non-US peer observations, below the median and
favorable-quartile claim minimums.
