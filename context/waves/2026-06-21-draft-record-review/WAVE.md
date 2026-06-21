# Wave: Draft Record Review

## Goal

Review the first extracted draft records so TAXLANE can distinguish raw draft
data from source-reviewed and budget-reviewed records before expanding the data
surface or publishing public claims.

## Thesis

Small extracted slices are useful only if their custody, row anchors, labels,
year basis, and reconciliations are checked. Review should promote confidence in
the extraction process without pretending draft records are complete public
models.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|---|---|---|
| 01 | IRS rates source review | done | Verified IRS Table 23 checksum, row anchors, source labels, and extracted 1913-1918 values. |
| 02 | OMB receipts source review | done | Verified Table 2.1 checksum, row anchors, source labels, and 1934-1936 extracted values. |
| 03 | OMB outlays source review | done | Verified Table 3.1 checksum, row anchors, labels, and 1940-1942 extracted values. |
| 04 | First-slice budget review | done | Checked year basis, amount/percent semantics, reconciliation evidence, and public-allocation blockers across first slices. |
| 05 | Public learning note | pending | Summarize what the reviewed slices teach and what remains blocked. |

## Success Criteria

- Review notes cite the exact draft files and raw artifacts.
- Source checks verify checksums and source row anchors.
- Budget checks do not advance public allocation claims.
- Any promotion recommendation is limited to the reviewed slice.
- Validation commands pass.

## Non-Goals

- Do not extract new rows in this wave.
- Do not publish taxpayer-facing allocation receipts.
- Do not infer legal dedication from receipt or outlay rows.
- Do not mark a broader source reviewed because a small slice passed.

## Validation

Run:

```powershell
git diff --check
```

For JSONL source-review pulses, parse draft JSONL and verify reviewed cells
against the captured workbook.
