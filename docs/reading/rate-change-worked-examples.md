# Rate Changes in Action — Three Worked Examples

These show the rate-adjustment operating model
(`docs/research/2026-06-24-rate-adjustment-operating-model.md`) running end to end.
Each is a `lane_rate_change` record — the single legible artifact a citizen would
get for "why did this rate move?" All figures are illustrative reform examples, not
current law.

Data: `data/derived/program_lane_rate_model/lane_rate_change.worked-examples.jsonl`.

---

## Example 1 — Defense goes UP (new commitment)

| | |
|---|---|
| **Lane** | National Defense |
| **Old → New** | 13.1¢ (3.0% of GDP) → **15.1¢ (3.5% of GDP)** |
| **Change** | **+$146,149M** |
| **Trigger** | New commitment |
| **Reason** | The 2025 NATO Hague Summit set a 3.5%-of-GDP core-defence target. The lane rises to the commitment — at the top of its 2.0-3.5% policy band. |
| **Decided by** | Ordinary legislation within the band; **supermajority** if it breaches the balance rule |
| **Effect on borrowing** | **Widens the gap by $146B** unless offset by revenue or another lane cut |
| **Citizen takeaway** | "Defense went up because we committed to the Hague 3.5% target — and here is the $146B it costs, which we must raise or offset." |

## Example 2 — Health goes DOWN (efficiency gain)

| | |
|---|---|
| **Lane** | Health + Medicare |
| **Old → New** | 28.2¢ → **22.5¢** of the tax dollar |
| **Change** | **−$395,046M** (20% cost-per-outcome reduction) |
| **Trigger** | Efficiency gain |
| **Reason** | US government health is ~2× the OECD cost share for narrower coverage. A 20% delivery/pricing efficiency gain — **not a coverage cut** — lowers the lane. |
| **Decided by** | Ordinary legislation + program rule changes |
| **Effect on borrowing** | **Closes 22% of the deficit** ($395B) |
| **Citizen takeaway** | "Health costs less because we pay closer to what peer countries pay for the same care — coverage unchanged, $395B saved." |

## Example 3 — Social Security adjusts the BASE, not the rate (shortfall)

| | |
|---|---|
| **Lane** | Social Security |
| **Old → New** | 12.4% on wages capped at $184,500 → **12.4% on a raised/removed cap** |
| **Change** | Close the **$296,937M** FY2025 trust-fund gap |
| **Trigger** | Shortfall (cost > dedicated receipts) |
| **Reason** | OASDI cost ($1.58T) exceeds its receipts ($1.28T) by ~$297B, covered by trust-fund drawdown. Peers tax at higher rates, often with **no cap**. Lift the cap, not the rate. |
| **Decided by** | Ordinary legislation (the cap is a statutory parameter) |
| **Effect on borrowing** | Closes the **Social Security** gap; does **not** touch the general-fund deficit |
| **Citizen takeaway** | "Social Security was short, so higher earners now pay on more of their wages — benefits unchanged, the rate unchanged, the cap lifted." |

---

## What the three show together

- **Up, down, and base-only** moves all run through the same legible record: a
  trigger, a reason citing evidence, a named decision authority, and the effect on
  the borrowed-share line.
- A change can never make the budget *look* balanced when it is not — every example
  recomputes the gap.
- The hard cases (defense up, health down, Social Security shortfall) are exactly
  where today's system hides the cost in borrowing; here each cost is named and
  attributed.

## Honesty caveats on these examples

These show the *record format*, not that the changes are easy or automatic:

- **The triggers do not fire on their own.** "Efficiency gain" is a one-word label
  for the most contested, most-failed maneuver in US health policy. A 20% cost cut
  with coverage intact is a decade-long fight, and the operating model now requires
  a coverage floor plus independent verification before the $395B is booked — it is
  not realized cash until the outcome holds.
- **Social Security is a path, not a one-year hole.** The $297B is the FY2025 gap;
  the trust-fund shortfall grows over time, and lifting the cap also raises future
  benefit accruals. A real fix shows a multi-year solvency horizon, not a single
  year.
- **Defense up is the honest one** — it names the $146B and says it must be raised
  or offset, not borrowed.
