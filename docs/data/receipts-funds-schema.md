# Receipts and Fund Schema

## Purpose

The `receipt_source` and `fund_group` record families capture federal receipts
by source and fund-group treatment by fiscal year. They are the budget-accounting
spine for any future TAXLANE claim about where income-tax receipts enter the
federal budget.

This file defines extraction schemas only. It does not import OMB spreadsheets.

## Source anchors

| Source ID | Role |
|---|---|
| `SRC-OMB-HIST-1-1-FY2027` | Top-level receipts, outlays, and surplus/deficit fiscal spine. |
| `SRC-OMB-HIST-1-4-FY2027` | Receipts, outlays, and surplus/deficit by fund group. |
| `SRC-OMB-HIST-2-1-FY2027` | Receipts by source. |
| `SRC-OMB-HIST-2-2-FY2027` | Percentage composition of receipts by source. |
| `SRC-OMB-HIST-2-4-FY2027` | Social insurance, retirement receipts, and excise composition. |
| `SRC-OMB-AP-6-CONCEPTS-FY2027` | Budget concepts for receipts, outlays, deficit, and surplus. |
| `SRC-OMB-AP-8-RECEIPTS-FY2027` | Governmental receipts definitions and treatment. |
| `SRC-OMB-AP-9-OFFSETTING-FY2027` | Offsetting collections and offsetting receipts distinction. |
| `SRC-OMB-AP-13-FUNDS-FY2027` | General fund, trust fund, special fund, and dedicated collection concepts. |

## Record identity

Use deterministic IDs:

```text
receipt:{fiscal_year}:{receipt_category}:{measure}
fund:{fiscal_year}:{fund_group}:{measure}
```

Examples:

- `receipt:2025:individual-income-taxes:amount`
- `receipt:2025:individual-income-taxes:share-of-total`
- `fund:2025:federal-funds:receipts`
- `fund:2025:trust-funds:outlays`

## `receipt_source` required fields

| Field | Type | Required | Meaning |
|---|---|---:|---|
| `record_id` | string | Yes | Stable row identifier. |
| `record_family` | string | Yes | Always `receipt_source`. |
| `fiscal_year` | integer | Yes | Federal fiscal year, not tax year. |
| `source_ids` | string list | Yes | OMB table or budget concept source IDs. |
| `source_table` | string | Yes | OMB historical table ID and release, such as `Table 2.1 FY2027`. |
| `source_row_ref` | string | Yes | Source row label, section, or cell anchor. |
| `receipt_category` | string | Yes | Stable receipt category. |
| `source_receipt_label` | string | Yes | Exact source label as written. |
| `measure` | string | Yes | `amount`, `share_of_total`, or `subcomposition_amount`. |
| `amount` | decimal or null | Yes | Numeric value for amount measures. |
| `percent` | decimal or null | Yes | Numeric value for percent measures. |
| `amount_units` | string | Yes | `millions_usd`, `billions_usd`, `percent`, or source-specific unit. |
| `actual_or_projection` | string | Yes | `actual`, `estimate`, `projection`, or `unknown`. |
| `fund_group_link` | string or null | Yes | Related fund-group record when available. |
| `allocation_status` | string | Yes | `general_receipt`, `dedicated_receipt`, `offsetting`, `mixed`, or `unknown`. |
| `status` | string | Yes | `draft`, `extracted`, `reviewed`, `superseded`, or `retired`. |
| `observed_date` | date | Yes | Date the source was observed or fetched. |
| `notes` | string | No | Extraction caveat. |

## Receipt category controlled values

Use these stable categories first:

- `individual-income-taxes`
- `corporation-income-taxes`
- `social-insurance-and-retirement-receipts`
- `excise-taxes`
- `estate-and-gift-taxes`
- `customs-duties`
- `miscellaneous-receipts`
- `other-receipts`
- `total-receipts`
- `source-subtotal`
- `source-other`

Preserve exact OMB wording in `source_receipt_label`.

## `fund_group` required fields

| Field | Type | Required | Meaning |
|---|---|---:|---|
| `record_id` | string | Yes | Stable row identifier. |
| `record_family` | string | Yes | Always `fund_group`. |
| `fiscal_year` | integer | Yes | Federal fiscal year. |
| `source_ids` | string list | Yes | OMB table and budget concept source IDs. |
| `source_table` | string | Yes | OMB historical table ID and release. |
| `source_row_ref` | string | Yes | Source row label, section, or cell anchor. |
| `fund_group` | string | Yes | Stable fund group. |
| `source_fund_label` | string | Yes | Exact source label as written. |
| `measure` | string | Yes | `receipts`, `outlays`, `surplus_deficit`, or source-specific measure. |
| `amount` | decimal or null | Yes | Numeric value for amount measures. |
| `amount_units` | string | Yes | `millions_usd`, `billions_usd`, or source-specific unit. |
| `legal_dedication` | string | Yes | `none`, `dedicated`, `mixed`, or `unknown`. |
| `appropriation_required` | string | Yes | `yes`, `no`, `mixed`, or `unknown`. |
| `budget_concept_source_id` | string | Yes | OMB/GAO source supporting fund interpretation. |
| `actual_or_projection` | string | Yes | `actual`, `estimate`, `projection`, or `unknown`. |
| `status` | string | Yes | `draft`, `extracted`, `reviewed`, `superseded`, or `retired`. |
| `observed_date` | date | Yes | Date the source was observed or fetched. |
| `notes` | string | No | Extraction caveat. |

## Fund group controlled values

Use:

- `federal-funds`
- `general-fund`
- `special-funds`
- `trust-funds`
- `revolving-funds`
- `offsetting-receipts`
- `mixed`
- `total`
- `other`

Use `general-fund` only when the source or concept source explicitly supports
that level. OMB Table 1.4 may require broader `federal-funds` treatment.

## Allocation status rules

| Status | Meaning |
|---|---|
| `general_receipt` | Receipt source enters general governmental receipts without legal dedication in this record. |
| `dedicated_receipt` | Source is legally dedicated to a fund or purpose. |
| `offsetting` | Collection offsets outlays or is treated as offsetting receipt/collection. |
| `mixed` | Source includes both general and dedicated or offsetting treatment. |
| `unknown` | Extraction lacks enough fund/legal context. |

Rules:

- Individual income taxes default to `general_receipt` only after linking to OMB
  fund concepts that support ordinary income tax as general fund revenue.
- Social insurance receipts often require `dedicated_receipt` or `mixed`
  treatment; do not merge them into individual income taxes.
- Offsetting collections and offsetting receipts are not ordinary tax receipts
  for taxpayer-lane allocation.

## Extraction rules

1. Treat all OMB historical table years as fiscal years unless the source states
   otherwise.
2. Preserve OMB source row labels exactly in `source_receipt_label` or
   `source_fund_label`.
3. Preserve source units. Convert units only in a derived field or derived
   record.
4. Mark estimates and projections separately from actuals.
5. Keep Table 2.1 amount records separate from Table 2.2 percentage records.
6. Use Table 2.4 to split social-insurance and excise subcomponents only after
   recording the parent category.
7. Use Table 1.4 for fund-group context, not as proof that every receipt in a
   broad group is legally dedicated.
8. Link to budget concept sources before labeling a receipt as legally
   dedicated, offsetting, trust-fund, or general-fund.

## Reconciliation checks

Before marking records `reviewed`, check:

1. `fiscal_year` is an integer and `year_basis` is `fiscal_year`.
2. All source IDs exist in `docs/sources/source-version-ledger.md`.
3. Amount rows have `amount` and null `percent`.
4. Percent rows have `percent` and null `amount`.
5. Total receipt records from Table 2.1 reconcile to Table 1.1 within the
   source's displayed precision.
6. Table 2.2 percentage shares sum to approximately 100 percent for each fiscal
   year, allowing source rounding.
7. Fund-group totals from Table 1.4 reconcile to Table 1.1 within the source's
   displayed precision.
8. No row uses `legal_dedication = "dedicated"` without a fund/concept source.
9. No public receipt model uses `receipt_source` rows without allocation-method
   labels.

## Public wording rules

| Data condition | Public wording |
|---|---|
| Individual income tax amount | "Federal individual income-tax receipts..." |
| Payroll/social-insurance amount | "Social-insurance and retirement receipts..." |
| General receipt | "This receipt is treated as general revenue unless a legal dedication is cited." |
| Dedicated receipt | "This receipt is legally credited to a fund or purpose under the cited source." |
| Projection | "OMB projects..." |
| Actual | "OMB reports..." |
| Offsetting collection | "This collection reduces reported outlays or is recorded as an offset, not as ordinary tax revenue." |

## Open questions

- Whether to create a separate `receipt_subcomponent` family for the Table 2.4
  social-insurance and excise detail.
- Whether to derive constant-dollar receipt records in a later model family.
- How to handle years where OMB revises historical values across releases.
