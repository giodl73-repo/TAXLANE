# OMB Table 1.4 Fund-Group Profile

## Artifact

- Source ID: `SRC-OMB-HIST-1-4-FY2027`
- Raw artifact:
  `data/raw/omb/SRC-OMB-HIST-1-4-FY2027/2026-06-21/hist01z4_fy2027.xlsx`
- SHA-256:
  `E88662D64700808C15A598FBB9018525CD00152159005010604BEA095B389733`

## Workbook Shape

| Sheet | Rows | Columns | Use |
|---|---:|---:|---|
| `Table` | 98 | 12 | Receipts, outlays, and surplus/deficit by fund group. |

## Table Regions

| Rows | Region | Extraction rule |
|---|---|---|
| 1 | Title | Preserve as table title in metadata or notes. |
| 2 | Units note | Amounts are in millions of dollars. |
| 3-4 | Header rows | Build measure and fund-group meanings from both rows. |
| 5-98 | Fiscal-year data rows | Extract fiscal-year records, 1934-2025. |

## Data Columns

| Excel column | Measure | Source fund label | Draft field mapping |
|---|---|---|---|
| A | year | Fiscal Year | `fiscal_year` |
| B | receipts | Total | `fund_group = "total"` |
| C | receipts | Federal Funds | `fund_group = "federal-funds"` |
| D | receipts | Trust Funds | `fund_group = "trust-funds"` |
| E | receipts | Interfund Transactions | `fund_group = "interfund-transactions"` |
| F | outlays | Total | `fund_group = "total"` |
| G | outlays | Federal Funds | `fund_group = "federal-funds"` |
| H | outlays | Trust Funds | `fund_group = "trust-funds"` |
| I | outlays | Interfund Transactions | `fund_group = "interfund-transactions"` |
| J | surplus_deficit | Total | `fund_group = "total"` |
| K | surplus_deficit | Federal Funds | `fund_group = "federal-funds"` |
| L | surplus_deficit | Trust Funds | `fund_group = "trust-funds"` |

## Extraction Hazards

- Interfund transactions are negative in many rows and should not be silently
  folded into `other`.
- Table 1.4 fund groups are accounting categories. They do not by themselves
  prove legal dedication for a broad receipt source.
- Table 1.4 does not expose `general-fund` as a separate column; it uses the
  broader `Federal Funds` category.
- Trust fund does not mean private trust ownership.

## Reconciliation Boundary

For each fiscal year:

- Total receipts should reconcile to Table 1.1 total receipts.
- Total outlays should reconcile to Table 1.1 total outlays.
- Total surplus/deficit should reconcile to Table 1.1 total surplus/deficit
  within displayed precision.
