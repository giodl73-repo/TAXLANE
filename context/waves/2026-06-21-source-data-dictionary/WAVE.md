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
| 03 | Receipts and fund schema | done | Defined fiscal-year receipt-source and fund-group identity, controlled values, extraction rules, reconciliation checks, and public wording rules. |
| 04 | Outlays and public-purpose schema | done | Defined OMB function/subfunction/program identity, lane crosswalk fields, reconciliation checks, and public wording rules. |
| 05 | Extraction custody review | done | Applied Source Custodian and Budget Accountant gates, approved first controlled OMB/IRS extraction, and deferred dynamic query sources. |

## Success criteria

- Data dictionaries exist before any raw source artifact is committed.
- Every record family declares tax-year versus fiscal-year semantics.
- Every public allocation record includes an allocation method.
- Extraction rules point to source-ledger IDs.
- Validation commands pass.
