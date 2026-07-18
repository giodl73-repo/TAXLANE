# Education Workforce Scenario Gate Schema

Machine record:
`education_workforce_scenario_gate.v1.draft.json`

Purpose: freeze the FY2025 federal education/workforce current-law context and
block target-cost, savings, federal-effect, and balanced-rate use until a
matched federal/state/local translation, policy package, behavior model,
distribution model, floor thresholds, and policy-specific score exist.

Required top-level fields:

| Field | Type | Required | Notes |
|---|---:|---:|---|
| `record_id` | string | yes | Stable record id. |
| `record_family` | string | yes | Must be `education_workforce_scenario_gate`. |
| `pulse` | number | yes | Must be `78`. |
| `lane_id` | string | yes | Must be `education-training-employment-social-services`. |
| `contract_path`, `rubric_path`, `coverage_contract_path`, `rate_model_path` | string | yes | Governing artifacts. |
| `source_custody_status` | string | yes | No new external request or contact. |
| `source_custody` | array | yes | Existing repo-custodied evidence paths. |
| `non_claim_boundary` | string | yes | Public boundary text. |
| `current_law_context` | object | yes | FY2025 current-law federal values. |
| `category_bases` | array | yes | Bases, formulas, periods, and reconciliation status. |
| `category_reconciliation` | object | yes | Must reconcile to $72.042B. |
| `perimeter` | object | yes | Must keep federal and whole-system perimeters separate. |
| `required_model_inputs` | array | yes | Missing inputs remain `null`. |
| `behavior_transition_incidence` | object | yes | All initial policy fields remain `null`. |
| `federal_translation` | object | yes | Federal cash-flow/target-cost fields remain `null`. |
| `outcome_floor_statuses` | object | yes | All initial floor statuses remain `false`. |
| `admissibility_gates` | object | yes | A1 may be true from existing custody; A2-A7 remain false. |
| `scenarios` | array | yes | Exactly `current_law`, `central_reform`, and `stress`. |
| `explicit_blockers` | array | yes | Must explain solver/rate blockers. |
| `claim_booleans` | object | yes | Every claim boolean must be `false`. |
| `readiness` | object | yes | Current-law context true; target/federal/solver/rate false. |

Validation requirements:

- Current-law gross program cost is exactly `72042` million dollars.
- Category reconciliation equals current-law gross program cost.
- Missing model inputs use `status: "missing"` and `value: null`.
- Perimeter mismatch keeps federal-effect and target-cost fields `null`.
- Central and stress scenarios are solver-ineligible.
- Stress is not a standalone aggressive cut; it remains blocked until it can be
  tied to the selected central policy.
- Every public warning phrase remains present in the reader.
