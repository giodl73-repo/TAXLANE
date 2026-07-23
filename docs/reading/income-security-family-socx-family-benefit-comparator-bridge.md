# Income-security/family SOCX family-benefit comparator bridge

This packet explains
`data/derived/breadth_benchmark_matrix/income_security_family_socx_family_benefit_comparator_bridge.v1.draft.json`.

Pulse 197 reuses the already source-custodied OECD SOCX old-age/family panel for
the income-security/family lane. The source is
`SRC-OECD-SOCX-OLDAGE-FAMILY-PANEL-2022`; the raw file is
`data/raw/oecd/SRC-OECD-SOCX-OLDAGE-FAMILY-PANEL-2022/2026-07-15/oecd-socx-oldage-family-panel-2022.csv`,
with 4,334 bytes and SHA-256
`0f138dc4e1dd3424890357cdbf4610645dd1d00bd3848d19509fe24860e8c253`.

The context value carried forward for the United States is 0.658 percent of GDP
for public family benefits in 2022, split into 0.051 percent cash and 0.607
percent in-kind services. The bounded SOCX panel has observed family rows for
seven countries and explicit missingness for Germany, Sweden, the Netherlands,
Poland, and Singapore.

This bridge closes SOCX family-benefit comparator context only. The exact SOCX
response does not include tax breaks for social purposes, and it does not carry
childcare participation, ESSPROS, ILO, same-year child-outcome linkage, or
missing-country replacement lineage.

This bridge is not complete international comparator lineage, not tax-credit
composition, not childcare participation context, not ESSPROS context, not ILO
context, not child-outcome linkage, not target-cost selection, not gross
savings, not net savings, not solver input, not rate calculation, not a public
rate card, not a department-cut instruction, not a technology-savings claim, and
not a balanced-budget claim.

Short validator phrases: SOCX family-benefit comparator context only; not
tax-credit composition; not solver input; not a balanced-budget claim.

Exact validator phrase: not tax-credit composition.
