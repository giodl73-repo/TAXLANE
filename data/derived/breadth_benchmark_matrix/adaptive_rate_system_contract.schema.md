# Adaptive Rate System Contract Schema

Machine record:
`adaptive_rate_system_contract.v1.draft.json`

Purpose: define the annual adaptive-rate lifecycle, rate calculation gate, rate
publication gate, assigned-base requirements, and no-claim boundaries.

Required top-level fields:

| Field | Type | Required | Notes |
|---|---:|---:|---|
| `record_id` | string | yes | Stable id. |
| `record_family` | string | yes | Must be `adaptive_rate_system_contract`. |
| `pulse` | number | yes | Must be `82`. |
| `contract_path`, `rubric_path`, `coverage_contract_path`, `rate_model_path` | string | yes | Governing artifacts. |
| `balanced_rate_readiness_gate_path` | string | yes | Pulse 80 no-rate gate. |
| `final_closure_readiness_gate_path` | string | yes | Pulse 81 no-closure gate. |
| `phase_plan_path` | string | yes | Adaptive-rate phase plan. |
| `non_claim_boundary` | string | yes | Must separate contract from public claims. |
| `annual_update_lifecycle` | array | yes | Ordered lifecycle steps. |
| `rate_gate_sequence` | object | yes | Calculation, publication, and balanced-budget gates. |
| `denominator_definitions` | object | yes | Both denominator formulas; values remain null. |
| `assigned_base_required_fields` | array | yes | All assigned-base requirements missing/null initially. |
| `output_placeholders` | object | yes | Rate outputs remain null. |
| `guardrail_booleans` | object | yes | Every public/rate/claim boolean remains false. |
| `explicit_blockers` | array | yes | Concrete blockers before calculation/publication. |

Validation requirements:

- Rate calculation and rate publication are separate gates.
- The all-receipt and residual-general-fund denominator definitions remain
  distinct.
- Every assigned-base field has `status: "missing"` and `value: null`.
- All rate outputs remain `null`.
- Every rate, savings, waste, fraud, technology-savings, public-card, and
  balanced-budget claim boolean remains `false`.
