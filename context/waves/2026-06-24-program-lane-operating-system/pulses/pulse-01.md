# Pulse 01: Per-Lane Rate-Rationale Cards

## Goal
Give each lane a public argument for its rate — the "why each rate makes sense" presentation.

## Changes
- `data/derived/program_lane_rate_model/lane_rate_cards.v1.draft.jsonl` (15 lanes; rate, funded_by, four-anchor argument, move-up/move-down triggers).
- `docs/reading/program-lane-rate-cards.md`: public-facing cards for the big lanes (Social Security, Medicare, Health, Net Interest, Defense, Income Security, Veterans) plus a table for the rest.

## Boundaries kept
- Modeled-allocation and proposed_reform labels; borrowed share visible; payroll/trust-excise noted as the only current legal dedications.

## Validation
- JSONL parses; `git diff --check`.

## Status
Done.
