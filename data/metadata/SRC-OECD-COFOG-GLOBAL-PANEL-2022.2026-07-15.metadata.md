# Source Metadata: SRC-OECD-COFOG-GLOBAL-PANEL-2022

| Field | Value |
|---|---|
| `source_id` | `SRC-OECD-COFOG-GLOBAL-PANEL-2022` |
| `publisher` | Organisation for Economic Co-operation and Development |
| `source_url` | <https://data-explorer.oecd.org/vis?df%5Bag%5D=OECD.SDD.NAD&df%5Bds%5D=DisseminateFinalDMZ&df%5Bid%5D=DSD_NASEC10%40DF_TABLE11> |
| `api_dataflows` | `OECD.SDD.NAD,DSD_NASEC10@DF_TABLE11,1.1` and `OECD.SDD.NAD,DSD_NAMAIN10@DF_TABLE1_EXPENDITURE,2.0` |
| `observed_date` | 2026-07-15 |
| `capture_method` | Two narrow OECD SDMX REST queries fetched with PowerShell `Invoke-WebRequest` and `Accept: text/csv`. |
| `raw_path` | `data/raw/oecd/SRC-OECD-COFOG-GLOBAL-PANEL-2022/2026-07-15/` |
| `checksums_sha256` | `oecd-cofog-panel-2022.csv`: `66d0af19fea30a0390240e6ef558148f83eec9285acb8c8bce75b243c0817fd6`; `oecd-gdp-panel-2022.csv`: `5ad56a019e9d2a03423604f1c5fe6292c4df73548ac1fb1f36da4613acc960c5` |
| `coverage` | Calendar 2022 general-government total expenditure for COFOG divisions GF01-GF10 and same-year current-price GDP for the United States plus ten selected peers. |
| `status` | `captured` |
| `notes` | COFOG spending and GDP are both current-price national-currency millions, so their ratio produces percent of GDP without currency conversion. Canada has GDP but no 2022 Table 11 observations in the bounded response; the United States has no GF05 row. Missing values are retained and never imputed. |
