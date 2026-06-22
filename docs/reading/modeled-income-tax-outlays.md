# Modeled Income-Tax Outlays

## What This Is

This packet answers a narrow question:

> If ordinary individual income-tax receipts were allocated by broad federal
> outlay shares, what would the split look like by year and by decade?

The answer is a model. It is not a claim that income-tax dollars are legally
dedicated to these categories, and it is not a program-level tracing of dollars.

## Current-Law Baseline

Ordinary individual income tax is treated here as a general receipt. That means
the tax source is separate from payroll taxes, corporate income taxes, excise
taxes, and other receipts. It also means a public "what it paid for" view needs
an explicit allocation method before it can be shown responsibly.

For this packet, the method is:

```text
individual income-tax receipts
  x broad outlay category share
```

The model uses fiscal years, not tax years.

## Data Behind the Model

| Artifact | What it contains |
|---|---|
| `income_tax_outlay_model.omb-fy2027.2026-06-21.draft.jsonl` | Canonical annual model rows, fiscal years 1940-2025. |
| `income_tax_outlay_model.omb-fy2027.2026-06-21.decade-summary.jsonl` | Canonical decade summary rows. |
| `income_tax_outlay_model.omb-fy2027.2026-06-21.annual-wide.csv` | Chart-ready annual view. |
| `income_tax_outlay_model.omb-fy2027.2026-06-21.decade-wide.csv` | Chart-ready decade view. |

The source spine is OMB Historical Tables 1.1, 2.1, and 3.1 from the FY2027
release. The annual model covers actual years through fiscal 2025. It excludes
FY2026-FY2031 estimates and projections.

## Broad Categories

The first model uses six broad Table 3.1 categories:

| Category | Why it stays visible |
|---|---|
| National defense | Major historical swing category, especially in wartime and Cold War years. |
| Human resources | Includes major social and benefit functions; dominant in later decades. |
| Physical resources | Infrastructure, natural resources, commerce, housing, and related functions. |
| Net interest | Financing cost, not a program service; should not be hidden. |
| Other functions | International affairs, science, agriculture, justice, general government, and allowances. |
| Undistributed offsetting receipts | A negative offset in OMB presentation; not a positive spending lane. |

## What The Decades Show

These percentages are decade rollups. They are cumulative modeled category
allocations divided by cumulative individual income-tax receipts in each decade,
not simple averages of annual percentages.

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

The 2020s row is partial because the actual-year model currently covers fiscal
2020-2025.

## What We Learn

1. The modeled allocation is defense-heavy in the 1940s, 1950s, and 1960s.
2. Human resources becomes the largest modeled category starting in the 1970s.
3. Net interest is large enough in some decades to require its own visible
   category.
4. Borrowing changes the civic picture. A tax allocation view that omits deficit
   context makes later decades look more tax-financed than total outlays were.
5. Offsetting receipts should stay negative because OMB treats them as offsets
   to spending totals.

## How To Say It

Use:

> In this model, if individual income-tax receipts are allocated by broad outlay
> shares, the 2010s split is about 16.8 percent national defense and 70.8
> percent human resources, with borrowing equal to about 22.0 percent of total
> outlays.

Avoid:

> Income taxes paid 70.8 percent to human resources in the 2010s.

That phrasing implies legal or program tracing that this model does not support.

## Next Useful View

The chart-ready exports support:

- a stacked area chart of annual modeled category shares,
- a stacked bar chart of decade shares,
- a separate borrowed-share line, and
- a separate income-tax coverage line.

The borrowed-share line should sit beside the allocation chart, not inside it,
because borrowing is financing context rather than an income-tax category.
