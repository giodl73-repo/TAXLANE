# OMB Outlays Source Review

## Scope

This review covers only the first OMB outlay-function draft slice:

- Draft records:
  `data/extracted/outlay_function/outlay_function.SRC-OMB-HIST-3-1-FY2027.2026-06-21.draft.jsonl`
- Raw artifact:
  `data/raw/omb/SRC-OMB-HIST-3-1-FY2027/2026-06-21/hist03z1_fy2027.xlsx`
- Metadata:
  `data/metadata/SRC-OMB-HIST-3-1-FY2027.2026-06-21.metadata.md`
- Schema:
  `docs/data/outlays-lanes-schema.md`

It covers 21 extracted rows: fiscal years 1940-1942 for National Defense, Human
resources, Physical resources, Net interest, Other functions, Undistributed
offsetting receipts, and Total Federal outlays.

## Source Custodian Findings

| Check | Result |
|---|---|
| Source ID exists in ledger | Pass: `SRC-OMB-HIST-3-1-FY2027` is recorded. |
| Metadata exists | Pass: metadata file records URL, observed date, raw path, checksum, schema, and status. |
| Raw checksum matches metadata | Pass: SHA-256 is `47864B5078698919C3237038AD7296FB7AEB62F6942C3D8ACCCF98AD0A7122F7`. |
| Source table named | Pass: draft rows identify OMB Historical Table 3.1 FY2027. |
| Row anchors specific | Pass: row anchors use Table row/column ranges. |
| Source values match row anchors | Pass: all 21 draft records match workbook labels and amount cells. |
| Source labels preserved | Pass: labels preserve OMB capitalization, including `Total, Federal outlays`. |
| Negative/offsetting values preserved | Pass: undistributed offsetting receipts remain negative and are explicitly labeled. |
| Dynamic query issue | Not applicable. |

## Validation Evidence

The review script checked:

- raw artifact SHA-256,
- JSONL parse for 21 rows,
- row references from Table rows 4, 5, 14, 22, 25, 32, and 35,
- source labels in column A,
- extracted amount cells for 1940-1942,
- presence of three undistributed offsetting receipt rows.

Result:

```text
hash ok
row_refs ok
rows 21
years 1940 1942
offsetting_rows 3
```

## Decision

The reviewed 1940-1942 OMB outlay-function slice is eligible for
`source-reviewed` status.

This does not review the full OMB Historical Table 3.1 workbook, Table 3.2
subfunctions, program-level tables, or any TAXLANE lane crosswalk.

## Public-Use Caveat

These rows identify broad outlay categories and amounts. They do not by
themselves allocate a taxpayer's income tax payment to a public-purpose lane.
