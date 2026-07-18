# Hybrid OECD–IMF COFOG Country Panel, 2022

Machine record:
`data/derived/breadth_benchmark_matrix/hybrid_cofog_country_panel.data2022.v1.draft.json`.

OECD remains the primary source for every available cell. IMF fills Canada's
ten missing cells. Korea remains on OECD. No overlapping observations are
averaged.

| Function | USA | Canada | Germany | France | UK | Japan | Korea | Australia |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| Defense | 2.98 | 0.79 | 1.15 | 1.80 | 2.14 | 0.90 | 2.55 | 2.20 |
| Public order and safety | 1.84 | 1.65 | 1.58 | 1.71 | 2.04 | 1.16 | 1.22 | 1.93 |
| Economic affairs | 3.00 | 3.44 | 5.60 | 6.77 | 4.57 | 5.31 | 5.73 | 5.17 |
| Environmental protection | — | 0.92 | 0.51 | 1.02 | 0.70 | 1.08 | 1.06 | 0.93 |
| Health | 10.14 | 8.41 | 8.25 | 9.07 | 8.70 | 8.77 | 5.57 | 7.50 |
| Education | 5.66 | 4.38 | 4.36 | 5.05 | 4.92 | 3.23 | 4.84 | 5.49 |
| Social protection | 8.30 | 11.26 | 19.85 | 23.81 | 14.91 | 16.30 | 9.02 | 9.71 |

Values are percent of GDP. The complete machine record also contains Sweden,
the Netherlands, Poland, and GF01, GF06, and GF08.

Across 89 populated source overlaps, 69 are within 0.05 percentage points and
75 within 0.10 points. The largest difference is Japan social protection at
0.706 points. This is why the sources are not averaged.

U.S. environmental protection remains missing. IMF reports exact zero throughout
1972–2024, which is a classification limitation rather than evidence that the
United States spends nothing on environmental protection.

```text
source gap fill != blended estimate != efficiency != savings
```
