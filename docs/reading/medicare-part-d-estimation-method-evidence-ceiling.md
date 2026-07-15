# Medicare Part D Estimation-Method Evidence Ceiling

Machine record:
`data/derived/breadth_benchmark_matrix/medicare_part_d_estimation_method_evidence_ceiling.fy2024.v1.draft.json`.

## Verified observations and custody status

Web extraction of the official
[FY 2024 HHS Annual Performance Report](https://www.hhs.gov/sites/default/files/hhs-fy-2024-agency-performance-report.pdf)
verifies the Part D text at printed page 88, PDF page index 87, extraction lines
3331–3338: “Based on these reviews, each PDE in the audit sample is assigned a
gross drug cost error.” It then states: “A representative sample of
beneficiaries undergoes a simulation to determine the Part D improper payment
estimate.”

This is not official source custody. On July 14, 2026, the HHS edge returned
Akamai HTTP 403 responses to scripted requests, and a bounded headless-browser
attempt did not yield the PDF. No source ID or metadata was assigned, no raw PDF
was recorded, and no access-denied response was retained.

The checksum-verified CMS FY2024 findings remain the captured same-period
source. They attest OMB sampling-and-estimation-plan compliance and statistical
validity; publish gross, net, rate, denominator, and 95% confidence-limit
outputs; and define overpayment and underpayment direction from corrected
versus reported gross drug cost. They do not publish the estimator.

The CMS Program Background page modified January 15, 2026 describes a random
5% beneficiary sample and extrapolation to remaining beneficiary payments. It
is current, non-same-period corroboration only. It cannot establish that those
mechanics governed CY2022/FY2024.

## Evidence ceiling and exact residuals

The combined record supports observations about PDE error assignment,
beneficiary simulation, statistical governance, published outputs, and gross
drug cost direction. It closes neither a component nor the full estimation-
method field. The remaining needs are:

- the dollar and rate estimator formulas;
- weights and calibration, normalization, trimming, or nonresponse adjustment;
- aggregation from PDE errors to beneficiary simulated errors;
- benefit-phase, reinsurance, low-income-subsidy, and payment-parameter
  simulation mechanics;
- linkage or nesting between the PDE and beneficiary samples;
- estimator treatment of overpayments, underpayments, missing documents, and
  zero-error records;
- variance estimation, finite-population treatment, and confidence-limit
  construction;
- same-period confirmation of any 5% sample and national extrapolation; and
- unrounded-to-published reconciliation and rounding rules.

## Retry contract and claim firewall

Retry the exact official HHS URL when binary access is available. Alternatively,
accept a user-provided PDF only when the user identifies it as downloaded from
that exact URL. Before promotion, verify the PDF signature, bytes, SHA-256,
pages, and printed-page-88 text, then create stable source metadata. Do not
retain an HTML denial page, substitute a third-party mirror as official custody,
or import estimator details from the 2026 background page.

Even successful custody would authenticate the process text, not automatically
close an estimation component or field. Medicare Part D remains three fields
closed and five open. All scoring, public-claim, fraud, waste, debt, recovery,
and savings gates remain blocked.
