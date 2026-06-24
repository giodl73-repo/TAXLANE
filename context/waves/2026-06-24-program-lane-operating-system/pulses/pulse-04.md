# Pulse 04: Public Presentation Packet + Chart

## Goal
Assemble the rate cards, operating model, and worked examples into one citizen-facing explainer, leading with the balanced-budget story, plus a chart.

## Changes
- `docs/reading/program-lane-system.md`: the assembled explainer — the $1.77T gap, the two balanced paths (both leaving the US below the OECD average), where each dollar goes and who pays, how rates move, and why it holds.
- `docs/charts/program-lane-rate-model/lane-rate-bar.vl.json` + `lane-shares.csv` + `README.md`: lane cents-per-dollar bar chart colored by funder.
- Indexed both new reading packets and the system explainer in `docs/reading/README.md`.

## Boundaries kept
- Modeled-allocation and proposed_reform labels; borrowed share visible throughout; sourced figures.

## Validation
- CSV + Vega-Lite parse; `git diff --check`.

## Status
Done.
