# Pulse 05b: Costing the Health-Efficiency Target

## Goal

Put dollar figures on the panel's strongest convergence — health is a
cost-per-outcome problem — and show how it changes the balanced-budget math.

## Changes

- Added `data/derived/program_lane_rate_model/health_efficiency_scenarios.fy2025.draft.jsonl`.
- Added `docs/research/2026-06-23-health-efficiency-target.md`:
  - Federal health lanes $1,975,229M; efficiency gains of 10/20/30% free
    197,523 / 395,046 / 592,569 $M (11/22/33% of the gap).
  - International context: US gov/compulsory health ~14.3% GDP (~$4.34T) vs OECD
    7.1% — federal scenarios are conservative vs the full gap.
  - Two balanced paths: revenue-led (31% cut / 69% collect, tax-to-GDP 25.6→29.6%)
    and panel-center (51% cut / 49% collect, 25.6→~28.5%) — both leave the US
    below the OECD average (34.1%).
  - Revised core rates after the cut side (Health 13.96→12.12%, Medicare
    14.22→12.35%, Defense 13.07→11.75% of the funded dollar).

## Boundaries kept

- No single prescribed number; cut/collect mix shown as a choice.
- GDP derived/approximate, flagged; defense %-GDP inherits that.
- Efficiency = cost-per-outcome/delivery reform, not coverage cuts; not a
  waste/fraud finding.
- `allocation_method = proposed_reform`.

## Validation

- JSONL parses.
- `git diff --check`.

## Status

Done.
