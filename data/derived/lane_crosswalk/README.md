# Lane Crosswalk Draft

## Purpose

This directory contains the first draft `lane_crosswalk` records for TAXLANE
public-purpose lanes.

The crosswalk maps OMB Historical Table 3.1 functions and Table 3.2
subfunctions to stable TAXLANE lane IDs. It is a mapping layer only. It does
not allocate taxpayer payments, trace legal dedication, or create a statutory
program-linked tax model.

## Current Artifact

| Artifact | Grain | Status |
|---|---|---|
| `lane_crosswalk.omb-fy2027-v1.2026-06-22.draft.jsonl` | one row per TAXLANE lane | Draft |

## Method

- Source functions and subfunctions come from OMB Historical Tables 3.1 and 3.2,
  FY2027 release.
- TAXLANE lane IDs follow `docs/data/outlays-lanes-schema.md`.
- Public labels stay close to the reader-facing lane taxonomy in
  `docs/research/2026-06-21-public-purpose-lane-taxonomy.md`.
- The crosswalk keeps net interest and undistributed offsetting receipts as
  visible lanes.
- Borrowed share / deficit gap remains required display context, but is not an
  OMB outlay function in this table.

## Public-Use Rule

A public receipt prototype may use these records only with:

1. an allocation method label,
2. deficit or borrowed-share context,
3. a caveat that ordinary individual income-tax allocation is modeled and not
   legal tracing,
4. role review before taxpayer-facing publication.

## Validation

Run:

```powershell
git diff --check
```
