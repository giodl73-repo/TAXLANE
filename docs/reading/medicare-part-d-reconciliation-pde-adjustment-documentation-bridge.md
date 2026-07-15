# Medicare Part D Reconciliation-PDE Adjustment Documentation Bridge

Machine record:
`data/derived/breadth_benchmark_matrix/medicare_part_d_reconciliation_pde_adjustment_documentation_bridge.cy2022.v1.draft.json`

## Result

One narrow payment-universe component closes internally. When a sponsor adjusts
a sampled reconciliation PDE after the June 29, 2023 reconciliation cutoff,
CMS requires two documentation tracks: documentation aligned to the sampled
reconciliation PDE remains required, and the sponsor must additionally submit
linked documentation showing the later adjustment.

The cutoff and sponsor-designated final reconciliation-PDE review target are
prior Pulse 39 and Pulse 43 context. Pulse 46 closes only the two-track
after-adjustment documentation treatment.

## Source Custody

The sole source is the official CMS CY2022 Part D IPM Submission Instructions,
captured locally at 5,962,810 bytes across 40 PDF file pages with SHA-256
`52A76C9910BB66EDD387D127744F864D78E59826BC5FF0162BC81EDE428C7199`.
Appendix A is printed page 36, PDF file page 39.

## Residual

The guide does not say whether an adjusted PDE is included, excluded, replaced,
reweighted, or assigned a different error in the national estimator. It does
not disclose denominator or payment effects, negative adjustments, reversals,
deletions, rejected or duplicate PDEs, counts, projection, simulation, variance,
or the mapping from GDC and combined liability to federal outlays.

Full payment universe remains open. Part D remains three fields closed and five
open, with three closure decisions and five residual gaps.

## Boundary

Operational documentation treatment is not estimator treatment. A Missing
Documentation Form is not proof of inclusion or exclusion, and a sampled
reconciliation PDE is not the complete Part D payment universe. This bridge
does not establish a payment effect, debt, collectibility, recovery, fraud,
waste, prevention, or savings. All claim and scoring gates remain false.
