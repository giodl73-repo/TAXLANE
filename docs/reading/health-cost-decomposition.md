# Why US Health Spending Is Higher

Machine record:
`data/derived/breadth_benchmark_matrix/health_cost_decomposition.v1.draft.json`.

## Headline

The United States spent **17.2% of GDP** on health in the OECD's CY2024
estimate, versus a **9.3% OECD average**: an observed gap of **7.9 percentage
points**. On the same source, spending was **$14,885 per person** in purchasing
power terms versus **$5,967**, a ratio of about **2.5 to 1**.

That is a large and credible cost-pressure signal. It is not a savings score.

## What Explains The Difference?

| Factor | Best current signal | What it establishes |
|---|---|---|
| Prices | The standardized US healthcare basket cost 52% more than the OECD average in 2023. | Prices are a material driver; the system index is not a service-level savings rate. |
| Volume and intensity | Estimated US healthcare volume per person was 52% above the OECD average in 2023 after removing the price component. | Quantity, mix, and intensity also matter; this is broader than visit counts. |
| Administration | About 8% of US health spending in 2019 and over $800 per person—more than five times the other-G7 per-person average. | Administrative complexity is a material investigation lane, not an automatically removable amount. |
| Coverage and case mix | Core-service coverage was 93%; obesity was higher, smoking lower, and older peers often had older populations. | Need, coverage, and risk must be controlled; age structure does not explain away the US spending level. |
| Outcomes | Life expectancy was 2.7 years below the OECD average and preventable and treatable mortality were higher, while acute heart-attack and stroke mortality was better. | Higher spending does not buy uniformly better outcomes; outcome floors must remain measure-specific. |

CMS adds an important time-series qualification: US national health expenditure
grew 7.2% in 2024 to $5.3T, and that year's growth was primarily driven by
non-price factors such as demand and service mix. That does not contradict the
cross-country price result. One explains recent US growth; the other helps
explain the level difference between countries.

## Defensible Reading

The evidence supports this statement:

> The United States pays more, uses a more intensive mix of some services, and
> carries unusually high administrative cost, while delivering mixed outcomes.

It does **not** support adding the price, volume, and administration signals
together. They use different years, denominators, and comparison groups, and
administration is already part of total expenditure. It also does not support
applying the 7.9-point GDP gap as a recoverable-savings percentage.

## Next Scoring Gate

The CY2024 service bridge is recorded in
`data/derived/breadth_benchmark_matrix/health_service_price_volume_bridge.cy2024.v1.draft.json`.
It compares hospital, physician/clinical, and retail-drug growth using
`expenditure = price × non-price factors`. Matched category-level peer prices,
case mix, and quality/access floors are still required before scoring. Until
those gates close:

```text
observed spending gap != inefficiency != fraud != recoverable savings
```

Sources: OECD *Health at a Glance 2025* United States country note; OECD (2022),
*Understanding differences in health expenditure between the United States and
OECD countries*; CMS *National Health Expenditures 2024 Highlights*.
