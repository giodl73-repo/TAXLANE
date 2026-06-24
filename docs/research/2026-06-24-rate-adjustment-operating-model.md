# Rate-Adjustment Operating Model

## Decision supported

Makes the lane system functional: defines **how a lane rate moves up or down**,
**who moves it**, and the **explanation every change must publish**. This is the
operating layer above the guardrail spec
(`2026-06-23-balance-rule-guardrail-spec.md`) — the spec says what rules a balance
must obey; this says how rates change day to day and how citizens are told why.

## Core principle

> A lane rate never changes silently. Every change runs through a defined trigger,
> a named decision authority, and a published explanation that shows the old rate,
> the new rate, the reason, and the effect on the borrowed-share line.

That is the whole point of a legible system: not that rates never move — they must,
as needs change — but that every move is visible and argued.

## The rate lifecycle

```text
proposed -> enacted -> in-effect -> under-review -> adjusted -> in-effect ...
                                         |
                                         +-> (sunset / merged / retired)
```

A lane sits `in-effect` until a trigger opens an `under-review` state. Review ends
in either `adjusted` (rate moves, with a published explanation) or `affirmed` (rate
held, with a published reason).

## What moves a rate (triggers)

| Trigger | Direction | Mechanism |
|---|---|---|
| **Shortfall** — receipts below target outlays | up (collect) or benefit cut | `shortfall_rule` forces a *visible* choice: raise the rate or reduce the benefit. No silent backfill. |
| **Surplus** — receipts above target | down or debt paydown | `surplus_rule` locks the surplus in-lane or to debt; a rate cut needs an explicit vote. |
| **Cost growth** — outlays drift up (aging, inflation) | up or efficiency | Raise the rate, or bend cost-per-outcome (the health lever). |
| **New commitment** — a policy adds outlays (e.g., NATO Hague 5%) | up | Rate rises to fund the commitment; the commitment is the published reason. |
| **Efficiency gain** — cost-per-outcome falls | down | The freed share lowers the rate or pays down debt. |
| **Beneficiary/eligibility change** | up or down | Caseload changes the target outlay. |
| **Scheduled review** — each lane reviewed every N years | either | Forces periodic re-justification even with no shock. |
| **Actual-receipts sequester** — automatic | up/cut | If actual receipts undershoot the locked estimate, a pre-defined adjustment fires with no new vote (supermajority to suspend). |

## Who moves it (decision authority)

Three tiers, matched to the lane's `override_rule`:

1. **Automatic** — formula and sequester adjustments fire on *actual* data from the
   independent estimator. No vote to enforce; supermajority to suspend. (Closes the
   optimistic-baseline and entitlement-autopilot evasions.)
2. **Ordinary legislative** — discretionary rate changes within a lane's existing
   authority pass by the normal threshold, but must publish the explanation record.
3. **Supermajority** — changing a *dedication* (re-pointing a lane's tax, sweeping a
   surplus, or breaching the balance rule via emergency) requires the higher
   threshold and an automatic future-year payback.

The independent estimator owns the locked revenue and cost numbers the triggers
bind to — so the authority that changes a rate cannot also pick the numbers that
justify it.

## The explanation every change must publish

A rate change is not in effect until its explanation record is published. Proposed
record family: `lane_rate_change`.

| Field | Meaning |
|---|---|
| `lane_id` / `public_label` | Which lane. |
| `old_rate` / `new_rate` | Before and after (receipt-share and/or % of base). |
| `direction` / `magnitude` | Up or down, and by how much. |
| `trigger` | Which trigger fired (from the table above). |
| `reason` | The argument, citing the four anchors (cost, solvency, history, peers). |
| `evidence_source_ids` | Ledger sources backing the reason. |
| `decision_authority` | Automatic / ordinary / supermajority, and the vote if any. |
| `effect_on_borrowed_share` | How the change moves the deficit_gap line. |
| `beneficiary_impact` | Who is affected and how (continuity, eligibility). |
| `effective_date` | When it takes effect. |
| `review_due` | When the lane is next scheduled for review. |
| `allocation_method` | `proposed_reform` until enacted. |

This record is the citizen-facing "why did my Defense lane go up?" answer — a single
legible artifact per change.

## How "up" and "down" actually work

- **Raising a rate** is justified by a shortfall, a new commitment, or cost growth —
  and is only one of two honest responses; the other is a visible benefit/scope
  reduction. The `shortfall_rule` forces the choice into the open rather than
  letting borrowing absorb it.
- **Lowering a rate** is earned by a surplus, an efficiency gain, or a completed
  mission — and the freed money goes to a rate cut *or* debt paydown, never to a
  quiet sweep into another lane.

In both directions the borrowed-share line is recomputed and shown, so a change can
never make the budget *look* balanced when it is not.

## Worked-example slots (Pulse 03)

The operating model will be demonstrated with end-to-end `lane_rate_change` records:
defense up on the Hague commitment; health down on an efficiency gain; Social
Security cap raised on a trust-fund shortfall.

## Boundaries kept

- Operating model, not statutory text.
- Automatic triggers bind to independent actual-data, not forecasts.
- No rate change without a published explanation and a recomputed borrowed share.
- `allocation_method = proposed_reform` for any not-yet-enacted lane.
