# Program-Lane Target Rates: First Cut (Receipt-Share)

## Decision supported

This is the first concrete per-lane rate output of the fair-rate wave: what share
of a fully funded federal tax dollar each public-purpose lane should carry, with
each lane's level checked against US history and international peers. It is the
**receipt-share** framing. The statutory "percent-of-income" framing follows once
the aggregate income/payroll base is extracted (wave Pulse 03).

Data record: `data/derived/program_lane_rate_model/`. Anchors:
`2026-06-23-program-lane-fair-rate-methodology.md` and
`2026-06-23-international-historical-benchmark.md`.

## The rate table (FY2025, lanes sized to fully funded outlays)

Each share is the lane's cost divided by total federal outlays. Funding all lanes
this way means receipts cover outlays — the borrowed share goes to zero.

| Lane to argue for | FY2025 cost ($M) | **Lane rate (share of tax dollar)** | Fairness check vs peers |
|---|---:|---:|---|
| Social Security | 1,580,673 | **22.55%** | Near peer-normal; lever is the wage cap |
| Medicare | 996,718 | **14.22%** | Part of US health ~2x peer cost — efficiency target |
| Health (non-Medicare) | 978,511 | **13.96%** | Same efficiency target |
| Debt Service (net interest) | 970,065 | **13.84%** | Cost of past borrowing; shrinks as deficit closes |
| National Defense | 916,140 | **13.07%** | Above NATO-Europe ~2%; policy band 2.0-3.5% GDP |
| Income Security & Family | 701,609 | **10.01%** | Below peers (family <1% vs 2.35% GDP) |
| Veterans | 377,163 | **5.38%** | Continuity-protected obligation |
| Transportation | 145,320 | **2.07%** | Dedicated-lane candidate (Highway Trust precedent) |
| Constitutional Govt & Justice | 123,692 | **1.76%** | Broad public good; keep general |
| Nat. Resources/Energy/Environ. | 110,599 | **1.58%** | Mixed fee/royalty/general |
| Community/Disaster/Regional | 82,374 | **1.17%** | Reserve-rule candidate (disaster smoothing) |
| Education, Work & Social Services | 72,042 | **1.03%** | Near average; mostly state/local |
| Agriculture & Food | 47,447 | **0.68%** | Mixed support/insurance |
| International Affairs | 45,171 | **0.64%** | Corrects the common overestimate |
| Science, Space & Civic Capacity | 41,966 | **0.60%** | Long-horizon investment |
| Commerce/Housing Credit | -28,131 | **-0.40%** | Net offset, display as negative |
| Undistributed Offsetting Receipts | -150,254 | **-2.14%** | Accounting offset |

Shares sum to 100.00%. Read as: of every dollar that funds the federal
government, ~23 cents is Social Security, ~28 cents is health (Medicare + Health),
~14 cents is interest on past debt, ~13 cents is defense.

## The honest solvency math

| | FY2025 |
|---|---:|
| Receipts cover this share of outlays | **74.7%** |
| Borrowed (the gap to close) | $1,774,684M — **25.3%** of outlays |
| Aggregate tax lift to fully fund, spending held | **+33.9%** |

Two levers close that gap, and the international evidence says use both:

1. **Revenue (the US is under-taxed vs peers).** US total tax-to-GDP is 25.6% vs
   the OECD average 34.1%. Federal receipts (17.3% of GDP) are at their 50-year
   norm. Lifting federal receipts ~34% to cover outlays would put them near 23%
   of GDP; adding state/local (~8% of GDP) gives a total around **31% of GDP —
   still below the OECD average of 34%**. So closing the gap on the tax side does
   not make the US a high-tax country by peer standards.
2. **Spending efficiency (health is the outlier).** US government/compulsory
   health spending is **14.3% of GDP, ~2x the OECD average (7.1%)**, for narrower
   coverage. The Health + Medicare lanes are ~28% of the tax dollar. This is where
   "get back to regular spending" has real international evidence: the problem is
   cost-per-outcome, not under-taxing.

A defensible reform argues a blend: modestly higher, clearly-named lane revenue
plus a health-efficiency target — not revenue alone, and not the claim that
cutting waste alone closes a 25% gap.

## Which lanes to argue for as dedicated rates

Not every lane should become a narrow earmark (the taxonomy's "some lanes stay
shared" rule). First-cut recommendation:

- **Make explicit / dedicated-candidate**: Social Security (already is),
  Medicare HI (already partial), Transportation (Highway Trust precedent),
  Community/Disaster (reserve-rule case). These have a clean beneficiary or
  existing trust structure.
- **Keep general-fund but make visible**: Defense, Justice/Government,
  International Affairs, Science — broad public goods where a rigid earmark would
  reduce needed flexibility.
- **Efficiency-first, not rate-first**: Health + Medicare — the lane where a higher
  rate without cost reform just funds 2x-peer spending.
- **Show as offsets, never as program lanes**: Commerce/Housing Credit,
  Undistributed Offsetting Receipts.

## Honesty guardrails kept

- `allocation_method` is `proposed_reform` on every row; no claim of current legal
  dedication.
- The borrowed share is shown, not hidden; no balanced-budget assertion.
- Efficiency targets are argued from peer data, not alleged as fraud.
- These are receipt-shares, not percent-of-income rates; that framing is pending
  the aggregate base.

## Next

- Pulse 02: extract FY2025 receipts-by-source split (which taxes fund the lift).
- Pulse 03: extract aggregate AGI/payroll base + GDP -> statutory percent-of-income
  lane rates.
- Pulse 05b: quantify the Health/Medicare efficiency target cost.
- Pulse 06: eight-role review of the proposed rates.
