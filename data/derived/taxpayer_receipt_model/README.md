# Taxpayer Receipt Model Draft

## Purpose

This directory contains draft visibility receipt prototypes. A visibility
receipt is an explanatory model, not a tax calculator, legal dedication claim,
or statutory lane proposal.

The first prototype uses a placeholder `$1,000` ordinary individual income-tax
payment and fiscal year 2025 OMB outlay shares. It answers:

> If a placeholder ordinary income-tax payment were allocated across TAXLANE
> public-purpose lanes by FY2025 outlay share, what would the labeled view look
> like?

## Current Artifact

| Artifact | Grain | Status |
|---|---|---|
| `taxpayer_receipt_model.placeholder-1000.fy2025.omb-fy2027-v1.draft.json` | one placeholder receipt scenario | Draft |

## Source Chain

- Lane labels and function/subfunction mapping:
  `data/derived/lane_crosswalk/lane_crosswalk.omb-fy2027-v1.2026-06-22.draft.jsonl`
- Allocation shares:
  `data/derived/income_tax_outlay_subfunction_model/income_tax_outlay_subfunction_model.omb-fy2027.2026-06-21.draft.jsonl`
- Financing context:
  `data/derived/income_tax_outlay_model/income_tax_outlay_model.omb-fy2027.2026-06-21.draft.jsonl`

## Public-Use Boundary

This prototype may be used for design review only. Before a taxpayer-facing
receipt is published, it needs role review for:

1. allocation method wording,
2. negative offset rows,
3. Social Security and Medicare financing caveats,
4. deficit and borrowed-share display,
5. plain-language presentation.

## Validation

Run:

```powershell
git diff --check
```
