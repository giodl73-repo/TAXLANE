# Family Support Target Rubric Application

Machine inputs:
`data/derived/breadth_benchmark_matrix/international_comparator_target_rubric.v1.draft.json`,
`data/derived/breadth_benchmark_matrix/socx_oldage_family_country_panel.data2022.v1.draft.json`,
and `data/derived/breadth_benchmark_matrix/age_relative_poverty_country_panel.v1.draft.json`.

## Metric Under Review

This packet applies the international comparator target rubric to family
support. The spending metric is 2022 public family social expenditure as a
share of GDP, split into cash and services. The outcome context is child
relative-income poverty, measured as the share of children living in households
below 50 percent of the national median equivalised disposable income.

The spending and poverty measures are not joined into an efficiency score. They
use different source families and not all countries share a common outcome year.

## Eligible Universe

The current spending panel contains seven observed family-spending countries:
United States, France, United Kingdom, Japan, South Korea, Canada, and
Australia. Germany, Sweden, the Netherlands, Poland, and Singapore remain
missing in the bounded 2022 SOCX response.

The child-poverty panel contains eleven observed countries. Singapore is
missing; Australia uses a 2020 fallback and the other observed peers use 2021.

Because the spending metric has only six non-US peer observations, it does not
meet TAXLANE's median-claim minimum of eight peers or favorable-quartile minimum
of ten peers. The current application therefore stays at display-only
descriptive status.

## Current Descriptive Result

| Country | Family spending, % GDP | Cash | Services | Child relative poverty |
|---|---:|---:|---:|---:|
| United States | 0.66 | 0.05 | 0.61 | 13.99 |
| France | 2.63 | 1.25 | 1.38 | 11.90 |
| United Kingdom | 1.85 | 0.90 | 0.96 | 12.70 |
| Japan | 1.98 | 0.67 | 1.31 | 11.54 |
| South Korea | 1.65 | 0.39 | 1.26 | 9.60 |
| Canada | 1.57 | 1.21 | 0.36 | 9.53 |
| Australia | 1.89 | 0.96 | 0.93 | 13.30 |

The United States is visibly below every observed non-US peer in public family
spending in this bounded panel. Its child relative-poverty value is also higher
than each observed non-US peer in the same display set. That pattern is useful
context for the family-support lane, but it is not yet a policy target,
efficiency estimate, or savings estimate.

## Rubric Decision

```text
family spending input = display-only descriptive comparison
child poverty outcome = separate outcome context
six observed non-US spending peers < median-claim minimum
mixed outcome years = no robust target
spending gap + outcome gap != causal efficiency or recoverable savings
```

Current claim gate: `G1 observed_descriptive`, display-only. TAXLANE may say the
United States is lower than the six observed non-US peers on the bounded 2022
family-spending panel and has a higher child relative-poverty value than those
same displayed peers. TAXLANE may not claim an optimal family-spending rate, a
favorable-quartile target, a causal spending/outcome relationship, fraud, waste,
or recoverable savings.

## Next Evidence Needed

- Broaden the official family-spending universe enough to meet the median
  comparison minimum, including logged missingness and exclusions.
- Add a common-year or sensitivity-tested child-outcome panel.
- Attach childcare access, service quality, labor participation, and adequacy
  floors before any favorable-quartile scenario.
- Test whether cash and service composition changes the interpretation.
- Preserve tax-benefit support as a separate source family unless a same-year
  harmonized series supports integration.
