# OECD COFOG Country Panel, 2022

Machine record:
`data/derived/breadth_benchmark_matrix/oecd_cofog_country_panel.data2022.v1.draft.json`.

These figures are calendar-2022 general-government total expenditure as a
share of GDP. Both numerator and denominator come from the OECD annual national
accounts in current-price national-currency millions.

| COFOG function | USA | DEU | FRA | GBR | SWE | NLD | POL | JPN | KOR | AUS | CAN |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| Defense (GF02) | 2.98 | 1.15 | 1.80 | 2.14 | 1.69 | 1.29 | 1.68 | 0.90 | 2.55 | 2.20 | — |
| Public order and safety (GF03) | 1.84 | 1.58 | 1.71 | 2.04 | 1.37 | 1.80 | 2.26 | 1.16 | 1.22 | 1.93 | — |
| Economic affairs (GF04) | 3.00 | 5.60 | 6.77 | 4.57 | 5.05 | 5.63 | 6.25 | 5.31 | 5.73 | 5.17 | — |
| Environmental protection (GF05) | — | 0.51 | 1.02 | 0.70 | 0.62 | 1.39 | 0.62 | 1.08 | 1.06 | 0.93 | — |
| Health (GF07) | 10.14 | 8.25 | 9.07 | 8.70 | 7.19 | 7.18 | 5.23 | 8.77 | 5.57 | 7.50 | — |
| Education (GF09) | 5.66 | 4.36 | 5.05 | 4.92 | 7.18 | 4.86 | 4.55 | 3.23 | 4.84 | 5.49 | — |
| Social protection (GF10) | 8.30 | 19.85 | 23.81 | 14.91 | 19.01 | 15.73 | 16.68 | 16.30 | 9.02 | 9.71 | — |

Values are percentages of GDP. `—` means absent from the bounded official
response, not zero. The record also contains GF01, GF06, and GF08.

This batch directly improves scope-matched context for defense, justice,
health, education, and environmental protection. Economic affairs and social
protection remain broad context: they cannot substitute for transportation,
agriculture, pensions, or family-support subfunctions. Spending alone is not an
outcome or efficiency measure.

```text
observed spending difference != service difference != efficiency != savings
```
