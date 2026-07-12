# National Private-Insurance Health Sensitivity

Machine record:
`data/derived/breadth_benchmark_matrix/health_national_phi_sensitivity.v1.draft.json`.

## National Payer Base

CMS's 2024 National Health Expenditure tables report that private health
insurance paid:

- **$558.9B** for hospital care; and
- **$505.1B** for physician and clinical services.

Together, those payments were **$1.064T**, or **64.7%** of the $1.6446T private
health insurance source-of-funds total.

## Mechanical Sensitivity

| Scenario | Hospital payer-payment change | Physician/clinical change | Combined |
|---|---:|---:|---:|
| Low change | −$61.9B | −$14.5B | **−$76.4B** |
| Central | −$117.1B | −$32.7B | **−$149.8B** |
| High change | −$172.3B | −$50.9B | **−$223.2B** |

This is the first national sensitivity, but it is Grade C and not a savings
estimate. It mechanically applies external Medicare-relative ratios to CMS
insurer-payment totals.

## Why It Is Not Yet A Score

- CMS's private-insurance source excludes separately reported patient
  out-of-pocket payments; it is not total commercial allowed spending.
- The hospital ratio uses 2022 claims and the professional ratio is a 2023
  valuation using earlier claims, while the national payer base is 2024.
- NHE service categories do not exactly equal the repriced-claims categories.
- No service, provider, geography, case-mix, quality, or access segmentation is
  applied.
- Volume, coding, site of care, networks, consolidation, premiums, benefit
  design, wages, and incidence can all respond.

The next bridge must reconcile insurer payments plus member cost sharing to
current commercial allowed claims, then model behavior, transition cost,
incidence, access, quality, and adequacy.

```text
national payer sensitivity != gross savings != net savings != federal savings
```

Sources: CMS NHE Tables 7 and 8 for CY2024; RAND Hospital Price Transparency
Study Round 5.1; Milliman commercial reimbursement benchmark.
