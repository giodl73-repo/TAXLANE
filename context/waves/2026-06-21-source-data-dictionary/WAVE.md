# Wave: Source Data Dictionary

## Goal

Define the source-backed data dictionaries TAXLANE needs before importing,
extracting, or publishing rates, receipts, outlays, and lane crosswalk records.

## Thesis

TAXLANE should make public tax claims from reproducible records, not narrative
memory. Before raw spreadsheets or statutory tables enter the repo, each record
family needs source IDs, year semantics, extraction rules, allocation labels,
and validation gates.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|---|---|---|
| 01 | Data dictionary scaffold | done | Created canonical record families, fields, and extraction rules for rates, receipts, outlays, and lanes. |
| 02 | Rates timeline schema | done | Defined tax-year rate-row identity, controlled values, extraction rules, validation checks, and public wording rules. |
| 03 | Receipts and fund schema | pending | Define fiscal-year receipt-source and fund-group records from OMB historical tables. |
| 04 | Outlays and public-purpose schema | pending | Define OMB function/subfunction/agency/program records and TAXLANE lane crosswalk fields. |
| 05 | Extraction custody review | pending | Apply Source Custodian and Budget Accountant roles before any raw-source import. |

## Success criteria

- Data dictionaries exist before any raw source artifact is committed.
- Every record family declares tax-year versus fiscal-year semantics.
- Every public allocation record includes an allocation method.
- Extraction rules point to source-ledger IDs.
- Validation commands pass.
