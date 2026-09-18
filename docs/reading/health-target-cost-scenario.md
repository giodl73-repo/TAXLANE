# Health Payment Sensitivity Boundary

Machine record:
`data/derived/breadth_benchmark_matrix/health_target_cost_scenario.v1.draft.json`.

## What The Number Is

For CY2024, CMS reports $1.6446 trillion in private health-insurance payments.
Hospital and physician/clinical payments account for $1.064 trillion, or
64.696583 percent of that payer total.

| Payment path | Hospital | Professional | Mechanical payer-payment change |
|---|---:|---:|---:|
| Current reference | 253% of Medicare | 139% of Medicare | $0.000B |
| Modest sensitivity | 225% | 135% | -$76.390B |
| Central sensitivity | 200% | 130% | -$149.786B |
| Aggressive sensitivity | 175% | 125% | -$223.182B |

For the central sensitivity, the hospital component is -$117.081818B and the
professional component is -$32.704317B. They sum to -$149.786135B.

> The $149.786 billion result is a mechanical CY2024 private-insurance payer-payment sensitivity. It is not gross savings, net savings, a premium forecast, provider revenue forecast, federal budget effect, or target for Medicare or Medicaid.

## Why Readiness Remains False

The Grade C calculation combines a CY2024 insurer-payment base with hospital
and professional Medicare-relative references from different periods and
categories. It has no service/provider segmentation, behavioral response,
transition cost, incidence model, outcome-floor result, federal policy
instrument, or federal score. Missing values remain null and all A1-A7 gates
remain false.

The aggressive sensitivity is not the fiscal solver's stress scenario. Fiscal
stress must later represent an adverse realization of the same selected,
policy-scored central reform.

```text
private-insurance payer-payment sensitivity
  != gross savings
  != net savings
  != premium forecast
  != provider revenue forecast
  != federal budget effect
  != target cost
  != balanced rate
```

The legacy $395.046B flat 20-percent federal arithmetic is illustrative only
and is prohibited from every solver and savings output.
