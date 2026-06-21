# Dynamic Query Rules

## Purpose

Dynamic sources can change by period, query parameter, publication revision, or
API behavior. TAXLANE should not commit Treasury, USAspending, CBO candidate, or
similar query exports until the query can be reproduced and reviewed.

These rules apply before any dynamic query export is placed in `data/raw/`.

## Covered Sources

| Source ID | Source | Status |
|---|---|---|
| `SRC-TREASURY-MTS` | Treasury Monthly Treasury Statement | Allowed only with query metadata and period lock. |
| `SRC-USASPENDING` | USAspending.gov | Allowed only with endpoint, parameters, and response-scope metadata. |
| `SRC-CBO-BUDGET-DATA` | CBO budget/economic data | Candidate-only until manual verification or accessible source capture. |

## Required Metadata

Every dynamic query metadata record must include:

| Field | Meaning |
|---|---|
| `source_id` | Source ID from `docs/sources/source-version-ledger.md`. |
| `publisher` | Official publisher or steward. |
| `query_url` | Exact URL or endpoint used. |
| `query_parameters` | Complete parameter set, including defaults when known. |
| `run_timestamp` | Timestamp and timezone when the query was run. |
| `observed_date` | Date TAXLANE observed or fetched the result. |
| `response_scope` | Period, fiscal year, agency, account, program, or result set covered. |
| `pagination` | Page size, page count, cursor behavior, or result limits. |
| `sort_order` | Sort field and order, if any. |
| `filters` | All included and excluded filters. |
| `raw_path` | Repo path to the captured response, if committed. |
| `checksum_sha256` | SHA-256 checksum for the captured response body. |
| `schema_file` | Schema or extraction rule governing use. |
| `mutability_note` | Whether future requests may return different results. |
| `status` | `captured`, `draft-extracted`, `source-reviewed`, `superseded`, or `retired`. |
| `notes` | Manual steps, caveats, or blockers. |

## Raw Capture Layout

Use:

```text
data/raw/{publisher}/{source-id}/{observed-date}/{query-key}/
```

Examples:

```text
data/raw/treasury/SRC-TREASURY-MTS/2026-06-21/fy-2025-month-12/
data/raw/usaspending/SRC-USASPENDING/2026-06-21/agency-hhs-fy-2025/
```

The `query-key` should be short, stable, lowercase, and descriptive. It is not a
substitute for full query parameters in metadata.

## Treasury MTS Rules

1. Record fiscal year, month, table, and reporting period.
2. Prefer Treasury's stable CSV/API response when available.
3. Preserve whether figures are monthly, fiscal-year-to-date, or prior-year
   comparable values.
4. Do not merge Treasury monthly context with OMB annual historical tables
   without an explicit period and source-basis field.

## USAspending Rules

1. Record endpoint, request body, filters, pagination, sort order, and response
   fields.
2. Record whether results are obligations, outlays, awards, accounts, or
   recipient records.
3. Treat USAspending as program/account exploration after OMB function-level
   records exist.
4. Do not use USAspending rows as direct proof of budget-function totals without
   reconciliation.

## CBO Candidate Rules

1. Keep `SRC-CBO-BUDGET-DATA` candidate-only until an accessible source capture
   is verified.
2. Record any manual download path and observed page state.
3. Use CBO data as cross-check or context unless a later source review promotes
   the source status.

## Review Gates

Before a dynamic query export is used in public docs:

1. Source Custodian confirms query reproducibility and checksum.
2. Budget Accountant confirms period, basis, and budget concept semantics.
3. The record status is promoted from `captured` or `draft-extracted`.
4. Public wording states the query date or reporting period.
