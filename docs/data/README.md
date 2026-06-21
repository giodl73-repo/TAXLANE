# TAXLANE Data

TAXLANE data work starts with dictionaries, not raw imports. Raw downloaded
spreadsheets, PDFs, or query exports should not be committed until the source
custody rules for that source family are written and role-reviewed.

## Files

| File | Purpose |
|---|---|
| `dictionary.md` | Canonical record families, required fields, extraction rules, and validation gates. |
| `rates-timeline-schema.md` | Tax-year rates timeline row schema and extraction rules. |
| `receipts-funds-schema.md` | Fiscal-year receipt-source and fund-group schemas for OMB tables. |

## Data principles

1. Use source IDs from `docs/sources/source-version-ledger.md`.
2. Keep tax-year records separate from fiscal-year records.
3. Keep descriptive records separate from modeled allocation records.
4. Label every allocation as legal dedication, proportional allocation,
   deficit-inclusive allocation, illustrative education, or proposed reform.
5. Store raw source artifacts only after a custody review says where and how.
