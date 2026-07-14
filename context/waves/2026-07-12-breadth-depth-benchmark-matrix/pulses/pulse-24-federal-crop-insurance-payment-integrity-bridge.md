# Pulse 24: Federal Crop Insurance Payment-Integrity Bridge

## Result

Captured checksum-fixed official custody for the Q4 2025 FCIC scorecard, RMA
COM-23-001, and the 2022 Standard Reinsurance Agreement, then reconciled them
to the existing FY2024 PaymentAccuracy annual-workbook custody.

The annual row reports $23,867.31M in covered outlays and $579.36M in improper
payments. Its complete payment-type split is $573.93M in overpayments, $5.43M
in underpayments, and zero technically improper or unknown payments. The
scorecard's $574M and 2.40% overpayment result is the rounded presentation of
the annual row.

COM-23-001 identifies a statistically valid sample of 326 policies from the
2022 reinsurance year for the FY2024 reporting period. The SRA definition maps
that year to July 2021 through June 2022, exactly matching the annual workbook
and scorecard.

## Decision Gate

Pass for internal closure of `sample period` and `payment type split`. Six
methodology fields remain open: sample design, payment universe, estimation
method, exclusion rules, the complete data-access outside-agency-control
root-cause definition, and recoverable-savings basis.

Fail for scoring the program or converting the statistical estimate or its
root-cause categories into fraud, waste, identified debt, collected recovery,
preventable loss, or net savings. The scorecard describes a recovery process
but publishes no amount-level lineage from the FY2024 estimate to collections.

## Custody

- `SRC-OMB-PAYMENTACCURACY-FY2024-DATA`: existing annual workbook custody
- `SRC-OMB-PAYMENTACCURACY-FCIC-Q4-2025`: checksum-fixed scorecard PDF
- `SRC-USDA-RMA-COM-23-001`: checksum-fixed official PDF rendition
- `SRC-USDA-RMA-SRA-2022`: checksum-fixed agreement PDF

## Integration Status

Complete. The custody and derived bridge now flow through the shared
methodology chain, agriculture and payment-integrity depth cards, breadth
matrix, scoreboard, WAVE, source ledger, manifest, and Rust validator.

## Next Bounded Action

Seek current official sample-frame, estimator, exclusion, and amount-level
recovery evidence without weakening the six remaining gates or allowing a
program score.
