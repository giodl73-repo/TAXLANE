# Wave: Table 2.4 Subcomponent Extraction

## Goal

Extract first OMB Table 2.4 receipt subcomponent records so TAXLANE can
separate payroll/social-insurance receipts and excise receipt streams from
ordinary individual income-tax receipts.

## Thesis

Historical funding claims need subcomponent evidence. Table 2.1 shows parent
receipt categories; Table 2.4 shows which parts of social-insurance and excise
receipts are federal-fund, trust-fund, on-budget, off-budget, or source-specific
streams. TAXLANE should extract these rows before assigning allocation labels to
social-insurance or excise categories.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|---|---|---|
| 01 | Table 2.4 workbook profile | done | Profiled the raw workbook XML, row groups, coverage, and first-year reconciliation targets. |
| 02 | 1940 subcomponent draft rows | done | Extracted first 1940 social-insurance and excise subcomponent draft rows. |
| 03 | 1940 subcomponent source review | done | Verified source anchors and promoted 1940 subcomponent rows to source-reviewed. |
| 04 | Historical funding learning note | done | Summarized what the 1940 subcomponents teach about historical funding. |

## Success Criteria

- Table 2.4 raw artifact remains under source custody with checksum.
- Extraction preserves exact OMB row labels.
- First-year subcomponent totals reconcile to Table 2.1 parent rows.
- Trust-fund and federal-fund distinctions are preserved.
- Missing markers such as `..........` and `*` are not treated as ordinary
  numeric amounts.
- Validation commands pass.

## Non-Goals

- Do not extract all years in one wave.
- Do not use subcomponent rows to create a taxpayer receipt.
- Do not infer statutory dedication beyond the row labels and reviewed concept
  sources.
- Do not merge payroll/social-insurance receipts into individual income taxes.

## Validation

Run:

```powershell
git diff --check
```

For JSONL pulses, parse every record as JSON and reconcile 1940 Table 2.4
totals to Table 2.1 parent rows.
