# Pell Current-Entrant Persistence Significance Screen

Machine record:
`data/derived/breadth_benchmark_matrix/pell_current_entrant_persistence_significance_screen.bps2020-2022.v1.draft.json`.

This screen applies the official NCES DataLab **Independent Estimates t-Test**
to the five Pell-receipt group comparisons preserved by query `396385`,
retrieval code `zclxfu`. For each `PROUT3_NEW` category, it uses the exact
weighted percentage and BRR standard error from the saved table, computes

`t = (estimate with no Pell - estimate with Pell) / sqrt(SE no Pell squared + SE Pell squared)`,

and preserves the p-value returned by DataLab's dataset-168, `WTA000`
`workspace/process/pvalue` endpoint.

## Independent-Estimates Results

| Three-year status | Difference, no Pell minus Pell | t | DataLab p-value | Nominal .05 | Bonferroni .01 |
|---|---:|---:|---:|---|---|
| Bachelor's attained | +0.3179366 pp | 2.428849 | 0.0160308 | Pass | Fail |
| Associate's attained | +0.6583063 pp | 1.337487 | 0.1825827 | Fail | Fail |
| Certificate attained | -3.6876000 pp | -8.665866 | 1.5193e-15 | Pass | Pass |
| No degree; enrolled in 2021-22 | +12.4041402 pp | 12.484295 | 7.9949e-27 | Pass | Pass |
| No degree; not enrolled in 2021-22 | -9.6927832 pp | -10.558216 | 5.3800e-21 | Pass | Pass |

“Pass” means only that the independent-estimates test crosses the stated
threshold. It is not a causal, practical-importance, program-performance, or
policy-value judgment. Because the screen makes five category comparisons, the
machine record also preserves a conservative Bonferroni familywise threshold of
0.01 and adjusted p-values. DataLab returned the unadjusted p-values; the
Bonferroni calculation is TAXLANE-derived.

## Independence And Covariance Boundary

DataLab states that this tool is valid only for independent groups. Its client
formula adds the two squared standard errors and does **not** include a
covariance term. `PELL20 = 0` and `PELL20 > 0` are mutually exclusive receipt
groups, so this is the official DataLab independent-estimates screen available
from the published table. It does not satisfy the stricter gate for a
replicate-weight or otherwise covariance-aware difference estimate. That gate
remains blocked and must not be silently relabeled as closed.

## Evidence Boundary

Positive `PELL20` records Pell receipt in academic year 2019-20, not
eligibility. Zero dollars does not establish ineligibility, nonapplication,
denial, or a valid untreated counterfactual. The groups are observational and
unadjusted. Statistical differences between them are not effects caused by
Pell receipt.

The five-category `PROUT3_NEW` result is not the six-category First Look table.
The cohort entered during the pandemic disruption window, many students were
still enrolled, and “no degree, not enrolled” is not permanent dropout. These
are early three-year statuses, not mature completion or labor-market outcomes.

The screen contains no full incremental program cost, compatible fiscal
allocation, return estimate, fraud finding, improper-payment estimate,
recovery, or budget saving. Mature outcomes remain gated on BPS:20/25; the
covariance-aware significance gate remains separately blocked.

The five exact endpoint responses and their request manifest are fixed under
`data/raw/nces/SRC-NCES-DATALAB-BPS20-22-PELL-PERSISTENCE-TTEST-2026/2026-07-13/`.
The underlying public table is [NCES DataLab query zclxfu](https://nces.ed.gov/datalab/powerstats/table/zclxfu).
