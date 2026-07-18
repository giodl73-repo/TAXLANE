# From Country Comparisons To Balanced Lane Rates

Machine record:
`data/derived/breadth_benchmark_matrix/program_lane_target_cost_contract.v1.draft.json`.

## What This Adds

Taxlane already knows today's FY2025 lane costs and can make them sum to 100
percent of a fully funded tax dollar. That arithmetic does not establish what a
lane *should* cost. The target-cost contract defines the missing step for all 15
comparison lanes.

For every lane, it now names:

1. the service, population, asset, policy band, or endogenous fiscal quantity
   that determines target cost;
2. the policy mechanism that could move the United States toward that target;
3. the outcome and adequacy floors that block a false efficiency cut;
4. the translation from all-government peer data to the federal budget;
5. the receipt base and ten-year solver treatment still required.

## The Rate And Share Quantities

| Public question | Calculation | Reconciliation rule |
|---|---|---|
| Where does fully funded federal cost go before dedicated receipts? | gross program cost / total funded federal cost | This is the all-receipt funding share. |
| Which residual general-fund needs remain after dedicated receipts and offsets? | residual general-fund need / total residual general-fund need | This is the residual general-fund requirement share. |
| What rate funds the lane? | lane required revenue / behaviorally adjusted assigned base | Revenue equals cost; statutory rates do not need to sum to 100 percent. |

A value calculated after subtracting dedicated receipts is not "share of every
tax dollar." It must be labeled as a residual general-fund requirement share.

The central accounting identity is:

```text
program outlays + net interest + reserves
  = dedicated receipts + general receipts + offsets + explicit deficit gap
```

The deficit gap is never hidden. Net interest is recomputed after changes to the
primary balance rather than held fixed.

Pulse 72 also freezes the sign conventions and identities in
`data/derived/breadth_benchmark_matrix/fiscal_accounting_rate_definitions.v1.draft.json`.
Public rounded views must put any residual on an explicit rounding line.

## Why The Contract Does Not Publish New Numbers Yet

A peer median can describe normal practice. A favorable quartile can anchor a
scenario only after outcome, adequacy, transferability, and stability tests. It
cannot directly turn a spending difference into a federal cut.

Health is closest to numeric calibration because Taxlane already has price,
volume, Medicare-relative, access, and scenario artifacts. Net interest must be
solved endogenously. Defense and disaster use policy or exposure bands.
Payment integrity and veterans use structured cases. Other lanes still require
an explicit service package, federal-share translation, or outcome bridge.

## Required Scenario Set

- `current_law`: the unchanged policy path;
- `central_reform`: the highest evidence-supported attainable path;
- `stress`: adverse demographics, utilization, exposure, revenue, or interest.

No numeric target cost, balanced receipt share, statutory rate, or savings claim
is opened by this design artifact.
