# Peer-informed savings and reinvestment profiles schema

This schema describes
`peer_informed_savings_and_reinvestment_profiles.ty2026.v1.draft.json`.

## Record contract

| Field | Requirement |
|---|---|
| `record_id`, `record_family`, `version`, `status`, `as_of_date` | Required provenance and lifecycle fields. |
| `model` | Required Tax-Calculator version, data, tax year, timing ratio, behavioral assumption, administration ceiling, interest sensitivity, baseline schedule, and baseline target. |
| `peer_direction` | Required directional interpretation of captured COFOG evidence. It must deny direct international tax-rate comparability. |
| `allocation_rule` | Required link to the recurring needs-and-balance cycle. It must deny automatic education funding, automatic peer-gap funding, and preallocation of the adaptive reserve. |
| `profiles` | Exactly four ordered goal-seeking profiles: focused, balanced, ambitious, and transformative. |
| `current_taxlane_reference` | Required zero-savings, $813.727B, +11-point reference. |
| `full_current_law_rate_reference` | Required arithmetic endpoint and warning; it is not an admitted policy result. |
| `decision` | Required readiness and authority booleans. Initiative savings and public release must remain false. |
| `public_warning` | Required plain-language non-claim. |

## Profile contract

Every profile must include:

- named savings initiatives with track ownership and goal amounts;
- an adaptive reinvestment reserve with eligible track ownership and a goal
  amount; no service lane is guaranteed an allocation;
- `gross savings - reinvestment = net savings`;
- `813.727 - net savings = remaining target`;
- zero admitted savings and an initiative status identifying the amounts as
  policy envelopes rather than estimates;
- the selected tenth-point central rate candidate and the immediately lower
  tested candidate;
- a nonnegative selected final target difference and a negative lower-candidate
  difference after the administration and first-order debt sensitivity;
- narrow taxpayer examples calculated from taxable ordinary income; and
- all eight evidence, outcome, interaction, and rerun gates.

The selected rate is conditional on the entire net-savings goal becoming real,
annual, owner-attributed, non-overlapping savings. It does not validate the
initiatives that supply the scenario amount.

## Reproduction

Generate the two rate grids with `run_grid.py`, covering uplifts
`2.7 2.8 4.7 4.8 6.7 6.8 8.8 8.9`, then run
`build_peer_savings_profiles.py` with both grids and the canonical taxpayer
profile record. Tax-Calculator 6.5.1 and `behresp` are pinned by the experiment
requirements.
