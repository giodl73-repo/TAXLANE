# Public-Purpose Lane Crosswalk Note

## Decision Supported

TAXLANE should use an explicit crosswalk between OMB budget functions and
reader-facing public-purpose lanes before building a taxpayer receipt prototype.

## Question

How should the first lane crosswalk map OMB Historical Table 3.1 functions and
Table 3.2 subfunctions to TAXLANE public-purpose lanes without implying legal
dedication of ordinary income-tax receipts?

## Sources

- `SRC-OMB-HIST-3-1-FY2027`
- `SRC-OMB-HIST-3-2-FY2027`
- `SRC-OMB-AP-6-CONCEPTS-FY2027`
- `SRC-OMB-AP-9-OFFSETTING-FY2027`
- `docs/research/2026-06-21-public-purpose-lane-taxonomy.md`
- `docs/data/outlays-lanes-schema.md`

## First Crosswalk Rule

The first crosswalk is function-aligned. It maps OMB functions directly when a
single public lane is clear, and combines functions only where the public lane
taxonomy already requires a combined concept:

- Energy plus Natural Resources and Environment become Natural Resources,
  Energy, and Environment.
- Administration of Justice plus General Government become Constitutional
  Government and Justice.
- Net Interest and Undistributed Offsetting Receipts stay separate.

This rule favors source legibility over a smaller public chart. Later receipt
views may roll lanes up for readability, but the rollup should reference this
crosswalk instead of replacing it.

## Guardrails

- The crosswalk does not allocate taxpayer payments.
- Ordinary individual income-tax allocations remain modeled unless a legal
  dedication source is cited.
- Borrowed share / deficit gap is required display context, not a Table 3.2
  outlay function.
- Medicare and Social Security stay separate because OMB reports them as
  separate functions and their financing structures differ from ordinary
  income-tax general receipts.
- Offsetting rows stay visible because public receipts should not hide netting
  mechanics.

## Artifact

The draft artifact is:

`data/derived/lane_crosswalk/lane_crosswalk.omb-fy2027-v1.2026-06-22.draft.jsonl`

It is draft until role review checks lane labels, inclusions, exclusions,
spending-control labels, legal-status labels, and public wording.
