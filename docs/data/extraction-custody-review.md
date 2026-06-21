# Extraction Custody Review

## Purpose

This review decides whether TAXLANE is ready to import raw source artifacts for
rates, receipts, outlays, and lane crosswalk work. It applies two roles before
any spreadsheet, PDF extraction, or query export is committed:

- Source Custodian: verifies source identity, versioning, storage, and update
  rules.
- Budget Accountant: verifies year basis, budget concepts, reconciliation, and
  public wording guardrails.

## Scope

Covered source families:

| Family | Primary schema | Initial source set | Decision |
|---|---|---|---|
| Historical income-tax rates | `rates-timeline-schema.md` | `SRC-IRS-SOI-HT23` | Approved for controlled extraction after fetch metadata is recorded. |
| Receipts and fund groups | `receipts-funds-schema.md` | OMB Historical Tables 1.1, 1.4, 2.1, 2.2, 2.4; OMB budget concept chapters | Approved for controlled extraction after OMB table release and row labels are preserved. |
| Outlays and lane crosswalks | `outlays-lanes-schema.md` | OMB Historical Tables 1.1, 3.1, 3.2, 4.1, 8.5, 8.7, 11.3; USAspending context | Approved for controlled extraction of OMB tables. USAspending remains query-review only. |
| Dynamic current-period context | `dictionary.md` plus source-specific notes | Treasury Monthly Treasury Statement, USAspending, CBO candidate data | Deferred until query snapshot rules exist. |

## Source Custodian Review

### Findings

- Source IDs exist in `docs/sources/source-version-ledger.md` for the initial
  IRS and OMB sources.
- Each source row records publisher, URL, observed date, coverage, cadence,
  status, and extraction rule.
- Dynamic sources are identified separately from annual/static sources.
- CBO budget data is marked candidate-only because automated fetch was blocked.
- The data directory currently defines schemas only and contains no raw source
  artifacts.

### Required raw-artifact rules

Before committing any raw artifact:

1. Store it under a future source-specific raw area, not beside narrative docs.
2. Preserve the publisher filename when possible.
3. Add a sidecar metadata record with source ID, URL, observed date, fetch
   method, checksum, and any manual steps.
4. Do not overwrite a prior source version. Add a new versioned artifact or mark
   the older one superseded.
5. Do not commit dynamic query exports unless the query parameters, run date,
   response scope, and source caveats are recorded.

### Decision

Approved for first controlled extraction of IRS SOI Table 23 and OMB FY2027
Historical Tables. Deferred for dynamic USAspending, Treasury, and candidate CBO
data until query custody rules are written.

## Budget Accountant Review

### Findings

- `rates_timeline` uses tax-year semantics.
- `receipt_source`, `fund_group`, `outlay_function`, and `outlay_program` use
  fiscal-year semantics.
- Receipt records must keep individual income taxes, corporate income taxes,
  and social-insurance receipts separate.
- Fund records require budget-concept support before any legal dedication,
  trust-fund, offsetting, or general-fund label is used publicly.
- Outlay records must reconcile OMB function/subfunction totals before
  lane-level public claims.
- Lane crosswalk records require included categories, excluded categories,
  allowed allocation methods, and deficit context.

### Required accounting gates

Before marking extracted records `reviewed`:

1. Confirm every source ID exists in the ledger.
2. Confirm tax-year versus fiscal-year basis for every row.
3. Preserve exact source row labels.
4. Separate actuals, estimates, and projections.
5. Reconcile top-level receipts and outlays to OMB Table 1.1 where applicable.
6. Reconcile subfunction and program details to their parent function only when
   the source supports that comparison.
7. Keep net interest and undistributed offsetting receipts visible.
8. Require an allocation method and deficit context before publishing taxpayer
   receipt views.

### Decision

Approved for first controlled extraction of schema-backed rate, receipt, fund,
outlay, and lane crosswalk records. Public taxpayer allocation remains blocked
until extracted records pass reconciliation and lane review.

## First Import Checklist

Use this checklist for the next data pulse:

1. Select one source family and one source release.
2. Confirm the source ID and schema file.
3. Fetch or manually save the source artifact.
4. Record sidecar metadata and checksum.
5. Extract only the fields defined by the schema.
6. Run family-specific reconciliation checks.
7. Run `git diff --check`.
8. Commit raw custody metadata and extracted draft records separately if both
   are needed.

## Open Follow-Ups

- Define the future raw-artifact directory shape before the first downloaded
  spreadsheet is committed.
- Define query snapshot rules for Treasury MTS and USAspending.
- Decide whether extracted draft records should live as CSV, JSONL, or another
  structured format.
