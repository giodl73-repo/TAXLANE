# Wave: Table 2.2 Receipt Share Extraction

## Goal

Extract OMB Historical Table 2.2 receipt-source percentage rows into draft
`receipt_source` JSONL records.

## Thesis

The income-tax allocation model needs context about how individual income taxes
fit among all federal receipt sources over time. Table 2.2 provides the
percentage composition of receipts by source and should be represented as
separate percent records, not merged with Table 2.1 amount rows.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|---|---|---|
| 01 | Table 2.2 receipt shares | done | Added Rust extraction for Table 2.2 share-of-total records and a source profile. |

## Success Criteria

- `cargo run -p taxlane-tools -- receipt-source table-2-2` writes draft JSONL
  and profile outputs.
- `cargo run -p taxlane-tools -- receipt-source table-2-2 --check` detects
  stale outputs.
- Receipt-source share records keep `measure = "share_of_total"` with null
  amounts and percent values.
- The transition-quarter row is excluded because it is not a fiscal year.
- Estimate years are labeled as estimates.

## Non-Goals

- Do not source-review the Table 2.2 rows in this pulse.
- Do not change Table 2.1 amount rows.
- Do not create a new derived model.

## Validation

Run:

```powershell
cargo run -p taxlane-tools -- receipt-source table-2-2 --check
cargo fmt --check
cargo test
git diff --check
```
