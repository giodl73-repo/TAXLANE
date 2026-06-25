# The American Health Funding Premium: Why the US Pays Double for Less, and the Case for an Efficiency-First Health Lane

**TAXLANE Research — panel-reviewed (draft, round 0).** An educational reform model.
Figures are FY2025 unless noted and are sourced to `docs/sources/source-version-ledger.md`.
Rate recommendations are reform proposals; the borrowed share stays visible; no
claim of legal dedication or tax advice is made.

## Abstract

The United States funds health more heavily than any peer democracy, yet covers
fewer people. Government and compulsory health spending is about **14.3% of GDP,
roughly twice the OECD average of 7.1%** [`SRC-OECD-HEALTH-2025`]. In the federal
budget, the Health and Medicare lanes together cost **$1,975,229M in FY2025 — 28.2%
of all outlays** [`SRC-OMB-HIST-3-2-FY2027`]. We argue that the evidence does *not*
support raising the health-funding rate; it supports an **efficiency-first health
lane** that bends cost-per-outcome toward peer levels under a binding coverage
floor. Moderate efficiency scenarios (10/20/30%) free **$198B / $395B / $593B**,
closing 11–33% of the federal deficit without reducing coverage. The modernization
case is to fund coverage while making the *price* of care legible and accountable.

## 1. Introduction

Most budget debates ask whether to spend more or less on health. That framing
misses the American anomaly: the US already spends about twice what peers spend,
for worse population coverage. The question this paper answers is therefore not
"more or less," but **"why so much, for so little — and how should the funding be
modernized?"**

Our contribution is threefold: (i) we locate the health lanes precisely in the
FY2025 federal budget and their financing; (ii) we benchmark US health funding
against peers on a scope-matched basis; and (iii) we show that the four fair-rate
factors — solvency, history, international comparison, and reform feasibility —
converge on an efficiency-first rate path, and we specify the lane and the operating
rule that make it accountable.

## 2. What the US funds today

The federal health function splits into two lanes:

| Lane | FY2025 cost ($M) | Share of outlays | Financed by |
|---|---:|---:|---|
| Health (non-Medicare) | 978,511 | 14.0% | general revenue |
| Medicare | 996,718 | 14.2% | HI payroll (2.9%) + premiums + general |
| **Total** | **1,975,229** | **28.2%** | mixed |

Source: `SRC-OMB-HIST-3-2-FY2027` (function totals); financing from
`SRC-OMB-HIST-2-4-FY2027`. Only Medicare Part A is payroll-funded (HI receipts
$395,350M); the majority of Medicare (Parts B/D) and all of the non-Medicare health
lane are **general revenue**. So health is overwhelmingly a general-fund commitment,
not a self-funded insurance lane — a fact obscured in public debate that treats
"Medicare" as fully pre-paid.

## 3. History: how health came to dominate the budget

Health's budget weight is recent. In the modeled allocation of federal outlays by
decade, the broad **human-resources** category rose from **16.5% of the dollar in
the 1940s to 69.3% in the 2020s**, while national defense fell from 60.9% to 12.4%
(decade summary, `data/derived/income_tax_outlay_model/decade-summary.md`). Health
and Medicare are the fastest-growing components of that rise. The historical lesson
is not that the 1940s mix should return — the mission shifted permanently from
war-finance toward human resources — but that the growth has been concentrated in a
lane whose **unit cost**, not just its caseload, runs far above peers.

## 4. Peer benchmark

On the OECD scope-matched measure of government and compulsory health spending:

| Country | Gov/compulsory health, % GDP (2024) |
|---|---:|
| United States | **14.3** |
| Germany | 10.6 |
| France | 9.7 |
| Sweden | 9.7 |
| United Kingdom | 9.1 |
| Japan | 9.0 |
| Netherlands | 8.3 |
| Canada | 7.9 |
| **OECD average** | **7.1** |

Source: `SRC-OECD-HEALTH-2025`. **Scope caveat (binding for this paper):** the OECD
"government/compulsory" measure counts US compulsory *private* insurance, so it is
broader than the federal health lanes alone; US total (public+private) health
spending is ~17.2% of GDP. The comparison is therefore at the *system* level — the
US health system costs roughly double the peer norm — and the federal lanes are the
largest public slice of that system. Every universal-coverage peer delivers broader
population coverage at a materially lower share of GDP.

## 5. All factors → the recommended rate

The fair-rate methodology decomposes any recommendation into solvency, historical,
and international components.

- **Solvency.** Health + Medicare is 28.2% of a federal dollar that is 25.3%
  borrowed. The lanes are a primary driver of the gap, so a *rate increase* to fund
  them at current unit cost would deepen the structural problem.
- **Historical.** The lane grew on unit cost, not only caseload, so the historical
  signal points at price, not at restoring an older share.
- **International.** Decisive: at ~2x the peer cost share for narrower coverage, the
  US is the high-side outlier. The international anchor says **reduce cost-per-outcome
  toward peer levels**, not raise the rate.

The three converge. The recommended direction is **efficiency-first**: hold or
reduce the health-funding rate while bending the unit cost down. Costed scenarios on
the federal health lanes:

| Efficiency gain (cost-per-outcome) | Frees ($M) | Share of the $1.77T gap |
|---|---:|---:|
| 10% | 197,523 | 11% |
| 20% | 395,046 | 22% |
| 30% | 592,569 | 33% |

Source: `health_efficiency_scenarios` record. These are conservative relative to the
2x international gap: even a 30% federal gain leaves US health above peer levels.

## 6. The needs, the problem, and the modernization case

**Why the US struggles here.** The premium is a *price* phenomenon — higher prices
per service, administrative fragmentation across many payers, and weak price
transparency — not primarily higher utilization. Funding the lane at this unit cost
means taxpayers buy less health per dollar than any peer population.

**The modernization case.** Modernizing health *funding* (the scope of this paper,
distinct from delivery reform) means making the lane's **price legible and
accountable**: publish the cost-per-outcome the lane buys; tie any efficiency saving
to a verified, coverage-protected reduction in unit price; and stop financing
open-ended unit-cost growth through borrowing. The aim is to fund coverage in full
while ending the practice of hiding the unit-cost premium in the deficit.

## 7. Reform design: the efficiency-first health lane

The Health and Medicare lanes carry the standard lane object-model fields, with
these health-specific rules (from the operating model and its hardening):

- `allocation_method = proposed_reform`; the lane is general-revenue funded with the
  Medicare-HI payroll piece shown separately.
- **Coverage floor (binding).** A "down on efficiency" change requires a
  coverage/enrollment floor **and** independent verification that outcomes and the
  served population held constant *before* any saving is booked. A cut that cannot be
  distinguished from reduced service does not qualify as efficiency.
- **No rate increase to fund unit-cost growth** without a published cost-per-outcome
  justification.
- Deficit/borrowed-share context required on every health-lane view.

## 8. Discussion and limitations

- **Scope.** The 14.3%-vs-7.1% figure is system-level (includes US compulsory
  private insurance); the federal lanes are the public slice. We do not claim the
  federal lanes alone can be halved to OECD levels.
- **Efficiency is contested.** A 20% cost-per-outcome reduction with coverage intact
  is the most-attempted, most-failed maneuver in US health policy; the scenarios are
  *potential*, gated by the coverage-floor verification, not realized cash.
- **Fungibility.** Labeling a lane "Health" does not by itself control its spending;
  control lives in the appropriation and coverage-floor rules.
- **Delivery reform** (how the price is bent) is out of scope; this paper addresses
  the *funding* question and the accountability rule, not the clinical mechanism.

## 9. Conclusion

The American health funding premium is the clearest fair-rate finding in the federal
budget: roughly double the peer cost share for narrower coverage. The evidence does
not support a higher health-funding rate; it supports an efficiency-first lane that
bends unit cost toward peer levels under a binding coverage floor, freeing
$198B–$593B without cutting coverage. Modernizing health funding means making the
price legible and accountable — funding coverage in full while refusing to borrow
for an open-ended unit-cost premium.

## Sources

`SRC-OMB-HIST-3-2-FY2027`, `SRC-OMB-HIST-2-4-FY2027`, `SRC-OMB-HIST-1-1-FY2027`,
`SRC-OECD-HEALTH-2025`, plus the derived `income_tax_outlay_model` decade summary and
`health_efficiency_scenarios` records. All recorded in
`docs/sources/source-version-ledger.md`.
