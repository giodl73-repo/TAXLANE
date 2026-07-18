# Balanced Rate Readiness Gate Schema

Machine record:
`balanced_rate_readiness_gate.v1.draft.json`

Purpose: freeze the Pulse 80 rule that balanced rates are not calculated until
all target paths reconcile, assigned bases are modeled, and the deterministic
solver can prove a zero unrounded deficit gap.

Required top-level fields:

| Field | Type | Required | Notes |
|---|---:|---:|---|
| `record_id` | string | yes | Stable record id. |
| `record_family` | string | yes | Must be `balanced_rate_readiness_gate`. |
| `pulse` | number | yes | Must be `80`. |
| `contract_path`, `rubric_path`, `coverage_contract_path`, `rate_model_path` | string | yes | Governing artifacts. |
| `source_custody_status` | string | yes | No new external request or contact. |
| `non_claim_boundary` | string | yes | Public no-rate boundary text. |
| `calculation_status` | object | yes | All rate/public claim statuses remain false. |
| `denominator_definitions` | object | yes | Definitions only; values remain null. |
| `sign_conventions` | object | yes | Frozen cash-flow signs. |
| `fy2025_budget_row_ledger` | object | yes | 17 rows; sums to $7,011.105B including offsets. |
| `analytical_lane_boundary` | object | yes | Keeps 15 analytical lanes distinct from 17 budget rows. |
| `assigned_base_requirements` | array | yes | Required base fields; all initial values null. |
| `solver_prerequisites` | object | yes | All currently false on this base. |
| `rate_outputs` | object | yes | Rate/share/card fields remain null or false. |
| `explicit_blockers` | array | yes | Concrete blockers before calculation. |
| `claim_booleans` | object | yes | Every claim boolean must be false. |

Validation requirements:

- The 17-row FY2025 ledger sums to `7011105` million dollars.
- The two negative offset rows remain present.
- The record distinguishes all-receipt funding share from residual
  general-fund requirement share.
- Every assigned-base requirement has `status: "missing"` and `value: null`.
- No statutory rate, effective rate, target cost, savings, public rate card, or
  balanced-budget claim is published.
