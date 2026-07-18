# Source Metadata: SRC-IMF-FM-OCT2025-FISCAL-PANEL-2022

| Field | Value |
|---|---|
| `source_id` | `SRC-IMF-FM-OCT2025-FISCAL-PANEL-2022` |
| `publisher` | International Monetary Fund |
| `source_url` | <https://data.imf.org/Datasets/FM> |
| `api_dataflow` | `IMF.FAD:FM_2025_OCT_VINTAGE(1.0.0)` |
| `observed_date` | 2026-07-15 |
| `raw_path` | `data/raw/imf/SRC-IMF-FM-OCT2025-FISCAL-PANEL-2022/2026-07-15/imf-fm-oct2025-fiscal-panel.csv` |
| `checksum_sha256` | `704ce9ab5ebe519471e099abf9cd820acdbd344fa84f85e61431d36345ba80b7` |
| `coverage` | Pinned October 2025 Fiscal Monitor vintage; 2022 general-government revenue, overall balance, primary balance, gross debt, and net debt for 12 countries. |
| `status` | `captured` |
| `notes` | The API returned 1991–2030 despite time parameters; the derived artifact filters `TIME_PERIOD == 2022`. Singapore lacks primary balance and net debt. A tested IMF GFS direct-interest query returned no observations; a separately labeled OECD D.41 source now supplies direct interest for the 11 core peers. Interest is never inferred from balance differences. |
