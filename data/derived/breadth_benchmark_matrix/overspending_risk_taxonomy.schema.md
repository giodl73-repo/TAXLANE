# Overspending Risk Taxonomy Schema

Machine record:
`overspending_risk_taxonomy.v1.draft.json`

Purpose: define safe overspending-risk signal classes and transitions without
creating waste, fraud, recoverability, savings, budget-score, or department-cut
claims.

Required top-level fields:

| Field | Type | Required | Notes |
|---|---:|---:|---|
| `record_id` | string | yes | Stable id. |
| `record_family` | string | yes | Must be `overspending_risk_taxonomy`. |
| `pulse` | number | yes | Must be `83`. |
| `adaptive_rate_system_contract_path` | string | yes | Pulse 82 contract. |
| `phase_plan_path` | string | yes | Adaptive-rate phase plan. |
| `non_claim_boundary` | string | yes | Must block waste/fraud/savings claims. |
| `signal_families` | array | yes | Initial signal families and disallowed inference. |
| `allowed_classes` | array | yes | Allowed class ids and evidence requirements. |
| `transition_rules` | array | yes | Evidence required to move between classes. |
| `hard_prohibitions` | object | yes | Every shortcut prohibition remains false. |
| `output_placeholders` | object | yes | Public finding/score/cut outputs remain null. |
| `claim_booleans` | object | yes | Every claim boolean remains false. |

Validation requirements:

- Required class ids are present:
  `descriptive_anomaly`, `efficiency_pressure`,
  `operations_review_candidate`, `control_weakness`,
  `recoverability_candidate`, `causal_savings_candidate`,
  `blocked_no_claim`.
- Fraud, savings, recoverability, technology-savings, budget-score, and
  department-cut shortcuts remain prohibited.
- All output placeholders remain `null`.
- Every claim boolean remains `false`.
