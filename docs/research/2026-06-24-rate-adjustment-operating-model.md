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
| `distributional_impact` | Who bears a rate increase or benefit trim, by income — not just who is "affected." |
| `coverage_floor` | The coverage/enrollment/eligibility floor the change may not breach, and the verification that outcomes held constant — a blocking field for any down/shortfall move. |
| `compliance_impact` | Whether the change requires re-withholding, a new form/line, or preparer changes. |
| `contestation` | How contested/hard-to-enact the change is, so the record shows the fight, not only the cost. |
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

## Hardening rules (from the pressure-test review)

The pressure-test (`reviews/2026-06-24-operating-system-panel.md`) found the
operating model had dropped principles the guardrail spec already established.
These are now binding:

1. **Over-the-cycle reserve, not annual austerity.** A shortfall from a *cyclical*
   revenue dip draws the pre-funded reserve **before** any rate rise or benefit
   sequester fires. The `surplus_rule` routes surpluses to the reserve (then debt)
   so the rule is measured over the cycle, not the year — no pro-cyclical cuts into
   a recession.
2. **Net interest is senior and sequester-exempt.** Debt service is the first claim
   on receipts; it is never subject to the `shortfall_rule` or the sequester. A
   debt-service-growth trigger is added so a rising interest lane is visible, not
   static.
3. **Coverage floor on efficiency moves — and on shortfall cuts.** A "down on
   efficiency" change (e.g., health) requires a **coverage/enrollment floor** and
   **independent verification that outcomes and served population held constant**
   before the saving is booked. "Coverage protected" is a rule with a check, not an
   adjective; a cut that can't be distinguished from reduced service does not qualify
   as efficiency. The **same floor applies to a shortfall-driven benefit cut**: the
   `shortfall_rule` may force a *visible* benefit reduction, but never below the
   coverage/eligibility floor without its own published distributional record. The
   floor is recorded in the `coverage_floor` field below; a change that breaches it
   cannot be booked.
4. **Receipt firewall.** A lane is a **line on the receipt** — it never acquires its
   own filing, base, or withholding. Any rate change that affects payroll
   withholding takes effect only at a **tax-year boundary** with fixed minimum
   notice, to avoid mid-cycle employer churn.
5. **Harden the two load-bearing nodes.** The independent estimator needs
   protected appointment and funding (anti-capture), or it cannot bind the
   triggers. The emergency suspend/payback path must **cap deferrals** and make each
   re-waiver cost more — closing the rolling-payback evasion (E11) the review named.
6. **Long-term drivers show a horizon.** A change to a structural driver (Social
   Security, Medicare) must show a **multi-year solvency path**, not a single-year
   gap number — a one-year cap lift that leaves the trajectory open is not a fix.

## Worked-example slots (Pulse 03)

The operating model will be demonstrated with end-to-end `lane_rate_change` records:
defense up on the Hague commitment; health down on an efficiency gain; Social
Security cap raised on a trust-fund shortfall.

## Boundaries kept

- Operating model, not statutory text.
- Automatic triggers bind to independent actual-data, not forecasts.
- No rate change without a published explanation and a recomputed borrowed share.
- `allocation_method = proposed_reform` for any not-yet-enacted lane.
