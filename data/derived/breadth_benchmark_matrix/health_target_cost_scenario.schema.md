# Health Target-Cost Scenario Schema

Machine record:
`data/derived/breadth_benchmark_matrix/health_target_cost_scenario.v1.draft.json`.

This record hardens the strongest bounded health sensitivity without converting
it into savings, a premium forecast, a provider revenue forecast, a federal
budget effect, or a target for Medicare or Medicaid.

Required fields:

- `record_id`, `record_family`, `lane_id`, `schema_version`, `as_of_date`;
- `contract_path`, `rubric_path`, `comparison_grade`, `source_ids`, and
  `source_custody_status`;
- `category_bases`, `exact_formulas`, and `category_reconciliation`;
- `perimeter` and `comparison_grade`;
- `behavior`, `transition_cost`, `incidence`, and `federal_translation`;
- `federal_target_cost_usd_billions`, `federal_effect_usd_billions`,
  `gross_savings_usd_billions`, `net_savings_usd_billions`, and
  `balanced_rate_percent`;
- `outcome_floor_statuses`, `admissibility_gates`, and `explicit_blockers`;
- `public_warning_phrases`, `target_cost_ready`, `federal_effect_ready`,
  `gross_savings_ready`, `net_savings_ready`, `balanced_rate_ready`;
- `status` and `next_gate`.

The behavior, transition-cost, incidence, and federal-translation fields stay
null until a specific federal policy instrument, service/provider segmentation,
annual phase-in, utilization and volume response, coding and site-of-care
behavior, network and consolidation response, transition/administration
costs, premium and incidence effects, and floor tests are modeled.

A1 through A7 remain false. The public warning phrase must remain visible:

```text
The $149.786 billion result is a mechanical CY2024 private-insurance payer-payment sensitivity. It is not gross savings, net savings, a premium forecast, provider revenue forecast, federal budget effect, or target for Medicare or Medicaid.
```

```text
aggressive sensitivity != fiscal stress
```
