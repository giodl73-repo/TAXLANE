# Every Cent: Full Receipts-to-Lane Allocation

## Decision supported

Answers "should payroll and corporate go to specific things?" by assigning every
FY2025 receipt source to lanes, with each assignment labeled by method: current
legal dedication, proposed dedication, or general pool. It accounts for all
$5,236,421M of receipts against all $7,011,105M of outlays, leaving the gap
visible.

Data record:
`data/derived/program_lane_rate_model/receipts_to_lane_allocation.fy2025.draft.jsonl`.

## The answer in one line

**Payroll is already tied to specific lanes by law; corporate is the one big
source you get to choose — and it almost exactly funds the public goods that
make commerce possible.**

## Tier 1 — Legal dedication (current law, no reform needed)

Payroll and trust-fund excise are **already** lane-dedicated. This is the proof
the lane model works in practice. (OMB Tables 2.1 and 2.4, FY2025.)

| Source piece | $M | Goes to lane (by law) |
|---|---:|---|
| OASI + DI payroll | 1,283,736 | Social Security |
| HI payroll | 395,350 | Medicare |
| Unemployment + RR + federal retirement | 69,208 | Income Security |
| Highway + airport + waterway excise | 66,994 | Transportation |
| Superfund/oil-spill/aquatic/UST excise | 2,695 | Nat. Resources/Environment |
| Vaccine + PCOR + SMI excise | 3,434 | Health/Medicare |
| Black lung / tobacco assessment | 249 | Income Security / Agriculture |
| **Total already dedicated** | **1,821,666** | |

Two truths this exposes:
- **Social Security runs a ~$296,937M gap** in FY2025 (cost 1,580,673 vs OASDI
  receipts 1,283,736) — covered by trust-fund drawdown. Visible, not hidden.
- **Medicare needs ~$597,934M of general revenue** beyond its HI payroll — most
  of Medicare (Parts B/D) is not payroll-funded.

## Tier 2 — Proposed corporate dedication (the choice)

Corporate income tax ($452,089M) is pure general revenue today, dedicated to
nothing. Recommendation: **dedicate it to the "commerce & capacity" cluster** —
the public goods business relies on but cannot be charged per-use:

| Lane | General-fund need ($M) |
|---|---:|
| Constitutional Govt & Justice (courts, contract law, rule of law) | 123,692 |
| Transportation (general-fund share) | 78,326 |
| Education, Work & workforce | 72,042 |
| Agriculture & Food | 47,441 |
| International Affairs & trade | 45,171 |
| Science, Space & R&D base | 41,966 |
| **Cluster total** | **408,638** |

Corporate ($452,089M) covers this cluster at **111%** — it fully funds it with
~$43B to spare. The match is also *structurally* sound: corporate revenue is
**volatile** (it swings with profits and the business cycle), so it belongs on
**flexible investment lanes** that can absorb year-to-year variation — not on
rigid must-fund entitlements where a recession-year shortfall would cut benefits.

**The honest alternative** (Reform Skeptic's position): keep corporate as general
revenue and don't earmark it, because any dedication can be overridden and
fungibility means the "Corporate → courts" label is presentational. Both are
recorded; the dedication is `proposed_reform`, the volatility logic is its
strongest defense, and it must carry the same shortfall/override guardrails as
any lane (see the guardrail spec).

## Tier 3 — General pool (income tax + other + general excise)

Income tax ($2,656,044) + other receipts ($274,057) + general excise ($32,565) =
**$2,962,666M** funds the remaining must-fund core: National Defense, Health,
Medicare's general-revenue share, Veterans, Income Security's general share, Net
Interest, Community/Disaster, and the Social Security trust-fund top-up question.

## Where the gap lands

After rational source-to-lane assignment, the arithmetic is unchanged but the
**gap is now located**:

| | $M |
|---|---:|
| Total residual general-fund need | 5,189,439 |
| General revenue (pool + corporate) | 3,414,755 |
| **General-fund balanced-budget gap** | **1,774,684** |

That gap — the entire federal deficit — sits squarely on the **income-tax-funded
core lanes** (defense, health, veterans, income security, interest). Payroll and
corporate roughly cover their assigned lanes; the shortfall is in the general
fund. So under "spend what we collect," the decision the panel debated —
collect more vs. bend health cost — applies precisely to this core, not to the
already-dedicated payroll or corporate lanes.

## Summary: should each source go to specific things?

| Source | FY2025 $M | Recommendation |
|---|---:|---|
| Payroll | 1,748,294 | **Yes — already does** (Social Security, Medicare, unemployment). Just make it visible. |
| Corporate | 452,089 | **Yes — propose** dedicating to the commerce & capacity cluster it ~fully funds; volatility fits flexible lanes. |
| Dedicated excise | 73,372 | **Yes — already does** (Highway/Airport Trust, environment). |
| Income tax | 2,656,044 | **No — keep general/flexible**; it must cover the must-fund core where the gap lives. |
| Other + general excise | 306,622 | **No — general pool.** |

## Boundaries kept

- Tier 1 is current law (descriptive). Tier 2 is `proposed_reform`. Tier 3 is a
  general pool, not a per-lane legal claim.
- The Social Security trust-fund gap and Medicare general-revenue need are shown,
  not hidden.
- Corporate volatility caveat and the keep-general alternative are both recorded.

## Adopt now

- Make payroll's existing legal dedication the headline example of legible lanes.
- Carry the corporate → commerce-capacity proposal into role review with its
  volatility rationale and required guardrails.
- Locate the deficit on the income-tax-funded core in all balanced-budget views.
