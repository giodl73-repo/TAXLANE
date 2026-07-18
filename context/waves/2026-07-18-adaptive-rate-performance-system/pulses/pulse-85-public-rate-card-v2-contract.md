# Pulse 85 — Public Rate-Card V2 Contract

## Scope

Create the public-card contract for valid and blocked rate displays.

## Artifacts

- `data/derived/breadth_benchmark_matrix/public_rate_card_v2_contract.v1.draft.json`
- `data/derived/breadth_benchmark_matrix/public_rate_card_v2_contract.schema.md`
- `docs/reading/public-rate-card-v2-contract.md`

## Boundary

This pulse does not publish a public rate card, statutory rate, effective rate,
tax proposal, savings estimate, waste finding, fraud finding, department cut,
technology-savings claim, or balanced-budget claim.

## Acceptance coverage

- Includes current cost, target cost, assigned base, rate, burden,
  distribution, floors, technology status, risk signals, evidence grade, and
  blockers.
- Labels `not_calculated` and `blocked` as first-class outcomes.
- Avoids statutory-rate language unless publication gates pass.
