# Pulse 116 — Wave 5 fiscal-control overlay depth packets

Date: 2026-07-18

## Scope

Added the fifth scaled-agent depth packet set for the fiscal-control overlays:

- Revenue-solvency.
- Payment integrity.
- Net interest.

## Artifacts

- `data/derived/breadth_benchmark_matrix/wave5_fiscal_control_overlay_depth_packets.v1.draft.json`
- `data/derived/breadth_benchmark_matrix/wave5_fiscal_control_overlay_depth_packets.schema.md`
- `docs/reading/wave5-fiscal-control-overlay-depth-packets.md`

## Boundary

This pulse only publishes overlay-depth scaffolds. It does not calculate rates,
publish receipt-base amounts, run a solver, select target costs, score savings,
identify waste or fraud, issue department-cut instructions, claim technology
savings, or claim a balanced budget.

Revenue-solvency and payment integrity remain non-additive overlays. Net
interest remains endogenous and cannot be cut directly.

## Validation

Added a focused Rust validator requiring:

- Exactly the three overlay packets.
- Revenue-solvency rate publication blocked until all receipt-base,
  behavioral, incidence, distribution, administrative, yield, and interaction
  fields are modeled.
- Payment-integrity savings blocked without causal prevention or same-cohort
  collection lineage.
- Improper-payment estimates not treated as fraud.
- Net interest kept endogenous and not directly cuttable.
- Missing values as `null`.
- Blocked gates as `false`.
- All public claim booleans blocked except packet publication.
