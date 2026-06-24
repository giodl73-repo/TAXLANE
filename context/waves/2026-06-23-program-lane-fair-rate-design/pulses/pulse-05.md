# Pulse 05: Quantified Target-Rate Model (Receipt-Share Framing)

## Goal

Produce the first concrete per-lane rate table — each lane's share of a fully
funded federal tax dollar — with each lane carrying its solvency, historical, and
international anchor.

## Changes

- Added `data/derived/program_lane_rate_model/`:
  - `program_lane_rate_model.fy2025.omb-fy2027-v1.draft.jsonl` — 17 lane rows,
    `model-id program-lane-receipt-share-v1`, `allocation_method proposed_reform`,
    each with cost, receipt share, target-cost basis, international anchor, and
    carried solvency context. Shares sum to 100.0000%; costs reconcile exactly to
    total outlays (7,011,105 $M).
  - `README.md` — method, inputs, solvency context, validation, boundaries.
- Added `docs/research/2026-06-23-program-lane-target-rate-first-cut.md`:
  - The 17-lane rate table (Social Security 22.55%, Medicare 14.22%, Health
    13.96%, Net interest 13.84%, Defense 13.07%, ...).
  - The solvency math: receipts cover 74.7%; +33.9% lift to fully fund; even then
    US lands ~31% of GDP, below the OECD average of 34%.
  - Both gap-closing levers (revenue + health efficiency) with the per-lane
    "argue for dedicated vs keep general vs efficiency-first" recommendation.

## Scope note

This pulse delivers the **receipt-share** framing only. The statutory
percent-of-income framing is deferred to Pulse 03 (aggregate base) and the
quantified Health/Medicare efficiency target to Pulse 05b.

## Boundaries kept

- Proposed-reform labeling on every row; no legal-dedication claim.
- Borrowed share visible; no balanced-budget assertion.
- Health efficiency target argued from OECD data, not alleged as waste/fraud.

## Validation

- JSONL parses; lane costs reconcile to total outlays; shares sum to 100%.
- `git diff --check`.
- Formal `taxlane-tools` validation of the new family deferred to a follow-up.

## Status

Done (receipt-share framing). Statutory framing pending Pulses 02-03.
