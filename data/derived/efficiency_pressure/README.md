# Efficiency Pressure

## Purpose

This draft family records where large outlays create pressure to look for
efficiency gains over time. It is not a waste finding dataset.

## Artifact

| Artifact | Role |
|---|---|
| `efficiency_pressure.fy2025.v1.draft.jsonl` | Draft pressure rows for major FY2025 spend surfaces. |
| `efficiency_pressure.schema.md` | Field contract and public-use boundary. |
| `cost_down_backlog.fy2025.v1.draft.jsonl` | Draft cost-down work items linked to pressure rows. |
| `cost_down_backlog.schema.md` | Backlog field contract and public-use boundary. |
| `cost_down_source_packets.fy2025.v1.draft.jsonl` | Reviewed-source packets attached to selected backlog rows. |
| `cost_down_source_packets.schema.md` | Source-packet field contract and public-use boundary. |
| `cost_down_evidence_queue.fy2025.v1.draft.jsonl` | First-pass extraction queue linked to every source packet. |
| `cost_down_evidence_queue.schema.md` | Evidence-queue field contract and public-use boundary. |
| `cost_down_first_pass_rollup.v1.draft.jsonl` | First-pass status rollup for all cost-down evidence queue rows. |
| `cost_down_first_pass_rollup.schema.md` | First-pass rollup field contract and public-use boundary. |
| `cost_down_scoring_readiness.v1.draft.jsonl` | Readiness-ranked next extracts for moving cost-down rows toward scoreable review. |
| `cost_down_scoring_readiness.schema.md` | Scoring-readiness field contract and public-use boundary. |
| `extracts/payment_integrity_eligibility_first_pass.jsonl` | First PaymentAccuracy.gov portal probe for payment-integrity eligibility work. |
| `extracts/payment_integrity_eligibility_first_pass.schema.md` | Payment-integrity first-pass extract field contract and public-use boundary. |
| `extracts/payment_integrity_scorecards_q4_2025_first_pass.jsonl` | Q4 2025 PaymentAccuracy scorecard probe rows for selected programs. |
| `extracts/payment_integrity_scorecards_q4_2025_first_pass.schema.md` | Payment-integrity scorecard extract field contract and public-use boundary. |
| `extracts/payment_integrity_program_review_gates_q4_2025.jsonl` | Program-review gates for selected PaymentAccuracy scorecard probes. |
| `extracts/payment_integrity_program_review_gates_q4_2025.schema.md` | Program-review gate field contract and public-use boundary. |
| `extracts/payment_integrity_program_review_tasks_q4_2025.jsonl` | Extraction task queue for selected PaymentAccuracy program-review gates. |
| `extracts/payment_integrity_program_review_tasks_q4_2025.schema.md` | Program-review task field contract and public-use boundary. |
| `extracts/payment_integrity_program_review_status_q4_2025.jsonl` | Program-level review status summary for selected PaymentAccuracy task queues. |
| `extracts/payment_integrity_program_review_status_q4_2025.schema.md` | Program-review status field contract and public-use boundary. |
| `extracts/payment_integrity_methodology_plans_q4_2025.jsonl` | Methodology extraction plans for selected PaymentAccuracy program-review rows. |
| `extracts/payment_integrity_methodology_plans_q4_2025.schema.md` | Methodology-plan field contract and public-use boundary. |
| `extracts/payment_integrity_methodology_fields_q4_2025.jsonl` | Field-level methodology checklist for selected PaymentAccuracy program-review rows. |
| `extracts/payment_integrity_methodology_fields_q4_2025.schema.md` | Methodology-field checklist contract and public-use boundary. |
| `extracts/payment_integrity_methodology_source_targets_q4_2025.jsonl` | Source-discovery queue for selected PaymentAccuracy methodology plans. |
| `extracts/payment_integrity_methodology_source_targets_q4_2025.schema.md` | Methodology source-target contract and public-use boundary. |
| `extracts/payment_integrity_methodology_queries_q4_2025.jsonl` | Query plan for selected PaymentAccuracy methodology source targets. |
| `extracts/payment_integrity_methodology_queries_q4_2025.schema.md` | Methodology-query field contract and public-use boundary. |
| `extracts/payment_integrity_methodology_query_runs_q4_2025.jsonl` | Pending query-run scaffold for selected PaymentAccuracy methodology queries. |
| `extracts/payment_integrity_methodology_query_runs_q4_2025.schema.md` | Methodology query-run field contract and public-use boundary. |
| `extracts/payment_integrity_methodology_results_q4_2025.jsonl` | Captured methodology-source results requiring review before field closure. |
| `extracts/payment_integrity_methodology_results_q4_2025.schema.md` | Methodology-result field contract and public-use boundary. |
| `extracts/payment_integrity_methodology_result_review_readiness_q4_2025.jsonl` | Readiness marker for captured methodology results entering field review. |
| `extracts/payment_integrity_methodology_result_review_readiness_q4_2025.schema.md` | Methodology result-review-readiness contract and public-use boundary. |
| `extracts/payment_integrity_methodology_field_reviews_q4_2025.jsonl` | Field-level review of captured methodology-source results. |
| `extracts/payment_integrity_methodology_field_reviews_q4_2025.schema.md` | Methodology field-review contract and public-use boundary. |
| `extracts/payment_integrity_methodology_gap_followups_q4_2025.jsonl` | Source-work follow-up queue for reviewed methodology gaps. |
| `extracts/payment_integrity_methodology_gap_followups_q4_2025.schema.md` | Methodology gap-followup contract and public-use boundary. |
| `extracts/payment_integrity_methodology_gap_source_captures_q4_2025.jsonl` | Official-source captures from methodology gap follow-ups. |
| `extracts/payment_integrity_methodology_gap_source_captures_q4_2025.schema.md` | Methodology gap source-capture contract and public-use boundary. |
| `extracts/payment_integrity_methodology_source_capture_rollup_q4_2025.jsonl` | Reviewer rollup for methodology source captures. |
| `extracts/payment_integrity_methodology_source_capture_rollup_q4_2025.schema.md` | Methodology source-capture rollup contract and public-use boundary. |
| `extracts/payment_integrity_methodology_closure_readiness_q4_2025.jsonl` | Closure-readiness triage over methodology source-capture rollups. |
| `extracts/payment_integrity_methodology_closure_readiness_q4_2025.schema.md` | Methodology closure-readiness contract and public-use boundary. |
| `extracts/payment_integrity_methodology_closure_decisions_q4_2025.jsonl` | Internal closure decisions for methodology fields. |
| `extracts/payment_integrity_methodology_closure_decisions_q4_2025.schema.md` | Methodology closure-decision contract and public-use boundary. |
| `extracts/payment_integrity_methodology_residual_source_gaps_q4_2025.jsonl` | Narrow residual source gaps after closure-readiness review. |
| `extracts/payment_integrity_methodology_residual_source_gaps_q4_2025.schema.md` | Methodology residual source-gap contract and public-use boundary. |
| `extracts/payment_integrity_methodology_closure_coverage_q4_2025.jsonl` | Closure coverage report for Part D and Medicaid methodology fields. |
| `extracts/payment_integrity_methodology_closure_coverage_q4_2025.schema.md` | Methodology closure-coverage contract and public-use boundary. |
| `extracts/payment_integrity_methodology_scoring_gate_q4_2025.jsonl` | Scoring gate over Part D and Medicaid methodology closure coverage. |
| `extracts/payment_integrity_methodology_scoring_gate_q4_2025.schema.md` | Methodology scoring-gate contract and public-use boundary. |
| `extracts/payment_integrity_methodology_program_rollup_q4_2025.jsonl` | Cross-program status rollup over blocked payment-integrity methodology gates. |
| `extracts/payment_integrity_methodology_program_rollup_q4_2025.schema.md` | Methodology program-rollup contract and public-use boundary. |
| `extracts/payment_integrity_next_program_selection_q4_2025.jsonl` | Payment-integrity branch selections for Medicaid/PERM and VA PLTSS methodology planning. |
| `extracts/payment_integrity_next_program_selection_q4_2025.schema.md` | Next-program selection contract and public-use boundary. |
| `extracts/payment_integrity_claims_timeliness_first_pass.jsonl` | First SSA/VA claims-timeliness probe rows. |
| `extracts/payment_integrity_claims_timeliness_first_pass.schema.md` | Claims-timeliness first-pass extract field contract and public-use boundary. |
| `extracts/debt_maturity_risk_first_pass.jsonl` | First Treasury debt-stock and average-rate probe rows. |
| `extracts/debt_maturity_risk_first_pass.schema.md` | Debt maturity-risk first-pass extract field contract and public-use boundary. |
| `extracts/debt_primary_balance_first_pass.jsonl` | First FY2025 fiscal-balance and primary-deficit proxy row. |
| `extracts/debt_primary_balance_first_pass.schema.md` | Debt primary-balance first-pass extract field contract and public-use boundary. |
| `extracts/disaster_supplemental_tracking_first_pass.jsonl` | First FEMA declaration-area probe rows for disaster supplemental tracking. |
| `extracts/disaster_supplemental_tracking_first_pass.schema.md` | Disaster supplemental-tracking first-pass extract field contract and public-use boundary. |
| `extracts/disaster_mitigation_first_pass.jsonl` | First FEMA HMA project probe rows for disaster mitigation. |
| `extracts/disaster_mitigation_first_pass.schema.md` | Disaster mitigation first-pass extract field contract and public-use boundary. |
| `extracts/defense_audit_control_first_pass.jsonl` | First DoD OIG audit-control probe rows for defense audit-control closure. |
| `extracts/defense_audit_control_first_pass.schema.md` | Defense audit-control first-pass extract field contract and public-use boundary. |
| `extracts/defense_procurement_control_first_pass.jsonl` | First GAO weapon-systems procurement-control probe rows. |
| `extracts/defense_procurement_control_first_pass.schema.md` | Defense procurement-control first-pass extract field contract and public-use boundary. |
| `extracts/health_price_discipline_first_pass.jsonl` | First health/Medicare price-discipline benchmark and anchor probe rows. |
| `extracts/health_price_discipline_first_pass.schema.md` | Health price-discipline first-pass extract field contract and public-use boundary. |
| `extracts/health_admin_simplification_first_pass.jsonl` | First health/Medicare administrative-simplification workflow probe rows. |
| `extracts/health_admin_simplification_first_pass.schema.md` | Health administrative-simplification first-pass extract field contract and public-use boundary. |

## Boundary

Rows may support public questions such as "what evidence would show this money is
working?" They must not be described as fraud, waste, abuse, or poor performance
findings unless a reviewed source is later attached.

## Validation

```powershell
cargo run -p taxlane-tools -- income-tax-outlay validate
```
