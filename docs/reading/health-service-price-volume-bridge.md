# Health Service Price And Volume Bridge

Machine record:
`data/derived/breadth_benchmark_matrix/health_service_price_volume_bridge.cy2024.v1.draft.json`.

## What The 2024 Growth Data Say

The three largest selected service categories account for **$3.211T**, or
**61% of US national health expenditure**. CMS separates their annual spending
growth from reported category price growth; TAXLANE calculates the residual
non-price growth multiplicatively.

| CY2024 category | Spending | Total growth | Price growth | Implied non-price growth |
|---|---:|---:|---:|---:|
| Hospital care | $1.635T | 8.9% | 3.4% | 5.319% |
| Physician and clinical services | $1.110T | 8.1% | 1.8% | 6.189% |
| Retail prescription drugs | $467.0B | 7.9% | 1.4% | 6.410% |

The residual is not pure visit volume. It includes use, service or drug mix,
and intensity, and may contain other non-price effects in the CMS accounts.

## How This Fits The International Result

OECD's 2023 system-wide indexes place both the US health price level and
estimated per-person healthcare volume at **152**, with the OECD average equal
to 100. The CMS rows show that both price and non-price factors also contributed
to recent spending growth. These findings point in the same direction, but they
cannot be multiplied into a savings estimate: system indexes and category
annual growth rates answer different questions.

## Scoring Gate

No category receives a peer savings score yet. Each still needs:

- a same-definition peer price for the service or drug basket;
- utilization and intensity denominators;
- payer and net-price reconciliation, including drug rebates;
- case-mix or risk adjustment; and
- quality, access, coverage, and clinical outcome floors.

```text
growth decomposition != peer efficiency finding != fraud != savings
```

Sources: CMS *National Health Expenditures 2024 Highlights* and OECD *Health at
a Glance 2025*.
