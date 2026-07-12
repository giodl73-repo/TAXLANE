# Wave: Spending-Side Map

## Goal

Turn TAXLANE's outlay-side records into a plain-language map of where federal
money actually goes: major lanes, subfunctions, agencies/programs where source
support allows, financing context, and follow-up accountability questions.

This wave stays inside TAXLANE unless the work becomes a reusable spend
classification product beyond federal tax/spend legibility.

## Pulse Table

| Pulse | Title | Status | Outcome |
|------:|---|---|---|
| 01 | Spend-side scope decision | done | Defined the boundary between TAXLANE spend analysis and any future standalone `SPENDCAT` repo. |
| 02 | FY2025 top-spend packet | done | Published a reader packet showing the largest FY2025 outlay subfunctions, with income-tax and borrowed-share context. |
| 03 | Source ladder for deeper spend | done | Identified which questions can be answered from OMB tables, which need agency budget tables, and which need USAspending or evaluation sources. |
| 04 | Spend-category record family | done | Added a draft derived record shape for top FY2025 spend categories with source level, amount, share, funding caveat, and evidence status. |
| 05 | Accountability handoff | done | Converted large spend categories into safe public questions without making fraud, waste, abuse, or performance findings. |

## Status

Complete. The wave now has a same-repo scope decision, FY2025 reader packet,
source ladder, draft spend-category record family, and accountability question
handoff.

## Design rules

- Keep "where outlays go" separate from "which tax dollar legally paid for it."
- Always show borrowing and total-receipts context beside income-tax allocation.
- Label every spend record by source level: function, subfunction, agency,
  account, program activity, award/recipient, or unsupported.
- Treat USAspending as obligation/award evidence unless an outlay source supports
  stronger wording.
- Do not infer fraud, waste, abuse, duplication, or performance failure from
  spend size alone.

## Same Repo vs. SPENDCAT

Keep this wave in TAXLANE while the object is federal fiscal legibility:
taxpayers, receipts, outlays, borrowing, and public-purpose lanes.

Consider a standalone `SPENDCAT` repo only if the core artifact becomes a
general-purpose spending categorizer that can classify and reconcile spend data
across federal agencies, state/local budgets, grants, nonprofits, businesses, or
household ledgers.

## Validation

```powershell
git diff --check
```
