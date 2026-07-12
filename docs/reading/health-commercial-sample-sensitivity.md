# Health Commercial Sample Sensitivity

Machine record:
`data/derived/breadth_benchmark_matrix/health_commercial_sample_sensitivity.v1.draft.json`.

## First Matched Dollar Sensitivity

Milliman reports analytical data volumes and Medicare-relative reimbursement
for inpatient, outpatient, and professional claims. Repricing those reported
volumes supplies the first dollar-denominated sensitivity on a matched basis.
It does **not** supply a national savings estimate.

| Sample category | Commercial analytical volume | Simulated Medicare base | Implied current ratio |
|---|---:|---:|---:|
| Hospital inpatient + outpatient | $182.229B | $77.347B | 235.598% |
| Professional services | $108.312B | $77.922B | 139% |

## Mechanical Sample Effects

| Category | Low change | Central | High change |
|---|---:|---:|---:|
| Hospital sample | −$8.198B at 225% | −$27.534B at 200% | −$46.871B at 175% |
| Professional sample | −$3.117B at 135% | −$7.013B at 130% | −$10.909B at 125% |

These are arithmetic changes to the cited analytical volumes if nothing else
changes. They are not estimates of national provider payments, premiums,
employer costs, household costs, or the federal budget.

## Basis Sensitivity Matters

The Milliman inpatient/outpatient mix implies a 235.6%-of-Medicare hospital
baseline. RAND's separate national hospital study reports 253%. The difference
is not an error to average away: samples, methods, component weights, and scopes
differ. A target applied to the RAND headline produces a different percentage
change than the same target applied to the matched Milliman volume.

Before national scoring, TAXLANE still needs nationally weighted current
commercial allowed spending by service and provider class, followed by volume,
coding, site-of-care, network, consolidation, transition-cost, incidence,
access, and quality models.

```text
sample sensitivity != national gross savings != net savings
```

Source: Milliman, *Commercial reimbursement benchmarking*. RAND and MedPAC are
used for external reference and adequacy boundaries, not to rescale this sample.
