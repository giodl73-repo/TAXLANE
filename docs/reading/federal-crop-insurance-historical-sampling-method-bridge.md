# Federal Crop Insurance Historical Sampling-Method Bridge

Machine record:
`data/derived/breadth_benchmark_matrix/federal_crop_insurance_historical_sampling_method_bridge.fy2020.v1.draft.json`.

This bridge closes one historical internal component: the policy-selection
method and payment-category coverage disclosed for FY2020, using reinsurance
year 2018. It does not close any current FY2024 methodology field.

## Historical Disclosure

Under `Sampling and Estimation` on printed page 66, the FCIC/RMA FY2020
financial report says RMA used a simple random sample to select policies. It
also says the improper-payment reviews included premium subsidies,
administrative-and-operating subsidies, and indemnity payments, and that RMA
used statistically valid estimates of both the improper-payment rate and
dollar amount.

The same page says OMB had approved RMA's statistically valid sampling
methodology for FY2017 and beyond and identifies reinsurance year 2018 as the
FY2020 sampling period. Printed page 67 identifies the related AIP reinsurance
agreement period as July 2017 through June 2018.

## Narrow Decision

The source supports an internal-only historical component closure for the
FY2020/RY2018 disclosure: simple-random policy selection, the three named
payment categories, statistically valid rate and dollar estimates, and the
reported OMB approval posture.

It does not support continuity to FY2024. The current `sample design`,
`estimation method`, and `exclusion rules` fields all remain open, as does
`recoverable savings basis`. The program aggregate therefore remains four
closed fields and four open fields.

## Residual Boundary

Current-period evidence is still required for the sample frame, allocation,
selection probabilities and implementation, replacement, nonresponse,
incomplete-document treatment, weights, rate estimator, dollar projection,
variance and confidence interval, and exclusions below the three broad
payment categories.

The cited section is labeled `Other Information (Unaudited)`. Its language
documents the historical method; it does not independently validate the
implementation or make the current method publicly reproducible.

No program score or public claim about fraud, waste, debt, recovery,
prevention, or savings is permitted by this historical component closure.

## Custody

- Source: `SRC-USDA-OIG-FCIC-RMA-FS-FY2020`
- Official URL: `https://usdaoig.oversight.gov/sites/default/files/reports/2024-11/05401-0012-11FRFOIA.pdf`
- Raw path: `data/raw/usda/SRC-USDA-OIG-FCIC-RMA-FS-FY2020/2026-07-13/05401-0012-11FRFOIA.pdf`
- Bytes: `13696922`
- SHA-256: `55FD128F191C3D0892F819F35A92929EFB02DBD7354626D3C844A84C3253AC4B`
- Evidence: printed page 66 (PDF file page 93; zero-based viewer index 92), continuing onto printed page 67 (PDF file page 94; zero-based viewer index 93)
