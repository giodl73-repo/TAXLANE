# Wave: Table 2.4 Milestone Years

## Goal

Extract a small set of OMB Table 2.4 milestone years that show how historical
funding streams changed across social-insurance and excise receipts.

## Thesis

The 1940 slice proved the extraction path and showed early fund splits. The next
useful historical-funding step is not all years; it is milestone years where new
receipt streams or trust-fund excise categories appear. This keeps the data
reviewable while showing historical change.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|---|---|---|
| 01 | Milestone year scan | done | Identified candidate years and row groups for expansion. |
| 02 | 1957 and 1966 social-insurance rows | done | Extracted milestone rows for disability insurance and hospital insurance emergence. |
| 03 | 1957 and 1983 excise trust rows | done | Extracted milestone rows for transportation and airport/environmental excise trust funds. |
| 04 | Milestone source review | pending | Verify source anchors and parent totals for extracted milestone years. |
| 05 | Historical funding evolution note | pending | Summarize how the source-backed historical funding picture changes across milestones. |

## Success Criteria

- Milestone choice is documented before extraction.
- Table 2.4 parent totals reconcile to Table 2.1 parent rows.
- Missing markers remain missing, not zero.
- Trust-fund/federal-fund treatment remains visible.
- No taxpayer receipt or reform claim is introduced.

## Non-Goals

- Do not extract the full 1940-2031 table.
- Do not infer legal account rules beyond source labels and reviewed concepts.
- Do not collapse social-insurance receipts into ordinary income tax.
- Do not allocate receipts to outlay programs.

## Validation

Run:

```powershell
git diff --check
```

For JSONL pulses, parse every row as JSON and reconcile extracted parent totals
to Table 2.1.
