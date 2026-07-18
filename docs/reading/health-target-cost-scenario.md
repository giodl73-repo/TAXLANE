# Health Target-Cost Scenario

Machine record:
`data/derived/breadth_benchmark_matrix/health_target_cost_scenario.v1.draft.json`.

Schema:
`data/derived/breadth_benchmark_matrix/health_target_cost_scenario.schema.md`.

This packet hardens the strongest bounded health result into a readiness-linking
scenario boundary. It does not turn the result into gross savings, net savings,
a premium forecast, a provider revenue forecast, a federal budget effect, or a
target for Medicare or Medicaid.

## Payment Paths

| Path | Hospital target | Professional target | Combined mechanical change |
|---|---:|---:|---:|
| Current reference | 253% | 139% | $0.000B |
| Modest sensitivity | 225% | 135% | −$76.390B |
| Central sensitivity | 200% | 130% | **−$149.786B** |
| Aggressive sensitivity | 175% | 125% | −$223.182B |

The $149.786 billion result is a mechanical CY2024 private-insurance payer-payment sensitivity. It is not gross savings, net savings, a premium forecast, provider revenue forecast, federal budget effect, or target for Medicare or Medicaid.

## What Is Still Missing

- Behavior, transition-cost, and incidence fields stay null.
- Federal translation fields stay null.
- Outcome-floor statuses stay false.
- A1 through A7 admissibility gates stay false.
- The aggressive sensitivity is not fiscal solver stress.

## Evidence Chain

1. `health_cost_decomposition.v1.draft.json`
2. `health_service_price_volume_bridge.cy2024.v1.draft.json`
3. `health_category_benchmark_ladder.v1.draft.json`
4. `health_target_admissibility.v1.draft.json`
5. `health_medicare_relative_scenarios.v1.draft.json`
6. `health_commercial_sample_sensitivity.v1.draft.json`
7. `health_national_phi_sensitivity.v1.draft.json`

The next step is claim-level commercial allowed-spending custody plus the
behavioral, transition-cost, incidence, access, quality, adequacy, and
delivery-feasibility floors required before any federal translation.

```text
aggressive sensitivity != fiscal stress
```
