# Pulse 32: Federal Crop Insurance Public Cohort Outcome Evidence Ceiling

## Result

Captured checksum-verified official custody of the September 25 and November
21, 2024 RMA Manager reports. They establish a public reporting transition:
after the August report completed the FY2024/Reinsurance Year 2022 sample and
reported a 2.43 percent rate, the later reports move Payment Integrity
Information Act reporting to the 388-policy FY2025/Reinsurance Year 2023
cohort.

The September report says 75 policies had closed with no improper payments
identified. The November report says 237 reviews had been completed and the
case policies closed, with Initial Findings due to AIPs by December 20, 2024.
Neither report retrospectively publishes FY2024 sample-specific Final Finding,
final-administrative-determination, debt, appeal, setoff, or collection data.

## Decision Gate

Pass for a narrow public reporting-continuity and search-ceiling component.

Fail for any new full methodology-field closure. The later Manager reports
switch cohorts rather than supply the missing FY2024 outcome lineage. FCIC
remains four closed and four open.

Fail for combining ordinary compliance findings with either payment-integrity
sample. The reports place those amounts under a separate Compliance heading
and do not attribute them to the 326-policy FY2024 cohort or the 388-policy
FY2025 cohort.

Fail for program scoring or public claims about fraud, debt, collectibility,
recovery, prevention, control cost, or savings.

## Custody

- September source: `SRC-USDA-RMA-FCIC-MANAGER-2024-09-25`
- September raw path: `data/raw/usda/SRC-USDA-RMA-FCIC-MANAGER-2024-09-25/2026-07-14/092524managers.pdf`
- September bytes: `105445`
- September SHA-256: `ACD35B47587751305F1199E675B715CCB5C074E2AD88D4DDB9D1D1E148EE9D22`
- September evidence: PDF file page 2
- November source: `SRC-USDA-RMA-FCIC-MANAGER-2024-11-21`
- November raw path: `data/raw/usda/SRC-USDA-RMA-FCIC-MANAGER-2024-11-21/2026-07-14/112124managers.pdf`
- November bytes: `144513`
- November SHA-256: `F581F11FB99A690743894BA932FC6E460F57E82B3FFC68E2952169523947E443`
- November evidence: PDF file page 3

## Integration Status

Custody, metadata, bridge, reader, depth card, ledger, READMEs, Rust validator,
and manifest are integrated. This pulse closes zero new full fields; FCIC
remains four closed and four open with all score and claim gates blocked.

## Next Bounded Action

Seek an authorized CARS/AIP export or a focused FOIA response providing a
deidentified row-level or aggregate table that preserves the FY2024/Reinsurance
Year 2022 cohort and reports outcome states and amounts. Do not repeat the
Manager-report search or combine ordinary compliance findings with sample
outcomes.
