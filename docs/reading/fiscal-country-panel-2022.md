# Fiscal Country Panel, 2022

Machine record:
`data/derived/breadth_benchmark_matrix/fiscal_country_panel.data2022.v1.draft.json`.

| Country | Tax | Overall balance | Primary balance | Gross interest / GDP | Gross interest / revenue | Net interest / GDP | Gross debt | Net debt |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| United States | 27.61 | -3.72 | -0.99 | 3.66 | 11.07 | 3.31 | 119.10 | 91.98 |
| Germany | 39.65 | -1.91 | -1.35 | 0.70 | 1.50 | 0.50 | 64.44 | 45.89 |
| France | 45.77 | -4.74 | -2.86 | 1.92 | 3.57 | 1.87 | 111.37 | 101.08 |
| United Kingdom | 35.42 | -4.56 | -0.91 | 4.19 | 10.61 | 3.95 | 99.61 | 89.81 |
| Sweden | 42.51 | 1.00 | 1.19 | 0.51 | 1.04 | 0.09 | 33.91 | 9.00 |
| Netherlands | 38.07 | 0.00 | 0.43 | 0.57 | 1.31 | 0.42 | 48.41 | 39.90 |
| Poland | 34.38 | -3.44 | -1.91 | 1.53 | 3.84 | 0.98 | 48.79 | 36.85 |
| Japan | 34.44 | -4.22 | -3.81 | 1.31 | 3.49 | 0.28 | 248.25 | 149.48 |
| South Korea | 31.98 | -1.49 | -1.73 | 1.10 | 4.39 | -0.05 | 49.80 | 6.18 |
| Canada | 33.83 | 0.56 | 0.17 | 2.67 | 6.48 | 1.62 | 104.21 | 13.62 |
| Australia | 29.40 | -2.21 | -1.47 | 1.93 | 5.46 | 1.33 | 50.21 | 31.48 |
| Singapore | — | 1.21 | — | — | — | — | 154.30 | — |

Tax, balances, both interest/GDP columns, and debt are percent of GDP. Gross
interest/revenue is percent of general-government revenue. `—` means missing or
blocked, not zero.

Tax revenue comes from OECD Revenue Statistics. Balances and debt come from the
IMF October 2025 Fiscal Monitor vintage. Direct interest payable comes from
OECD annual general-government accounts transaction D.41, with the already
captured OECD GDP denominator. The revenue ratio combines that observed result
with the separately labeled IMF revenue denominator. Singapore remains a
visible gap; nothing is inferred mechanically from the two balance series.

OECD D.41 is gross interest payable. A separately captured OECD Government at a
Glance measure supplies net interest spending after receipts. Korea's small
negative net value is therefore valid and is not clipped to zero. Neither
general-government measure is equivalent to United States federal net interest.
The panel supports fiscal-pressure context, not a country ranking or target.

Japan and Singapore illustrate why gross debt cannot stand alone: asset
positions and institutional arrangements materially change the interpretation.
No country value is an automatic U.S. target.

```text
peer fiscal position != policy target != available savings
```
