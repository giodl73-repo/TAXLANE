# Pulse 08: Full Receipts-to-Lane Allocation ("Every Cent")

## Goal

Answer "should payroll and corporate go to specific things?" by assigning every
FY2025 receipt source to lanes with a labeled method (legal dedication, proposed
dedication, general pool), accounting for all $5,236,421M of receipts.

## Changes

- Parsed OMB Table 2.4 FY2025 for the payroll and excise composition.
- Added `data/derived/program_lane_rate_model/receipts_to_lane_allocation.fy2025.draft.jsonl`
  (17 lanes with dedicated-payroll, dedicated-excise, proposed-corporate, and
  general-pool tiers).
- Added `docs/research/2026-06-23-receipts-to-lane-allocation.md`:
  - Tier 1 legal dedication ($1,821,666M): payroll -> Social Security 1,283,736 /
    Medicare 395,350 / Income Security 69,208; trust excise -> Transportation
    66,994 / environment / health.
  - Exposes the Social Security trust-fund gap (~296,937M) and Medicare's
    general-revenue need (~597,934M).
  - Tier 2 proposed: corporate ($452,089M) -> commerce & capacity cluster
    (justice, transportation, education, agriculture, international, science =
    408,638M) at 111% coverage; volatility matches flexible investment lanes;
    keep-general alternative recorded.
  - Tier 3 general pool (income + other + general excise = 2,962,666M) -> must-fund
    core; the $1,774,684M gap is located on the income-tax-funded core.

## Boundaries kept

- Tier 1 descriptive (current law); Tier 2 `proposed_reform`; Tier 3 general pool.
- SS trust-fund gap and Medicare general need shown.
- Corporate volatility caveat + keep-general alternative both recorded.

## Validation

- JSONL parses; payroll (1,748,294) and excise (73,372) dedications reconcile;
  gap = 1,774,684 ties to total deficit.
- `git diff --check`.

## Status

Done.
