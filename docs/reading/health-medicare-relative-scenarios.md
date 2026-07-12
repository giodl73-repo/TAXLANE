# Medicare-Relative Health Price Scenarios

Machine record:
`data/derived/breadth_benchmark_matrix/health_medicare_relative_scenarios.v1.draft.json`.

## Scenario Paths

These are transparent policy assumptions for testing—not selected expected
prices and not savings estimates.

| Category | Current reference | Low change | Central | High change |
|---|---:|---:|---:|---:|
| Hospital care | 253% of Medicare | 225% (−11.067%) | 200% (−20.949%) | 175% (−30.830%) |
| Professional services | 139% of Medicare | 135% (−2.878%) | 130% (−6.475%) | 125% (−10.072%) |

The percentage in parentheses is the mechanical rate change from the current
reference, calculated as `target / current − 1`. It is not a spending reduction:
providers and payers can respond through volume, coding, service mix, site of
care, networks, consolidation, premiums, wages, and benefit design.

## Why There Is No Dollar Number Yet

The $1.635T hospital and $1.110T physician/clinical NHE totals cover all payers
and accounting components. Applying commercial-to-Medicare rate changes to
those totals would be a scope error. Dollar modeling requires commercial
allowed spending for matched services, repriced at the claim or service-line
grain.

## Gates Before Fiscal Scoring

- Segment hospitals by inpatient/outpatient service, geography, teaching,
  uncompensated care, rurality, volume, case mix, and quality.
- Segment professional services by code, specialty, site, geography, primary
  care and safety-net role, participation, access, and quality.
- Model volume, coding, site-of-care, network, and consolidation responses.
- Add transition, administration, monitoring, and enforcement costs.
- Separate provider payment, premium, employer, household, and federal effects.
- Stop or revise a scenario when an access, quality, solvency, or participation
  floor fails.

```text
illustrative rate path != spending reduction != federal savings
```

Sources: RAND Hospital Price Transparency Study Round 5.1; Milliman commercial
reimbursement benchmark; MedPAC *March 2026 Report to the Congress*.
