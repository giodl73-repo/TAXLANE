# Pulse 01: Fair-Rate Methodology + FY2025 Lane-Cost Baseline

## Goal

Open the Tier 3 program-linked rate work with a methodology that makes the rate
question auditable: define "fair rate," set the sourced FY2025 cost baseline each
rate must answer to, and specify the target-rate model schema and data gaps.

## Changes

- Added `docs/research/2026-06-23-program-lane-fair-rate-methodology.md`:
  - FY2025 solvency baseline (outlays 7,011,105; receipts 5,236,421; deficit
    1,774,684; income tax 2,656,044), all sourced to OMB Tables 1.1/2.1/3.x.
  - Per-lane FY2025 cost table reconciled from OMB Table 3.2 functions to the
    16-lane crosswalk; reconciles exactly to total outlays.
  - Three-benchmark "fair rate" definition: solvency, historical, international.
  - `program_lane_rate_model` schema with rate, benchmark, statutory-rule, and
    burden fields; allocation method fixed to `proposed_reform`.
  - Two rate framings: receipt-share (populatable now) and statutory-rate-on-base
    (pending aggregate base).
  - Declared data gaps driving Pulses 02-04 and carried role-review gates.
- Added `context/waves/2026-06-23-program-lane-fair-rate-design/WAVE.md` with the
  six-pulse roadmap and owner-confirmed scope.

## Boundaries kept

- No final rates published; allocation method is `proposed_reform`, never legal
  dedication.
- Deficit/borrowed share stays visible.
- Anti-waste position recorded as an argued advocacy claim, not a fraud finding.

## Validation

- `git diff --check`

(Heavier validation — `cargo ... income-tax-outlay validate`, `cargo test`,
vtrace validate — applies from Pulse 02 when data records change.)

## Status

Done.
