# Modeled Income-Tax Subfunction Outlays

## What This Is

This packet answers a narrower drilldown question than the broad outlay packet:

> If ordinary individual income-tax receipts were allocated by OMB Table 3.2
> subfunction outlay shares, which detailed budget subfunctions would appear
> largest, and how do selected subfunctions change over time?

The answer is a model. It is not a claim that income-tax dollars are legally
dedicated to those subfunctions, and it is not program-level dollar tracing.

## Current-Law Baseline

Ordinary individual income tax is treated here as a general receipt. Congress
appropriates spending through budget law; the model does not change that legal
baseline. It uses OMB subfunction outlays as a visibility denominator, not as
proof that a taxpayer's dollars were assigned to a specific program.

For this packet, the method is:

```text
individual income-tax receipts
  x Table 3.2 subfunction outlay share
```

The model uses fiscal years, not tax years.

## Data Behind The Drilldown

| Artifact | What it contains |
|---|---|
| `income_tax_outlay_subfunction_model.omb-fy2027.2026-06-21.draft.jsonl` | Canonical annual model rows, fiscal years 1962-2025. |
| `income_tax_outlay_subfunction_model.omb-fy2027.2026-06-21.annual-long.csv` | Chart-ready annual long view by fiscal year and subfunction. |
| `income_tax_outlay_subfunction_model.omb-fy2027.2026-06-21.fy2025-top-subfunctions.csv` | Chart-ready FY2025 top-subfunction view. |
| `income_tax_outlay_subfunction_model.omb-fy2027.2026-06-21.decade-long.csv` | Chart-ready decade rollup by subfunction. |

The source spine is OMB Historical Tables 2.1 and 3.2 from the FY2027 release.
The model covers actual fiscal years 1962-2025. It excludes TQ and FY2026-FY2031
estimates.

## FY2025 Drilldown

The FY2025 ranked export shows the largest modeled allocations when individual
income-tax receipts are allocated by Table 3.2 subfunction outlay share:

| Rank | Subfunction | Function | Modeled share |
|---:|---|---|---:|
| 1 | Social security | Social Security | 22.5% |
| 2 | Interest on Treasury debt securities (gross) | Net Interest | 17.3% |
| 3 | Medicare | Medicare | 14.2% |
| 4 | Health care services | Health | 13.2% |
| 5 | Department of Defense-Military | National Defense | 12.4% |

These are modeled shares of ordinary individual income-tax receipts. They are
not statutory earmarks.

## Decade Drilldown

The decade rollup sums modeled allocation dollars within each decade and then
calculates each subfunction's share of that decade total. It is useful for
long-run comparison, but it is not an average of annual percentages or ranks.

| Decade | Coverage | Largest modeled subfunction allocation |
|---|---|---|
| 1960s | partial, FY1962-FY1969 | Department of Defense-Military, 44.2% |
| 1970s | full decade | Department of Defense-Military, 27.2% |
| 1980s | full decade | Department of Defense-Military, 25.5% |
| 1990s | full decade | Social security, 22.0% |
| 2000s | full decade | Social security, 21.6% |
| 2010s | full decade | Social security, 23.2% |
| 2020s | partial, FY2020-FY2025 | Social security, 20.1% |

The 1960s and 2020s rows are partial-decade buckets because the actual-year
model starts in FY1962 and currently ends in FY2025. Do not compare those
buckets as if they contain ten fiscal years.

## Financing Context

The subfunction chart is not a complete picture of federal financing. It splits
ordinary individual income-tax receipts by subfunction outlay share, but total
federal outlays are also financed by other receipts and, in deficit years,
borrowing. That means a subfunction label should not be read as "income taxes
paid this program."

Use subfunction charts beside the broad modeled-outlay packet, especially the
borrowed-share and income-tax-coverage context. If a public display cannot show
that context nearby, treat the subfunction chart as an analysis artifact rather
than a taxpayer receipt.

## Why Subfunctions Are Useful

Subfunctions are useful because broad categories can hide the shape of the
budget. "Human resources" becomes clearer when Social Security, Medicare,
health care services, income security, and veterans benefits are visible
separately. "Net interest" also needs to stay explicit because interest is a
financing cost, not a direct public service.

Subfunctions also raise the risk of overclaiming. A label can sound like a
program receipt even when the model is only proportional allocation. For that
reason, public use should show the broad model first, then use subfunctions as a
drilldown.

## How To Say It

Use:

> In this model, if FY2025 individual income-tax receipts are allocated by OMB
> Table 3.2 subfunction outlay shares, Social Security is about 22.5 percent of
> the modeled allocation and gross interest on Treasury debt securities is about
> 17.3 percent.

For decade views, say:

> In the decade rollup, cumulative modeled allocations shift from national
> defense as the largest subfunction in the 1960s-1980s to Social Security as
> the largest subfunction from the 1990s forward. The 1960s and 2020s are
> partial buckets.

Avoid:

> Income taxes paid for Social Security and Treasury interest in those amounts.

That phrasing implies legal dedication or taxpayer-dollar tracing that this
model does not support.

## Use With The Broad Packet

Read this packet after `modeled-income-tax-outlays.md`. The broad packet gives
the fiscal context: outlay categories, decade change, borrowing, and
income-tax coverage. The subfunction packet is a drilldown view, not a
replacement for the broad financing context.
