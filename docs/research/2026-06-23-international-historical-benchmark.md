# International and Historical Lane Benchmark

## Decision supported

This note supplies the **historical** and **international** components of the
fair-rate methodology (`2026-06-23-program-lane-fair-rate-methodology.md`). It
answers, per lane, "is the United States taxing and spending more or less than a
solvent historical era and than comparable countries for this public purpose?"
so that a proposed lane rate can be called fair rather than merely arithmetic.

## Research question

For each public-purpose lane, what do (a) US solvent-era norms and (b) peer
countries levy or spend, and what does that imply for whether the lane's target
cost and rate should move up, down, or hold?

## Method and scope caveats

Figures were gathered from authoritative primary sources (OECD Revenue
Statistics 2025, OECD Pensions/Health at a Glance 2025, OECD SOCX, SIPRI, NATO,
World Bank/UNESCO, IMF, CBO, OMB). Three scope rules are enforced because mixing
them produces false comparisons:

1. **Government level**: US *federal* receipts/outlays (~17% / ~23% of GDP) are
   not the same scope as *all-government* tax-to-GDP (25.6%). Cross-country tax
   burden uses the all-government basis.
2. **"Compulsory" health**: OECD "government/compulsory" health spending counts
   US compulsory private insurance, so US health looks higher than a
   general-government-only series would show. Labeled throughout.
3. **Year drift**: latest comparable years differ by series (tax 2024, pensions
   2021, health 2024, social 2022). Each figure carries its year.

Values are external comparators, not TAXLANE data records. Confidence is per
source; OECD estimates and CBO projections are flagged.

## Headline finding: the US is a low-tax country with a revenue-side gap

| Measure | United States | OECD average | Peers |
|---|---:|---:|---|
| Total tax-to-GDP, all government, 2024 | **25.6%** | **34.1%** | France 43.5, Sweden 41.4, Germany 38.0, Canada 34.9, UK 34.4, Japan 33.7 |
| US federal receipts, % GDP, FY2025 (50-yr avg) | 17.3% (17.3%) | — | — |
| US federal outlays, % GDP, FY2025 (50-yr avg) | 23.1% (21.2%) | — | — |

The US is roughly 31st of 38 OECD members on tax burden — about **8.5 points of
GDP below the OECD average**. The FY2025 federal gap (outlays 23.1% vs receipts
17.3% of GDP) is therefore **as much a revenue shortfall as a spending excess**:
federal receipts are at their 50-year average while outlays run ~2 points above
theirs. This materially shapes "fair": closing the borrowed share by rate alone
would still leave the US below peer tax levels.

## Per-lane benchmark (government/compulsory spending, % of GDP)

| Lane | US gov't spend % GDP | OECD avg | Peer range | US vs peers | Direction for target cost |
|---|---:|---:|---|---|---|
| Social Security / pensions | 7.3 (2021); OASDI 5.3 (2026) | 8.1 | Canada 5.9 – Italy 16.1 | Slightly below avg | Hold / modest |
| Health + Medicare (govt/compulsory) | 14.3 (2024) | 7.1 | Germany 10.6, France 9.7 | **~2x highest peers** | **Cost-reduction target** |
| National Defense | 3.2–3.4 (2024-25) | NATO Europe ~2.0 | Poland 4.2, France 2.05 | Above guideline, below 5% Hague aim | Policy-driven band |
| Income Security / family | ~2.3 fed (FY2024); family <1.0 (2021) | family 2.35 | — | Below avg on family support | Hold / modest increase |
| Education | 5.42 (2021) | 5.06 | Sweden 7.32 | Near average | Hold |
| Total public social (SOCX) | 19.0 (2022) | ~21 | France 31.4, Germany 27.7 | Below avg | Context only |

### Reading per lane

- **Health is the clearest waste/efficiency case.** US government/compulsory
  health spending (14.3% of GDP) is nearly double the OECD average (7.1%) and
  above every universal-coverage peer, while covering fewer people. This is the
  strongest international evidence for the owner's "push out waste" thesis: the
  problem here is not under-taxing, it is cost per outcome. The Medicare HI rate
  (2.9%) funds only a slice; most health is general-fund, so a legible Health
  lane would expose the gap between what health costs and what any dedicated
  health tax raises.
- **Social Security is roughly peer-normal-to-low.** Public pension spend (7.3%)
  is just under the OECD average; the OASDI rate (12.4% combined) is below peer
  pension rates (Germany 18.6%, Japan 18.3%, Italy 33%), but the US uniquely caps
  the wage base ($184,500 in 2026) where several peers do not. The fair-rate
  lever here is the **cap**, not primarily the rate.
- **Defense is above the European norm and the 2% NATO floor, below the new aim.**
  US ~3.2–3.4% of GDP exceeds NATO-Europe (~2.0%) and the long-standing 2%
  guideline; the 2025 Hague summit set a new 5%-by-2035 aspiration (3.5% core +
  1.5% broader). Historically the US ran ~14% of GDP at the 1953 Korean-War peak.
  The Defense lane rate is policy-driven within a defensible 2–3.5% band.
- **Social and family support are below peers.** US public social spending
  (19.0%) trails the OECD (~21%), and family benefits (<1%) are far below the
  OECD average (2.35%). If solvency is met partly by raising these lanes, the
  international anchor supports room to grow, not cut.

## Statutory-rate benchmarks (for the rate-on-base framing)

Combined employee+employer contribution rates, peer systems:

| Purpose | US | Germany | Japan | Italy | Sweden | Netherlands |
|---|---|---|---|---|---|---|
| Pensions | 12.4% (capped) | 18.6% | 18.3% | 33% | ~17.2% | 17.9% (AOW, employee-side) |
| Health | 2.9% + 0.9% | ~17.1% (GKV) | ~10% | — | — | 6.51% + premium |

Caveats: the UK has no ring-fenced pension rate (National Insurance is the
proxy); France cannot be reduced to one rate (base scheme + AGIRC-ARRCO + CSG);
the Netherlands AOW is levied inside income tax with no employer share. These
rates are comparators for a future dedicated-lane rate, not direct US proposals.

## Historical anchor (US solvent era)

From `data/derived/income_tax_outlay_model/decade-summary.md` plus federal
%-GDP series: solvent post-war decades ran a **~5% borrowed share** with federal
receipts covering outlays far more fully; the 2020s run a ~32% borrowed share
(25.3% in FY2025). The mission mix shifted permanently from defense (59.5% of the
modeled allocation in the 1950s) toward human resources (69.3% in the 2020s), so
the historical anchor is the **solvency ratio and the ~5% borrowed share**, not
the 1950s functional shares.

## Implications for the rate model (feeds Pulse 05)

1. Set each lane's `international_anchor` from the per-lane table; set
   `target_cost_basis` to `international-norm` for Health (downward efficiency
   target) and to `hold-current` for near-average lanes.
2. Record the system-level finding: the borrowed share cannot be closed by lane
   rates alone without acknowledging the US is ~8.5 points of GDP below OECD tax
   levels. The model must show both the revenue gap and the health-efficiency gap
   rather than implying a single rate fixes solvency.
3. Use the contribution-rate table as the comparator column for the
   statutory-rate-on-base framing once the US aggregate base is extracted
   (Pulse 03).

## Confidence and non-goals

- Confidence: high for OECD tax-to-GDP, health, defense (multiple primary
  sources); medium for single-country contribution rates with complex multi-pillar
  systems (France, Netherlands); CBO/OECD estimates flagged inline.
- Non-goal: this note does not set US rates. It bounds them with comparators.
- Non-goal: it does not claim peer systems are transferable wholesale; financing
  structures differ (e.g., compulsory-private vs general-fund).

## Adopt now

- Adopt the per-lane direction column as the international component of each
  lane's recommended rate.
- Adopt the headline framing: revenue gap + health-efficiency gap, both visible.

## Defer

- Final per-lane rates (Pulse 05, after the receipts split and aggregate base).
- Any claim that matching a peer rate is automatically optimal for the US.
