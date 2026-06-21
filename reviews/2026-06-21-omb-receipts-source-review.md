# OMB Receipts Source Review

## Scope

This review covers only the first OMB receipt-source draft slice:

- Draft records:
  `data/extracted/receipt_source/receipt_source.SRC-OMB-HIST-2-1-FY2027.2026-06-21.draft.jsonl`
- Raw artifact:
  `data/raw/omb/SRC-OMB-HIST-2-1-FY2027/2026-06-21/hist02z1_fy2027.xlsx`
- Metadata:
  `data/metadata/SRC-OMB-HIST-2-1-FY2027.2026-06-21.metadata.md`
- Schema:
  `docs/data/receipts-funds-schema.md`

It covers 18 extracted rows: fiscal years 1934-1936 for individual income taxes,
corporation income taxes, social-insurance and retirement receipts, excise
taxes, other receipts, and total receipts.

## Source Custodian Findings

| Check | Result |
|---|---|
| Source ID exists in ledger | Pass: `SRC-OMB-HIST-2-1-FY2027` is recorded. |
| Metadata exists | Pass: metadata file records URL, observed date, raw path, checksum, schema, and status. |
| Raw checksum matches metadata | Pass: SHA-256 is `1212DA86947A71D9A0268E23D3DE789402BCFB80B308CD48A32926A65B2E7BC3`. |
| Source table named | Pass: draft rows identify OMB Historical Table 2.1 FY2027. |
| Row anchors specific | Pass: row anchors use Table row ranges and column labels. |
| Source values match row anchors | Pass: all 18 draft records match workbook fiscal year and amount cells. |
| Source labels preserved | Pass: labels preserve OMB wording, including footnote markers in source labels. |
| Footnote or special markers documented | Pass: footnote markers in labels are preserved; no dotted blank values appear in this slice. |
| Dynamic query issue | Not applicable. |

## Validation Evidence

The review script checked:

- raw artifact SHA-256,
- JSONL parse for 18 rows,
- row references from `Table!A5:K7`,
- category-to-column mapping for columns B, C, D, G, H, and I,
- extracted amounts against workbook cell values.

Result:

```text
hash ok
row_refs ok
rows 18
years 1934 1936
```

## Decision

The reviewed 1934-1936 OMB receipt-source slice is eligible for
`source-reviewed` status.

This does not review the full OMB Historical Table 2.1 workbook. Fund allocation
status remains `unknown` until fund-group and budget-concept review occurs.

## Public-Use Caveat

These rows identify receipt categories and amounts. They do not show legal
dedication, taxpayer-dollar tracing, or program allocation.
