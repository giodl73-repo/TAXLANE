# Decade Income-Tax Outlay Summary Note

## Question

What does the yearly modeled allocation show when summarized by decade?

## Method

The decade summary is derived from the annual
`individual-income-tax-proportional-outlays-v1` model. For each category and
decade:

```text
decade_category_percent =
  cumulative modeled category allocation
  / cumulative individual income-tax receipts
```

This is not a simple average of annual percentages. It weights years by the
amount of individual income-tax receipts in those years.

The 2020s are partial because the current actual-year model covers fiscal years
2020-2025 only. It excludes FY2026-FY2031 estimates and projections.

## Decade Pattern

| Decade | National defense | Human resources | Physical resources | Net interest | Other functions | Offsetting receipts | Borrowed share |
|---|---:|---:|---:|---:|---:|---:|---:|
| 1940s | 60.9% | 16.5% | 5.4% | 8.0% | 12.4% | -3.2% | 40.5% |
| 1950s | 59.5% | 22.8% | 6.0% | 7.3% | 9.3% | -5.0% | 4.5% |
| 1960s | 46.4% | 31.9% | 8.4% | 6.8% | 11.3% | -4.8% | 4.6% |
| 1970s | 27.8% | 50.6% | 9.9% | 7.5% | 8.4% | -4.2% | 10.9% |
| 1980s | 26.2% | 50.8% | 7.3% | 12.8% | 6.5% | -3.6% | 17.7% |
| 1990s | 18.5% | 59.3% | 5.2% | 14.6% | 5.1% | -2.8% | 10.3% |
| 2000s | 18.9% | 63.8% | 6.0% | 8.6% | 5.4% | -2.6% | 14.5% |
| 2010s | 16.8% | 70.8% | 3.4% | 6.8% | 4.8% | -2.5% | 22.0% |
| 2020s | 12.4% | 69.3% | 6.1% | 9.7% | 4.8% | -2.4% | 31.7% |

## Interpretation

1. The 1940s and 1950s are defense-heavy in this model. More than half of
   modeled individual income-tax receipts map to national defense by outlay
   share in those decades.
2. Human resources becomes the dominant category starting in the 1970s and
   remains dominant through the partial 2020s.
3. Net interest is not incidental. It reaches 12.8 percent in the 1980s and
   14.6 percent in the 1990s in the decade rollup.
4. Deficit context changes the story. The borrowed share is 40.5 percent in the
   1940s, drops sharply in the 1950s and 1960s, then rises again in later
   decades, reaching 31.7 percent in the partial 2020s.
5. Offsetting receipts stay negative. They should be shown as an offset, not
   recast as a positive public-purpose lane.

## Public Wording

Use: "In this decade rollup, if ordinary individual income-tax receipts are
allocated by broad outlay shares, the modeled split is..."

Avoid: "Income taxes went to..." or "Income taxes funded..." unless a separate
legal source supports that destination.
