# OMB AP13 Budget Concept Extraction Workspace

This directory is reserved for draft budget-concept records extracted from
`SRC-OMB-AP-13-FUNDS-FY2027`.

## Source Artifact

- Metadata: `data/metadata/SRC-OMB-AP-13-FUNDS-FY2027.2026-06-21.metadata.md`
- Raw artifact:
  `data/raw/omb/SRC-OMB-AP-13-FUNDS-FY2027/2026-06-21/ap_13_funds_fy2027.pdf`
- Related schema: `docs/data/receipts-funds-schema.md`

## Planned Output

Use JSONL records named by source and observed date:

```text
budget_concept.SRC-OMB-AP-13-FUNDS-FY2027.2026-06-21.draft.jsonl
```

Each row should include:

- `record_id`
- `record_family`
- `concept_key`
- `source_ids`
- `source_ref`
- `concept_summary`
- `fund_group_implication`
- `legal_dedication_implication`
- `appropriation_implication`
- `public_caveat`
- `status`
- `observed_date`
- `notes`

## Review Gates

1. Confirm PDF checksum.
2. Confirm source anchor against captured text.
3. Keep concept summaries separate from numeric fund-group rows.
4. Keep legal dedication unknown unless a source specifically supports the
   label.
5. Run `git diff --check`.
