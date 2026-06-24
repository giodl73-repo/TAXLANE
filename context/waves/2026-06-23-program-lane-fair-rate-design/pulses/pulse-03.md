# Pulse 03: Aggregate Income Base + Illustrative Statutory Rates

## Goal

Add the aggregate income base (IRS SOI) so each lane can be expressed as a
percent of income — the statutory rate-on-base framing the owner asked for.

## Changes

- Added IRS SOI Pub 1304 (TY2022) to the source ledger as `SRC-IRS-SOI-1304`:
  aggregate AGI 14,833,957; taxable income 11,714,186; income tax after credits
  2,098,923 ($M).
- Added `data/derived/program_lane_rate_model/program_lane_statutory_rate.ty2022-illustrative.draft.jsonl`
  (16 lanes; % of income tax, % of AGI, % of taxable income).
- Added `docs/research/2026-06-23-statutory-rate-illustrative.md`:
  - Today's income tax ≈ 14.2% of AGI / 17.9% of taxable income.
  - Per-lane illustrative rate (Medicare 2.60% of AGI, Health 2.55%, Net interest
    2.53%, Defense 2.39%, ...).
  - Decisive finding: funding all non-Social-Security general-fund lanes from the
    income tax alone needs **36.6% of AGI (2.6x today)** — quantified proof the
    balance requires the all-receipts base plus spending discipline.

## Boundaries kept

- Labeled **illustrative**: TY2022 base mapped onto FY2025 lane structure
  (cross-year); not a matched-year statutory rate, not a marginal/individual rate.
- AGI and taxable-income denominators both shown.
- `allocation_method = proposed_reform`.

## Validation

- JSONL parses; AGI shares sum to the current 14.15% effective rate.
- `git diff --check`.

## Status

Done (illustrative). Matched-year statutory rate deferred.
