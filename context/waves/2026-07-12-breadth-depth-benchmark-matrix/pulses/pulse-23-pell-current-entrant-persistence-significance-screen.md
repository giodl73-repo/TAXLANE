# Pulse 23: Pell Current-Entrant Persistence Significance Screen

## Result

Captured five exact official NCES DataLab p-value endpoint responses and a
request manifest for the independent-estimates comparisons implied by the
Pulse 22 BPS:20/22 Pell-receipt cross-tab. The calculation uses the full-
precision estimates and BRR standard errors from saved query `396385`,
retrieval code `zclxfu`, rather than values rounded for display.

A display-rounding sensitivity check using DataLab's one-decimal estimates and
two-decimal standard errors leaves every nominal-alpha and Bonferroni decision
unchanged. The custody responses nevertheless use the full-precision t values;
the rounded sensitivity values do not replace them.

At nominal two-sided alpha 0.05, the bachelor's, certificate, enrolled-without-
degree, and not-enrolled-without-degree comparisons pass the independent-
estimates screen; the associate's comparison does not. With a conservative
five-comparison Bonferroni threshold of 0.01, the bachelor's comparison no
longer passes. The certificate and both no-degree enrollment-status
comparisons remain below the adjusted threshold.

## Decision Gate

Pass for preserving the official DataLab independent-estimates calculation and
dataset- and weight-specific p-values, with unadjusted and derived Bonferroni
results kept distinct. Fail for claiming a covariance-aware or replicate-
weight difference test: DataLab's tool explicitly assumes independent groups
and uses no covariance term.

Fail for a Pell eligibility contrast, Pell program effect, practical-importance
judgment, adjusted counterfactual, mature completion or permanent dropout,
incremental cost, compatible fiscal return, fraud, improper payment, recovery,
or savings claim. Statistical significance does not remove any of those
boundaries.

## Custody And Reproducibility

- Source ID: `SRC-NCES-DATALAB-BPS20-22-PELL-PERSISTENCE-TTEST-2026`
- Dataset ID: `168`
- Weight: `WTA000`
- Source query ID: `396385`
- Source retrieval code: `zclxfu`
- Comparison count: `5`
- Formula: `(E_no_Pell - E_Pell) / sqrt(SE_no_Pell^2 + SE_Pell^2)`
- Official p-value endpoint: `workspace/process/pvalue`
- Request manifest: `data/raw/nces/SRC-NCES-DATALAB-BPS20-22-PELL-PERSISTENCE-TTEST-2026/2026-07-13/request-manifest.json`
- Method boundary: independent estimates; no covariance term
- Multiplicity boundary: DataLab p-values are unadjusted; Bonferroni values are derived

## Integration Status

Integration complete. The standalone screen remains the canonical home for the
five result rows and now links through the BPS bridge, current-entrant baseline,
education depth card, higher-education bridge, adjacent Pell evidence, breadth
row, public scoreboard, WAVE, source ledger, Rust catalog and validator, and
generated manifest. The validator reconciles each derived result to its exact
request-manifest row and checksum-fixed DataLab response.

## Next Bounded Action

Preserve a replicate-weight or otherwise covariance-aware difference test
before replacing this independent-estimates screen. Capture BPS:20/25 before
treating the cohort as a mature completion or labor-market baseline. Retain
receipt-not-eligibility, five-versus-six-category, observational, pandemic-era,
noncausal, cost, fiscal, fraud, and savings boundaries.
