# Pulse 05c: Income-Tax-as-Budget Allocation

## Goal

Realize "spend only what we collect, no more" for the individual income tax:
treat current income-tax collection ($2,656,044M) as a fixed budget envelope and
allocate it across the general-fund lanes it funds.

## Changes

- Added `data/derived/program_lane_rate_model/income_tax_budget_allocation.fy2025.v1.draft.jsonl`
  (16 general-fund lanes, Social Security carved to payroll, percentages sum to
  100% of the income-tax budget).
- Added `docs/research/2026-06-23-income-tax-budget-allocation.md` showing income
  tax covers 48.9% of the general-fund lanes and the $2,774,388M remainder must be
  cut or collected, not borrowed.

## Boundaries kept

- Proportional baseline labeled as neutral starting point, not a recommendation.
- Social Security carved to its dedicated payroll tax.
- `allocation_method = proposed_reform`; not legal dedication.

## Validation

- JSONL parses; percentages sum to 100%.
- `git diff --check`.

## Status

Done.
