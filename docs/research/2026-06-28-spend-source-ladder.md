# Spend Source Ladder

## Purpose

This note defines how TAXLANE should move from "where federal money goes" at the
OMB function/subfunction level toward deeper agency, account, program, and
recipient questions without overstating what each source proves.

## Ladder

| Level | Source family | What it can support | What it cannot support by itself |
|---:|---|---|---|
| 1 | OMB Table 1.1 | Total receipts, outlays, surplus/deficit, borrowing context | Category-level spending or legal dedication |
| 2 | OMB Table 3.1 | Broad outlays by function | Program ownership, recipient detail, performance |
| 3 | OMB Table 3.2 | Outlays by function and subfunction | Agency/account/program ownership when a subfunction spans multiple owners |
| 4 | OMB Table 4.1 | Outlays by agency | Public-purpose category unless crosswalked to functions/subfunctions |
| 5 | OMB Tables 8.5 and 8.7 | Mandatory and discretionary program views | Recipient-level spending or performance |
| 6 | OMB Table 11.3 | Payments for individuals by category and major program | Non-individual public goods or recipient-level award detail |
| 7 | Agency budget justifications | Account/program-activity detail and agency narrative | Whole-government comparability without reconciliation |
| 8 | Treasury Monthly Statement | Current-period receipts, outlays, deficit/surplus | Stable annual historical spine until fiscal year closes |
| 9 | USAspending | Awards, obligations, accounts, recipients, object classes | Final outlays unless linked to outlay source; performance or waste |
| 10 | GAO, CBO, inspectors general, program evaluations | Performance, risk, duplication, improper payments, control findings | Comprehensive spend ledger unless tied back to fiscal totals |

## Rule

Use the highest source level needed for the question and the lowest claim level
the evidence supports.

Example: OMB Table 3.2 can say that FY2025 Social Security outlays were the
largest subfunction. It cannot say which local office, recipient, vendor, or
administrative process caused cost growth. That requires a deeper source.

## First derived family

`data/derived/spend_category_map/` records the top FY2025 spend categories at
the OMB subfunction level. Each row carries:

- source level,
- source ID,
- fiscal year,
- function and subfunction labels,
- outlay amount,
- share of total outlays,
- modeled income-tax allocation,
- funding caveat,
- next-source need.

These rows are entry points for questions, not findings.

## Source-use guardrails

- Use OMB historical tables for the annual federal spine.
- Use USAspending only after recording query parameters and source date.
- Use agency budget justifications for program-activity explanations, not for
  whole-budget totals unless reconciled.
- Use GAO/CBO/IG reports for evaluated risk or performance claims.
- Do not label "waste" unless a source specifically supports that finding.
