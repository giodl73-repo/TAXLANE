# Pulse 41: Medicare Part D Estimation-Method Evidence Ceiling

## Objective

Record the strongest FY2024 estimation-process evidence and its custody limit
without converting process observations into estimator closure.

## Evidence and custody result

Web extraction of the official HHS FY2024 Annual Performance Report verifies
the Part D passage at printed page 88, PDF page index 87, extraction lines
3331–3338: “Based on these reviews, each PDE in the audit sample is assigned a
gross drug cost error.” It then states: “A representative sample of
beneficiaries undergoes a simulation to determine the Part D improper payment
estimate.”

Official bytes could not be captured on July 14, 2026. The HHS edge returned
Akamai HTTP 403 to scripted requests, and a bounded headless-browser attempt did
not yield the PDF. No source custody, raw PDF, source ID, checksum, or metadata
is claimed.

The captured CMS FY2024 findings provide the same-period OMB plan-compliance
and statistical-validity attestation, gross and net estimates, rate,
denominator, confidence limits, and corrected-versus-reported gross drug cost
direction. The CMS background page modified in 2026 describes a 5% beneficiary
sample and extrapolation, but it is non-same-period corroboration only.

## Decision

Record a custody-blocked evidence ceiling with zero component and zero full-
field closures. The sources do not disclose the estimator formula, weights,
PDE-to-beneficiary aggregation or sample linkage, benefit-parameter simulation,
missing-document and zero-error treatment, variance method, same-period 5%
design, or rounding and reconciliation mechanics.

## Retry contract and boundary

Retry the exact official HHS URL, or accept a user-provided PDF identified as
downloaded from that URL. Verify signature, bytes, SHA-256, pages, and the
printed-page-88 text before creating metadata. Never retain an HTML denial page,
claim a third-party mirror as official custody, or use the 2026 background page
to fill CY2022/FY2024 estimator gaps.

Successful custody would authenticate the process passage but would not by
itself close an estimation component or field. Part D remains three closed and
five open. Every methodology-scoring, public-claim, fraud, waste, debt,
recovery, and savings gate remains blocked.
