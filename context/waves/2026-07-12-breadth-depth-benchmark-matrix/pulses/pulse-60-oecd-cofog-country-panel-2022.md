# Pulse 60: OECD COFOG Country Panel, 2022

## Goal

Move the global comparison contract from source planning to the first captured,
scope-matched multi-country spending observations.

## Result

- Captures two narrow OECD SDMX responses: 2022 general-government COFOG
  divisions and same-year current-price GDP.
- Covers the United States, six European peers, Japan, Korea, Australia, and a
  Canadian placeholder.
- Produces 99 observed country/function cells and 11 explicit missing cells.
- Preserves all ten Canadian function cells and U.S. environmental protection
  as `missing_not_imputed`.
- Derives percent GDP only where numerator and denominator share country, year,
  current-price basis, national currency, and OECD source family.
- Adds visible comparisons for defense, public order and safety, health,
  education, and environmental protection.
- Keeps economic affairs and social protection as broad context rather than
  relabeling them transportation, agriculture, pensions, or family support.
- Keeps rankings, causal efficiency, fraud, and savings blocked.

## Artifacts

- `data/raw/oecd/SRC-OECD-COFOG-GLOBAL-PANEL-2022/2026-07-15/`
- `data/metadata/SRC-OECD-COFOG-GLOBAL-PANEL-2022.2026-07-15.metadata.md`
- `data/derived/breadth_benchmark_matrix/oecd_cofog_country_panel.data2022.v1.draft.json`
- `data/derived/breadth_benchmark_matrix/oecd_cofog_country_panel.schema.md`
- `docs/reading/oecd-cofog-country-panel-2022.md`

## Next Gate

Attach lane-specific service and outcome measures, then capture specialist
datasets for revenue/debt, pensions/family support, transport, disasters,
agriculture, aid, veterans, and payment integrity.

```text
observed spending difference != service difference != efficiency != savings
```
