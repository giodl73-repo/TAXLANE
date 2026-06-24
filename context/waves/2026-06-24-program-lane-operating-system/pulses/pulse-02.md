# Pulse 02: Rate-Adjustment Operating Model

## Goal
Define how a lane rate moves up or down, who moves it, and the explanation every change must publish.

## Changes
- `docs/research/2026-06-24-rate-adjustment-operating-model.md`: the rate lifecycle; eight triggers (shortfall, surplus, cost growth, new commitment, efficiency, beneficiary change, scheduled review, actual-receipts sequester); three decision-authority tiers (automatic/ordinary/supermajority); the `lane_rate_change` explanation schema; and how up/down moves work without hiding the borrowed share.

## Boundaries kept
- Operating model, not statutory text; automatic triggers bind to independent actual data; no change without a published explanation.

## Validation
- `git diff --check`.

## Status
Done.
