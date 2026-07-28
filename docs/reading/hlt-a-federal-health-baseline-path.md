# HLT-A Federal Health Baseline Path

Machine record:
`data/derived/breadth_benchmark_matrix/hlt_a_federal_health_baseline_path.v1.draft.json`

HLT-A-02 is complete as a bounded horizon record. One captured OMB FY2027
public-budget-database vintage supplies exact function 550 and 570 outlays for
FY2025-FY2031. Each combined health/Medicare row recomputes from those two
components.

The source contains no function rows for FY2032-FY2035, so all three amount
fields remain null in those years. CBO major categories are not substituted,
and no interpolation or mixed-vintage stitching is used.

HLT-A-03 financing lineage has since completed, handing off to active HLT-A-04.
This path does not add state, local, or
private spending; admit reform effects; complete a numeric eleven-year path;
run a solver; publish a rate or savings; close HLT-A; or start HLT-B.
