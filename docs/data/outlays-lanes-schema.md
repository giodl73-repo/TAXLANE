# Outlays and Lane Crosswalk Schema

## Purpose

The `outlay_function`, `outlay_program`, and `lane_crosswalk` record families
capture federal spending by fiscal year and map source categories to TAXLANE's
reader-facing public-purpose lanes.

This file defines extraction and crosswalk schemas only. It does not import OMB
or USAspending data, and it does not allocate any taxpayer receipt.

## Source anchors

| Source ID | Role |
|---|---|
| `SRC-OMB-HIST-1-1-FY2027` | Top-level receipts, outlays, and surplus/deficit fiscal spine. |
| `SRC-OMB-HIST-3-1-FY2027` | Outlays by superfunction and function. |
| `SRC-OMB-HIST-3-2-FY2027` | Outlays by function and subfunction. |
| `SRC-OMB-HIST-4-1-FY2027` | Outlays by agency. |
| `SRC-OMB-HIST-8-5-FY2027` | Mandatory and related program outlays. |
| `SRC-OMB-HIST-8-7-FY2027` | Discretionary program outlays. |
| `SRC-OMB-HIST-11-3-FY2027` | Payments for individuals by category and major program. |
| `SRC-OMB-AP-6-CONCEPTS-FY2027` | Budget concepts for outlays, obligations, deficit, and surplus. |
| `SRC-OMB-AP-9-OFFSETTING-FY2027` | Offsetting collections and offsetting receipts distinction. |
| `SRC-USASPENDING` | Program/account exploration after OMB function records exist. |

## Record identity

Use deterministic IDs:

```text
outlay-function:{fiscal_year}:{function_code}:{subfunction_code}:{measure}
outlay-program:{fiscal_year}:{program_key}:{measure}
lane:{lane_id}:{model_version}
```

Examples:

- `outlay-function:2025:050:051:outlays`
- `outlay-function:2025:900:total:outlays`
- `outlay-program:2025:social-security:outlays`
- `lane:income-security:omb-fy2027-v1`

## `outlay_function` required fields

| Field | Type | Required | Meaning |
|---|---|---:|---|
| `record_id` | string | Yes | Stable row identifier. |
| `record_family` | string | Yes | Always `outlay_function`. |
| `fiscal_year` | integer | Yes | Federal fiscal year. |
| `year_basis` | string | Yes | Always `fiscal_year`. |
| `source_ids` | string list | Yes | OMB historical table and concept source IDs. |
| `source_table` | string | Yes | OMB historical table ID and release, such as `Table 3.2 FY2027`. |
| `source_row_ref` | string | Yes | Source row label, section, or cell anchor. |
| `superfunction` | string or null | Yes | OMB superfunction when provided. |
| `function_code` | string | Yes | OMB function code or stable source label when no code is present. |
| `function_label` | string | Yes | Exact OMB function label. |
| `subfunction_code` | string or null | Yes | OMB subfunction code when provided. |
| `subfunction_label` | string or null | Yes | Exact OMB subfunction label when provided. |
| `measure` | string | Yes | `outlays`, `share_of_total`, or source-specific measure. |
| `amount` | decimal or null | Yes | Numeric value for amount measures. |
| `percent` | decimal or null | Yes | Numeric value for percent measures. |
| `amount_units` | string | Yes | `millions_usd`, `billions_usd`, `percent`, or source-specific unit. |
| `actual_or_projection` | string | Yes | `actual`, `estimate`, `projection`, or `unknown`. |
| `offsetting_treatment` | string | Yes | `gross`, `net`, `offsetting-receipts`, `undistributed-offsetting-receipts`, `unknown`. |
| `status` | string | Yes | `draft`, `extracted`, `reviewed`, `superseded`, or `retired`. |
| `observed_date` | date | Yes | Date the source was observed or fetched. |
| `notes` | string | No | Extraction caveat. |

## `outlay_program` required fields

| Field | Type | Required | Meaning |
|---|---|---:|---|
| `record_id` | string | Yes | Stable row identifier. |
| `record_family` | string | Yes | Always `outlay_program`. |
| `fiscal_year` | integer | Yes | Federal fiscal year. |
| `year_basis` | string | Yes | Always `fiscal_year`. |
| `source_ids` | string list | Yes | OMB table, USAspending query, or concept source IDs. |
| `source_table` | string | Yes | OMB historical table ID, USAspending query ID, or release note. |
| `source_row_ref` | string | Yes | Source row label, program line, account, or query anchor. |
| `program_key` | string | Yes | Stable TAXLANE program key. |
| `source_program_label` | string | Yes | Exact source label as written. |
| `agency_label` | string or null | Yes | Source agency label when provided. |
| `account_label` | string or null | Yes | Source account label when provided. |
| `budget_category` | string | Yes | `mandatory`, `discretionary`, `net-interest`, `offsetting`, `mixed`, or `unknown`. |
| `beneficiary_view` | string | Yes | `payments-for-individuals`, `public-service`, `debt-service`, `administration`, `mixed`, or `unknown`. |
| `linked_function_record_ids` | string list | Yes | Related `outlay_function` records. |
| `measure` | string | Yes | `outlays`, `obligations`, `budget_authority`, or source-specific measure. |
| `amount` | decimal or null | Yes | Numeric value for amount measures. |
| `amount_units` | string | Yes | `millions_usd`, `billions_usd`, or source-specific unit. |
| `actual_or_projection` | string | Yes | `actual`, `estimate`, `projection`, or `unknown`. |
| `status` | string | Yes | `draft`, `extracted`, `reviewed`, `superseded`, or `retired`. |
| `observed_date` | date | Yes | Date the source was observed or fetched. |
| `notes` | string | No | Extraction caveat. |

## `lane_crosswalk` required fields

| Field | Type | Required | Meaning |
|---|---|---:|---|
| `record_id` | string | Yes | Stable row identifier. |
| `record_family` | string | Yes | Always `lane_crosswalk`. |
| `model_version` | string | Yes | Crosswalk version, such as `omb-fy2027-v1`. |
| `lane_id` | string | Yes | Stable TAXLANE public-purpose lane key. |
| `public_label` | string | Yes | Reader-facing lane name. |
| `source_ids` | string list | Yes | Sources supporting the crosswalk. |
| `included_function_codes` | string list | Yes | OMB functions included in the lane. |
| `included_subfunction_codes` | string list | Yes | OMB subfunctions included in the lane. |
| `excluded_function_codes` | string list | Yes | Explicitly excluded functions or subfunctions. |
| `linked_program_keys` | string list | Yes | Program records that inform or explain the lane. |
| `spending_control` | string | Yes | `mandatory`, `discretionary`, `trust-fund`, `net-interest`, `offsetting`, or `mixed`. |
| `legal_status_today` | string | Yes | `general-fund`, `dedicated`, `mixed`, `modeled`, or `non-tax-context`. |
| `allocation_method_allowed` | string list | Yes | Allowed public allocation methods. |
| `deficit_context_required` | boolean | Yes | Whether public display must show borrowing/deficit context. |
| `review_status` | string | Yes | `draft`, `source-reviewed`, `budget-reviewed`, `approved`, or `retired`. |
| `notes` | string | No | Crosswalk caveat. |

## Controlled values

Use these initial `lane_id` values:

- `national-defense`
- `veterans`
- `international-affairs`
- `health`
- `medicare`
- `income-security`
- `social-security`
- `education-training-employment-social-services`
- `transportation`
- `environment-energy-natural-resources`
- `justice-general-government`
- `community-regional-development`
- `agriculture`
- `science-space-technology`
- `commerce-housing-credit`
- `net-interest`
- `undistributed-offsetting-receipts`
- `borrowed-share-deficit-gap`
- `other`

Preserve exact OMB labels in function and program source-label fields. Use
TAXLANE lane labels only in `public_label`.

## Extraction rules

1. Treat OMB historical table years as fiscal years unless the source states
   otherwise.
2. Extract Table 3.1 superfunction/function records before Table 3.2
   subfunction records.
3. Preserve OMB source row labels exactly in `function_label`,
   `subfunction_label`, or `source_program_label`.
4. Keep agency records from Table 4.1 separate from function records. Agency is
   not a substitute for public purpose without a crosswalk.
5. Keep mandatory and discretionary program records separate unless a source row
   explicitly combines them.
6. Keep payments-for-individuals records separate from all public benefits.
7. Label net interest as its own lane; do not distribute it silently across
   program lanes.
8. Label undistributed offsetting receipts separately when present in OMB
   function tables.
9. Use USAspending only for program/account exploration after the OMB
   function-level spine exists, and record query parameters before citing it.

## Reconciliation checks

Before marking records `reviewed`, check:

1. `fiscal_year` is an integer and `year_basis` is `fiscal_year`.
2. All source IDs exist in `docs/sources/source-version-ledger.md`.
3. Amount rows have `amount` and null `percent`.
4. Percent rows have `percent` and null `amount`.
5. Table 3.1 function totals reconcile to Table 1.1 total outlays within the
   source's displayed precision.
6. Table 3.2 subfunction totals reconcile to their parent functions within the
   source's displayed precision.
7. Program records from Tables 8.5, 8.7, and 11.3 are not treated as complete
   government-wide spending totals unless reconciled to function records.
8. Every lane crosswalk includes explicit inclusions, exclusions, and allowed
   allocation methods.
9. Every public allocation using outlays shows deficit context or links to a
   record explaining why deficit context is not applicable.

## Public wording rules

| Data condition | Public wording |
|---|---|
| Function outlay | "OMB reports federal outlays for..." |
| Subfunction detail | "Within that function, OMB reports..." |
| Program detail | "This program-level view is narrower than total function spending." |
| Agency detail | "Agency spending is an administrative view, not automatically a public-purpose lane." |
| Net interest | "Net interest is shown separately because it pays financing costs, not a program service." |
| Undistributed offsetting receipts | "OMB records this as an offset to spending totals, not as a direct program outlay." |
| Borrowed share | "Federal spending also reflects borrowing when outlays exceed receipts." |

## Open questions

- Whether the first public lane model should follow OMB functions exactly or use
  a smaller reader-facing lane set with documented rollups.
- Whether USAspending query snapshots belong in a later `program_account`
  record family.
- Whether deficit allocation should be a lane, a display companion, or both.
