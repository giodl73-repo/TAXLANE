# Wave: Receipt Source Allocation Review

## Goal

Review the first OMB receipt-source slice against the newly reviewed AP13
budget concepts so allocation labels can move only where the sources support
them.

## Thesis

TAXLANE can now improve the first Table 2.1 receipt records without building a
taxpayer receipt. Individual income-tax rows can carry a reviewed
`general_receipt` label, while other receipt categories should remain blocked
or mixed until their own budget-concept support exists.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|---|---|---|
| 01 | Individual income receipt allocation | done | Promoted the first receipt-source slice to source-reviewed and labeled individual income-tax rows as general receipts. |
| 02 | Other receipt category blockers | done | Recorded blockers for corporation, social-insurance, excise, other, and total receipt categories. |
| 03 | Receipt-source public wording update | pending | Tighten public wording for receipt-source allocation labels. |

## Success Criteria

- Numeric Table 2.1 values remain unchanged.
- Individual income-tax receipt rows cite AP13 concept support before using
  `general_receipt`.
- Social-insurance receipts stay separate from individual income taxes.
- Excise and other receipts are not generalized as ordinary income-tax revenue.
- Validation commands pass.

## Non-Goals

- Do not build taxpayer receipts in this wave.
- Do not allocate receipts to program outlays.
- Do not classify social-insurance or excise subcomponents without Table 2.4
  review.
- Do not treat corporation income taxes as individual income taxes.

## Validation

Run:

```powershell
git diff --check
```

For JSONL receipt pulses, parse every record as JSON and recheck Table 2.1
totals against the recorded Table 1.1 reconciliation.
