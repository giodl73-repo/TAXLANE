# Costing the Health-Efficiency Target

## Decision supported

Puts dollar figures on the panel's strongest convergence — that US health is the
one lane where the problem is cost-per-outcome, not under-taxing — and shows how a
health-efficiency gain plus a defense trim changes the balanced-budget math and
the per-lane rates. Closes wave pulse 05b.

Data record:
`data/derived/program_lane_rate_model/health_efficiency_scenarios.fy2025.draft.jsonl`.

## The international context (why this lane)

US government/compulsory health spending is **~14.3% of GDP (~$4.34T)** vs the
OECD average of **7.1% (~$2.15T)** — about 2x, for narrower coverage (benchmark
note). That all-payer gap (~$2.19T of GDP) is the upper bound. The *federal*
health lanes are a subset: **Health $978,511M + Medicare $996,718M = $1,975,229M**.
The scenarios below apply only to the federal lanes, so they are conservative
relative to the full international gap.

## Health-efficiency scenarios (federal Health + Medicare)

A reduction in **cost-per-outcome** — delivery/pricing reform, not coverage cuts:

| Efficiency gain | Frees ($M) | Closes this share of the $1.77T gap |
|---|---:|---:|
| 10% | 197,523 | 11% |
| 20% | 395,046 | 22% |
| 30% | 592,569 | 33% |

Even a 30% federal gain leaves US health well above peer levels — these are not
austerity numbers, they are "stop paying ~2x for the same care."

## Defense convergence (for comparison)

Toward the European norm (GDP-based, approximate):

| Target | Defense ($M) | Saves ($M) |
|---|---:|---:|
| to 2.5% of GDP | 758,778 | 157,362 |
| to 2.0% of GDP | 607,022 | 309,118 |

## Two balanced paths (no borrowing)

Both close the full $1.77T gap; they differ in the cut/collect mix the panel left
as the real decision.

| | Path A — revenue-led | Path B — panel-center |
|---|---|---|
| Health efficiency | 20% (395,046) | 30% (592,569) |
| Defense | to 2.5% (157,362) | to 2.0% (309,118) |
| **Cut side** | 552,408 (**31%**) | 901,687 (**51%**) |
| **Revenue side** | 1,222,276 (**69%**) | 872,997 (**49%**) |
| All-gov tax-to-GDP | 25.6% → **29.6%** | 25.6% → **~28.5%** |

**Either way the US lands below the OECD average (34.1%) and below the UK (34.4%)
and Canada (34.9%).** That is the core honest finding: you can balance the federal
budget, with a real health-efficiency push, and *still* be a relatively low-tax
country by peer standards.

## What it does to the core rates

After the Path-A cut side, the funded total drops from $7.01T to ~$6.46T and the
biggest lanes shrink as a share of each tax dollar:

| Lane | Before | After |
|---|---:|---:|
| Health (non-Medicare) | 13.96% | 12.12% |
| Medicare | 14.22% | 12.35% |
| National Defense | 13.07% | 11.75% |

The freed share redistributes toward debt paydown and the under-funded social
floor, or lowers the required revenue lift.

## Caveats (Source Custodian / Reform Skeptic)

- **No single prescribed number.** Two paths are shown; the cut/collect mix is a
  political choice, not arithmetic.
- **GDP is derived** (outlays = 23.1% of GDP, ~$30.35T) and approximate, not yet a
  repo data record; the defense %-of-GDP figures inherit that approximation.
- Health efficiency is **cost-per-outcome / delivery reform**, never a coverage or
  benefit cut; the repo bars calling current spending "waste/fraud" as a finding.
- `allocation_method = proposed_reform`.

## Adopt now

- Use the federal health-efficiency range ($198-593B) as the anchored cut lever in
  any balanced-budget presentation.
- Show both paths so the cut/collect choice stays explicit.

## Defer

- A quantified delivery-reform mechanism (how the efficiency is achieved).
- A GDP data record (extract OMB Table 1.2) to replace the derived approximation.
