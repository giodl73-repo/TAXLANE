# Global Country Comparison Coverage

Machine contract:
`data/derived/breadth_benchmark_matrix/global_country_comparison_coverage.v1.draft.json`.

## Panel

The default comparison starts with Germany, France, the United Kingdom,
Sweden, the Netherlands, and Poland in Europe; Japan and Korea in Asia; and
Canada and Australia as additional peers. Singapore is included only when an
official source supplies a genuinely matched definition.

Panels may vary by lane, but a published display needs at least three
comparator countries and should include both a European and an Asian peer when
the same official series covers them.

The [International Comparator Target Rubric](international-comparator-target-rubric.md)
governs how those panels are summarized. The peer median and interquartile range
describe what is typical. A favorable quartile becomes a conditional ambition
only after outcome, floor, transferability, and stability tests. The best
country and the 85th percentile are not default targets.

## Coverage

| Treatment | Lanes |
|---|---|
| Harmonized spending plus service or outcome measures | Health; pensions; family support; revenue and solvency; transportation; education and workforce; justice; agriculture |
| Policy or fiscal-pressure context, not an efficiency target | Defense; net interest |
| Split component comparisons, not one composite rank | Science, energy, and environment; international affairs |
| Exposure-normalized multiyear comparison | Disaster resilience |
| Structured country cases only | Payment integrity; veterans |

The COFOG backbone supplies a common all-government perimeter for many
spending functions. It does not by itself measure service quality or
efficiency. Lane-specific sources add health outcomes, pension design, defense
definitions, transport outcomes, education outcomes, court performance,
research and energy measures, farm support, and development finance.

Payment integrity has no harmonized international fraud or improper-payment
rate. Veterans systems differ in conflict history, eligibility, conscription,
pensions, health-system placement, and dependent coverage. Those two lanes use
structured cases rather than numerical league tables.

## Current Gate

The first observed batch is now available in the
[OECD COFOG Country Panel](oecd-cofog-country-panel-2022.md). It captures ten
top-level all-government spending functions for the 2022 panel with explicit
missingness. Numeric ranking and efficiency claims remain blocked while lane-
specific service and outcome measures are still being attached.

The [hybrid OECD–IMF panel](hybrid-cofog-country-panel-2022.md) then fills
Canada's ten missing functions without averaging overlapping sources. It leaves
U.S. environmental protection unresolved.

The [fiscal country panel](fiscal-country-panel-2022.md) adds matched tax,
revenue, balance, debt, and direct OECD D.41 interest-payable context. Its 11
core peers are complete for those fiscal measures; Singapore retains explicit
gaps. Gross interest is kept separate from net interest, and no peer value is
promoted to a target or savings claim.

The [QPSD maturity panel](qpsd-maturity-country-panel-2022q4.md) adds a partial
2022 Q4 general-government debt-maturity snapshot. Its original- and remaining-
maturity components stay separate, and a combined one-year stock appears only
for the six countries reporting both inputs.

The [SOCX old-age and family panel](socx-oldage-family-country-panel-2022.md)
adds 2022 public spending and cash/service composition for seven countries.
Public old-age spending is not treated as a complete pension-system measure,
and family spending is not treated as an outcome or adequacy score.

The [pension replacement panel](pension-replacement-country-panel-2024.md)
adds a single forward-looking OECD model scenario for 11 peers. The
[age-relative-poverty panel](age-relative-poverty-country-panel.md) separately
adds actual older-person and child income outcomes with explicit country years.
Neither is divided into spending to manufacture an efficiency ranking.

The [program-lane target-cost contract](program-lane-target-cost-contract.md)
defines what must happen next before any comparison can change a rate. It keeps
the 15 analytical lanes distinct from the 17 budget rows and requires a costed
mechanism, outcome floors, federal-scope translation, an assigned receipt base,
and ten-year fiscal reconciliation. Every numeric rate gate remains blocked.

```text
comparison design != observed country result != efficiency != fraud != savings
```
