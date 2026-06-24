# Pulse 02: FY2025 Receipts-by-Source Extraction

## Goal

Extract the actual FY2025 receipts-by-source split from OMB Table 2.1 so the
all-receipts lane model rests on real revenue numbers (closes Source Custodian
P1 on the revenue base).

## Changes

- Parsed `hist02z1_fy2027.xlsx` (Table 2.1, row 97 = FY2025) and added
  `data/extracted/receipt_source/receipt_source.SRC-OMB-HIST-2-1-FY2027.2026-06-23.fy2025-source-split.jsonl`
  (5 sources + total; reconciles exactly to 5,236,421 $M).
- Added `docs/research/2026-06-23-receipts-by-source-fy2025.md`: the five-source
  split (individual 50.7%, payroll 33.4%, corporate 8.6%, other 5.2%, excise
  2.0%), which taxes fund which lanes, and the ~$1.94T general-fund gap that is
  the balanced-budget target.

## Boundaries kept

- `allocation_status` separates dedicated payroll from general receipts.
- No legal-dedication claim; "Other" sub-split deferred.

## Validation

- JSONL parses; five sources sum exactly to total receipts.
- `git diff --check`.

## Status

Done.
