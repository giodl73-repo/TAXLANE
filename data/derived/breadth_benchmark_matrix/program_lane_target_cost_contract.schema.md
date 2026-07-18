# Program-Lane Target-Cost Contract Schema

Machine record:
`program_lane_target_cost_contract.v1.draft.json`.

The record defines the bridge between international comparison evidence and a
balanced Taxlane rate. It covers exactly the 15 comparison lanes.

The 15 records are analytical lanes, not a substitute for the 17-row FY2025
budget reconciliation. Health maps to two rate rows, science/energy/environment
maps to two rows, revenue-solvency and payment integrity are non-additive
overlays, and the commerce/housing-credit and undistributed-offset rows remain
outside the analytical set but inside final balance arithmetic.

Each lane must declare:

- `rate_components`: legacy FY2025 rate-model rows mapped to the comparison lane;
- `target_cost_method`: the method that produces gross and net target cost;
- `comparator_role` and `directionality`: how comparison evidence may be used;
- `policy_levers`: mechanisms that can actually move cost or service;
- `outcome_floors`: conditions that block a cost-down scenario;
- `federal_translation`: treatment of federal, state, local, private, and
  compulsory-financing scope;
- `financing_bases`: candidate receipt bases, not enacted dedication;
- `solver_treatment`: its role in the ten-year integrated model;
- `readiness_status`, `target_cost_ready`, and `balanced_rate_ready`.

`target_cost_ready` and `balanced_rate_ready` remain false until numeric source
records and every common gate are validated. Empty `rate_components` are allowed
only for the revenue-solvency funding system and payment-integrity overlay.

The three required scenarios are `current_law`, `central_reform`, and `stress`.
The solver must keep major trust funds separate, recompute net interest after
each primary-balance change, expose any deficit gap, and reconcile using
unrounded values.

Pulse 72 freezes two separate share quantities:

- `all_receipt_funding_share` = gross program cost / total funded federal cost;
- `residual_general_fund_requirement_share` = residual general-fund need / total
  residual general-fund need.

A value calculated after subtracting dedicated receipts is not a "share of every
tax dollar." Reserve rules and public rounding residual treatment are governed
by `fiscal_accounting_rate_definitions.v1.draft.json`; any public rounding
residual must be shown on an explicit rounding line.

This is a proposed-reform design contract. It is not a legal dedication, a
statutory rate, a savings score, or a claim that peer spending is efficient.
