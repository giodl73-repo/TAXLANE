# OMB Fund Group Source Review

## Scope

This review covers only the first OMB fund-group draft slice:

- Draft records:
  `data/extracted/fund_group/fund_group.SRC-OMB-HIST-1-4-FY2027.2026-06-21.draft.jsonl`
- Raw artifact:
  `data/raw/omb/SRC-OMB-HIST-1-4-FY2027/2026-06-21/hist01z4_fy2027.xlsx`
- Metadata:
  `data/metadata/SRC-OMB-HIST-1-4-FY2027.2026-06-21.metadata.md`
- Schema:
  `docs/data/receipts-funds-schema.md`

It covers 11 extracted rows for fiscal year 1934: receipts, outlays, and
surplus/deficit by total, federal funds, trust funds, and interfund
transactions where Table 1.4 provides those columns.

## Source Custodian Findings

| Check | Result |
|---|---|
| Source ID exists in ledger | Pass: `SRC-OMB-HIST-1-4-FY2027` is recorded. |
| Metadata exists | Pass: metadata file records URL, observed date, raw path, checksum, schema, and status. |
| Raw checksum matches metadata | Pass: SHA-256 is `E88662D64700808C15A598FBB9018525CD00152159005010604BEA095B389733`. |
| Source table named | Pass: draft rows identify OMB Historical Table 1.4 FY2027. |
| Row anchors specific | Pass: row anchors use Table row/column ranges. |
| Source values match row anchors | Pass: all 11 draft records match workbook fiscal year and amount cells. |
| Source labels preserved | Pass: source labels preserve Total, Federal Funds, Trust Funds, and Interfund Transactions. |
| Negative values preserved | Pass: interfund transaction amounts remain negative where OMB reports them that way. |
| Dynamic query issue | Not applicable. |

## Validation Evidence

The review script checked:

- raw artifact SHA-256,
- JSONL parse for 11 rows,
- row references from `Table!A5:L5`,
- extracted amounts against workbook cell values.

Result:

```text
hash ok
row_refs ok
rows 11
years [1934]
```

## Decision

The reviewed 1934 OMB fund-group slice is eligible for `source-reviewed` status.

This does not review the full OMB Historical Table 1.4 workbook or establish
legal dedication for any receipt category.

## Public-Use Caveat

Table 1.4 shows fund-group accounting context. It does not by itself prove that
a broad receipt source is legally dedicated to a program or that a taxpayer's
income-tax payment entered a particular fund.
