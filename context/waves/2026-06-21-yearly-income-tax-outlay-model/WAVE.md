# Wave: Yearly Income-Tax Outlay Model

## Goal

Build a source-labeled fiscal-year model that allocates individual income-tax
receipts across broad OMB outlay categories by each category's share of total
federal outlays.

## Thesis

TAXLANE can answer "what percentage of spending categories would income tax map
to by year?" only as a transparent model. Ordinary individual income tax remains
a general receipt unless a legal dedication source says otherwise, so the first
yearly output must preserve its modeled status and deficit context.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|---|---|---|
| 01 | Model contract and source profile | done | Defined method, record shape, source coverage, and validation checks. |
| 02 | Annual draft model rows | done | Generated fiscal-year 1940-2025 broad-category modeled allocation rows. |
| 03 | Reconciliation review | done | Verified year totals, source row anchors, and modeled-allocation sums. |
| 04 | Reader-facing note | done | Summarized how to explain the modeled percentages without implying legal dedication. |

## Success Criteria

- Every row is fiscal-year based and labeled as modeled.
- Individual income-tax receipts stay separate from payroll, corporate, excise,
  and other receipts.
- The method uses Table 3.1 broad-category outlay shares and Table 2.1
  individual income-tax receipts.
- Deficit or surplus context from Table 1.1 is carried on every modeled row.
- Net interest and undistributed offsetting receipts remain visible.
- Actual years are separated from estimates and projections.

## Non-Goals

- Do not claim legal tracing of income-tax dollars to programs.
- Do not use subfunction or program-level allocation in this wave.
- Do not include 2026-2031 estimates/projections in the first draft dataset.
- Do not turn this visibility model into a reform proposal.

## Validation

Run:

```powershell
python data/derived/income_tax_outlay_model/build_income_tax_outlay_model.py --check
git diff --check
```

The generator must reconcile Table 3.1 total outlays to Table 1.1 for each
modeled year and verify that category allocations sum back to individual
income-tax receipts.
