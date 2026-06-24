# Program-Lane Rate Model (Receipt-Share Framing)

## Purpose

This derived record expresses each public-purpose lane as its **share of a fully
funded federal tax dollar** — the first concrete "fair rate" output of the
program-lane rate design wave. It answers: if federal receipts covered outlays
(no borrowing), what share of every tax dollar would each lane consume, and what
is the historical/international case for that level?

It is a **proposed-reform visibility model**, not legal dedication and not a
balanced-budget claim.

## Model

- `model_id`: `program-lane-receipt-share-v1`
- `rate_framing`: `receipt-share` (share of funded total). The statutory
  `rate-on-base` framing (e.g., "X% of taxable income") is deferred until the
  aggregate income/payroll base is extracted (wave Pulse 03).
- `allocation_method`: always `proposed_reform`.

## Inputs

| Source ID | Role |
|---|---|
| `SRC-OMB-HIST-1-1-FY2027` | FY2025 total outlays, receipts, deficit. |
| `SRC-OMB-HIST-3-2-FY2027` | FY2025 outlays by function -> lane cost. |
| `SRC-OECD-REVSTATS-2025` | International tax-burden anchor (see benchmark note). |

Per-lane international and historical anchors come from
`docs/research/2026-06-23-international-historical-benchmark.md`.

## Method

```text
current_cost_share = lane_cost / total_outlays * 100
recommended_receipt_share = current_cost_share   (current-cost basis, v1)
```

Lane costs are reconciled from OMB Table 3.2 functions to the 16-lane crosswalk
and **sum exactly to total outlays** (7,011,105 $M FY2025); the 17 receipt shares
sum to 100.0000%. Two lanes (Commerce/Housing Credit, Undistributed Offsetting
Receipts) are net-negative and display as offsets.

`target_cost_basis` records whether the lane's target cost should hold or move on
the international evidence: `international-norm` for Health and Medicare (US spends
~2x peers per the benchmark note), `policy-band` for Defense, `hold-current`
otherwise. v1 does not yet quantify the adjusted target costs; it records the
basis and direction.

## Solvency context (carried on every row)

| Measure | FY2025 ($M) |
|---|---:|
| Total outlays | 7,011,105 |
| Total receipts | 5,236,421 |
| Deficit (borrowed) | 1,774,684 |
| Receipts coverage of outlays | 74.69% |
| Aggregate receipt lift to fully fund | +33.89% |
| Borrowed share of outlays | 25.31% |

## Validation

- Structural: file parses as JSONL; lane costs reconcile to total outlays; shares
  sum to 100%.
- `git diff --check`.
- Formal `taxlane-tools` validation of this new family is a follow-up pulse.

## What this is not

- Not legal dedication of any current receipt source.
- Not a statutory rate on income; that framing awaits the aggregate base.
- Not a balanced-budget assertion; the borrowed share is shown, not hidden.
- Not a fraud finding; efficiency targets are argued, not alleged.
