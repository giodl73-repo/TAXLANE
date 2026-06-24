# Balance-Rule Guardrail Specification

## Decision supported

The eight-role panel (`reviews/2026-06-23-fair-rate-panel.md`) accepted the
receipt-share lane model but raised a blocking objection from the Reform Skeptic:
a "spend only what we collect" rule and named lane rates are **transparency, not
control**, and every historical balanced-budget regime has been gamed. This spec
answers that objection by defining the guardrails a balance rule needs to be
real. It is reform-design, not statutory text.

## The rule, stated precisely

> In each measurement period, total federal outlays may not exceed total federal
> receipts actually collected in that period, measured on a consistent accrual
> basis, with net interest funded first and trust-fund lanes balanced on their own
> dedicated receipts.

Four precision requirements follow, each closing a known loophole:

1. **Measurement basis: accrual, multi-year true-up.** Cash-basis annual balance
   invites timing games (push a payment past Sept 30). Measure obligations when
   incurred and true up over a rolling window.
2. **Per-fund, not one pool.** The general fund balances on general-fund receipts
   (income, corporate, excise, customs); Social Security and Medicare-HI balance
   on their dedicated payroll receipts. No fund may "balance" by spending another
   fund's surplus. Every inter-fund transfer is a disclosed line, never netted
   into "balance."
3. **Net interest is senior.** Debt service is funded before any program lane.
   Skipping it is borrowing by default — the worst kind.
4. **Measured over the cycle, not the year.** A pre-funded reserve (built in
   surplus years) absorbs recessions so the rule does not force pro-cyclical
   austerity. The reserve is saving, not back-door borrowing, and is capped.

## Statutory lane fields each balance rule requires

These extend the lane object model in `program-linked-tax-model.md`. No lane is
"balanced" in name until all are populated:

| Field | Guardrail purpose |
|---|---|
| `appropriation_rule` | Whether spending needs annual appropriation, standing authority, or both — names who can spend. |
| `shortfall_rule` | What happens if receipts fall below target outlays: a *visible* rate increase or a *visible* benefit reduction — never a silent backfill. |
| `surplus_rule` | Surplus is locked in-lane or applied to debt paydown; it may not be swept to cover another lane without a supermajority. |
| `reserve_rule` | Whether and how balances accumulate as the over-the-cycle reserve, with a cap. |
| `override_rule` | Exactly how Congress may alter the lane/rate/base, and the vote threshold required. |
| `deficit_gap_display` | The borrowed share remains a visible line even when nominally zero. |

## Evasion-to-guardrail map

The Reform Skeptic named ten evasions. Each gets a specific guardrail.

| Evasion | How it breaks the rule | Guardrail |
|---|---|---|
| E1 Optimistic baselines | Score receipts high so it "balances" on paper | Independent, non-partisan locked revenue estimate; sequester triggers on **actual** receipts, not forecast |
| E2 Timing shifts | Move a payment across the fiscal line | Accrual measurement + multi-year true-up |
| E3 Off-budget vehicles | Hide spending in GSEs, credit guarantees, "independent" funds | Count credit subsidy and contingent liabilities in-lane at fair value; no new off-budget category without supermajority |
| E4 Emergency designation | Declare spending "emergency" to exempt it | Hard statutory emergency definition + supermajority vote + automatic future-year payback into the breached lane |
| E5 Surplus sweeping | Sweep a lane surplus to cover another's gap | `surplus_rule` locks surpluses in-lane or to debt; override needs supermajority |
| E6 Shortfall backfill | General fund quietly backfills a protected lane | `shortfall_rule` forces a visible rate rise or visible benefit cut — no silent transfer |
| E7 Tax-expenditure laundering | Move spending to the credit/deduction side | Count refundable credits and major preferences as in-lane spending; cap tax expenditures inside the rule |
| E8 Entitlement autopilot | Formula growth structurally breaches the cap, then waived | Pre-committed triggers auto-adjust formula or rate on breach — not re-litigated annually |
| E9 Rate-base flight | Set a lane rate so high the base avoids/relocates | Realistic elasticity in the locked estimate; rates set with avoidance built in, not headline arithmetic |
| E10 "It balances" laundering | A balanced chart hides the residual gap | Mandatory always-visible `deficit_gap` lane + published forecast-vs-actual reconciliation |

## Institutional triggers

- **Independent estimator.** A non-partisan body produces the locked revenue and
  cost estimates; the rule binds to its numbers, not to the majority's.
- **Actual-receipts sequester.** If actual receipts undershoot, an automatic,
  pre-defined adjustment closes the gap (rate step-up and/or formula trim per each
  lane's `shortfall_rule`) — no new vote required to enforce, a supermajority
  required to suspend.
- **Override transparency.** Any change to a lane/rate/base/appropriation is
  recorded, timestamped, attributed, and shown against the deficit_gap line, so a
  future majority *can* change the rule but cannot do it quietly.

## What this spec deliberately does not claim

- It does not claim guardrails make a future Congress permanently bound; it makes
  evasion **visible and costly**, which is the achievable goal.
- It does not assert that lane rates by themselves improve discipline — the panel
  rejected that. Discipline lives in the appropriation, shortfall, surplus, and
  override rules, not in the rate label.
- It is reform-design, not bill text, and is subject to role review before any
  public advocacy.

## Adopt now

- Treat the six statutory lane fields as required before any lane is described as
  "balanced."
- Treat the always-visible `deficit_gap` line and the actual-receipts sequester as
  non-negotiable for any balanced-lane public artifact.

## Defer

- Statutory language and constitutional-vs-statutory placement of the rule.
- The numeric reserve cap and sequester step sizes (need the receipts split and
  base from Pulses 02-03 and an elasticity model).
