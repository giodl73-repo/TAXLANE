# Yearly Income-Tax Outlay Model Note

## Question

Can TAXLANE estimate, by year, what percentage of ordinary individual income-tax
receipts maps to different outlay categories?

## Short Answer

Yes, as a modeled fiscal-year allocation. The current draft allocates each
year's individual income-tax receipts across six broad OMB Table 3.1 categories
in proportion to reported outlays, while keeping deficit or surplus context
visible.

This does not mean income-tax dollars were legally dedicated to those
categories. Ordinary individual income tax remains a general receipt unless a
separate legal source supports a narrower dedication.

## Draft Model

- Model: `individual-income-tax-proportional-outlays-v1`
- Years: fiscal 1940-2025
- Rows: 516
- Categories: national defense, human resources, physical resources, net
  interest, other functions, and undistributed offsetting receipts
- Sources: OMB Historical Tables 1.1, 2.1, and 3.1, FY2027 release

## Sample Pattern

Allocation shares below use the model allocation denominator for each year.
They are category shares, not tax-law destinations.

| Fiscal year | National defense | Human resources | Physical resources | Net interest | Other functions | Offsetting receipts | Borrowed share of outlays |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1940 | 17.5% | 43.7% | 24.4% | 9.5% | 8.2% | -3.3% | 30.8% |
| 1950 | 32.2% | 33.4% | 8.6% | 11.3% | 18.7% | -4.3% | 7.3% |
| 1960 | 52.2% | 28.4% | 8.7% | 7.5% | 8.4% | -5.2% | 0.0% |
| 1970 | 41.8% | 38.5% | 8.0% | 7.3% | 8.8% | -4.4% | 1.5% |
| 1980 | 22.7% | 53.0% | 11.2% | 8.9% | 7.6% | -3.4% | 12.5% |
| 1990 | 23.9% | 49.4% | 10.1% | 14.7% | 4.8% | -2.9% | 17.6% |
| 2000 | 16.5% | 62.4% | 4.7% | 12.5% | 6.4% | -2.4% | 0.0% |
| 2010 | 20.1% | 69.0% | 2.6% | 5.7% | 5.0% | -2.4% | 37.4% |
| 2020 | 11.1% | 66.6% | 13.0% | 5.3% | 5.6% | -1.6% | 47.5% |
| 2025 | 13.1% | 67.1% | 4.4% | 13.8% | 3.7% | -2.1% | 25.3% |

## What We Are Learning

1. A by-year model is feasible from official annual OMB tables starting in
   1940.
2. The broad mix changes materially across decades: wartime and Cold War years
   are much more defense-heavy, while later years are dominated by human
   resources.
3. Net interest becomes a large visible category in some periods and should not
   be hidden inside program services.
4. Deficit context materially changes the civic story. In years such as 2010,
   2020, and 2025, income-tax receipts are only one part of financing total
   outlays.
5. Undistributed offsetting receipts can be negative and should remain visible
   rather than forced into a positive spending category.

## Public Wording Rule

Use wording like: "If ordinary individual income-tax receipts were allocated by
that year's broad outlay shares, the modeled split would be..."

Avoid wording like: "Income taxes paid for..." unless a separate legal
dedication source supports that claim.
