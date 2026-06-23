# Pulse 01

## Objective

Prevent empty partial evidence replies from passing intake validation.

## Done

- Added evidence-required and evidence-forbidden helpers to `PerformanceDemandResponseClass`.
- Rejected complete or partial evidence response intake rows without `evidence_received`.
- Kept process-only and no-evidence intake rows from listing `evidence_received`.
- Updated the generated intake schema gate rules.
- Added review and VTRACE rows for `WP-TAX-049`.
