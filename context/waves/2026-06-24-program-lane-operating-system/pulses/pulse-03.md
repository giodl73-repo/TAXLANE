# Pulse 03: Worked Rate-Change Examples

## Goal
Demonstrate the operating model end to end with real lane_rate_change records.

## Changes
- `data/derived/program_lane_rate_model/lane_rate_change.worked-examples.jsonl` (3 examples).
- `docs/reading/rate-change-worked-examples.md`:
  - Defense UP +$146B (Hague 3.5% commitment).
  - Health DOWN -$395B (20% efficiency, coverage protected; closes 22% of the gap).
  - Social Security base adjustment (lift the cap on a $297B shortfall; rate and benefits unchanged).

## Boundaries kept
- Illustrative; each example recomputes the borrowed-share effect; proposed_reform.

## Validation
- JSONL parses; `git diff --check`.

## Status
Done.
