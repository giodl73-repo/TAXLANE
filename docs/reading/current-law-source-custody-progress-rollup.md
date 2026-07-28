# Current-Law Source Custody Progress Rollup

Machine record:
`data/derived/breadth_benchmark_matrix/current_law_source_custody_progress_rollup.v1.draft.json`.

This rollup summarizes partial custody progress across the eight required
current-law paths. It does not replace the older batch plan. The batch plan
still describes the original future work order; this rollup records that later
validated artifacts now provide partial custody or boundary evidence for each
path.

The data-acquisition eight-gap status is linked as acquisition-status evidence,
not as complete source custody. It records the latest acquisition pass and
preserves the remaining CBO, OMB, general-fund, Medicare HI, receipt-base,
transportation, net-interest, solver, rate, savings, and balanced-budget
blockers.

## What Changed

The eight required current-law paths now all have partial custody progress:

| Path | Current state |
|---|---|
| FY2025 17-row ledger | FY2025 baseline-year custody, FY2025-FY2031 17-row PBD outlay context, and OMB PBD User's Guide horizon-boundary custody exist, but FY2032-FY2035, fund split, and solver binding remain missing. |
| Baseline plus ten-year horizon | FY2025-FY2031 OMB outlay, receipt, and deficit context exists, CBO open data now supplies FY2032-FY2035 top-line/category context, and CBO revenue detail supplies FY2026-FY2035 receipt-category context; unified OMB/CBO reconciliation, fund split, matched receipt bases, lane row allocation, and solver binding remain missing. |
| General fund path | FY2025 OMB fund-group context and Treasury MTS Table 8 federal-fund context exist; federal-fund/general-fund source-boundary reconciliation, annual general-fund path, and transfers remain missing. |
| OASDI fund path | FY2025 receipt/outlay anchors, a combined OASDI FY2025-FY2035 fiscal-year context path, and CBO OASI/DI balance context exist; separate solver-ready OASI/DI paths, complete post-depletion fields, explicit transfers, calendar-to-fiscal receipt bridge, and solver inputs remain missing. |
| Medicare HI fund path | FY2025 receipt/outlay anchors, Treasury MTS Tables 4 and 5 FY2025 HI receipt/outlay anchor context, local CMS Trustees custody, CY2025-CY2035 HI current-law context, and CBO FY2032-FY2035 HI balance context exist; fiscal-year HI path, OMB/CMS timing and receipt-row bridge, trust-fund solver-yield mapping, rate bridge, and solver input remain missing. |
| Transportation trust-fund path | Source custody, accounting boundary, OMB Table 13-4 FY2025-FY2031 Highway Trust Fund and Airport and Airway Trust Fund context values, Treasury MTS Tables 4 and 5 FY2025 transportation receipt/outlay anchor context, and CBO FY2032-FY2035 balance context exist; income/outgo reconciliation, explicit transfers, credited collections, Function 400 mapping, and solver input remain missing. |
| Health fiscal current-law path | FY2025 OMB/CMS context, February 2026 CBO health-insurance PDF/spreadsheet raw custody and Table 2 rowmap context, and CBO FY2026-FY2035 major health-category context exist; fiscal-year HI path, SMI split path, non-Medicare health path, OMB/CBO/CMS reconciliation, and solver inputs remain missing. |
| Net-interest current-law path | OMB PBD FY2025-FY2031 net-interest context, CBO FY2032-FY2035 net-interest/debt context, Treasury latest-month average-interest-rate context, and formula/floor-definition boundaries exist; OMB PBD FY2032-FY2035 row mapping, debt-stock reconciliation, maturity schedule, fiscal-year rate path, feedback, and solver inputs remain missing. |

This is partial custody progress, not complete path readiness.

CBO major outlay-category context now adds FY2026-FY2035 category values for
defense, Social Security, Medicare, Medicaid, income security, SNAP, family
support, child nutrition, veterans, agriculture, transportation-related
expired-authority rows, higher education, and administration of justice. Those
values are context only; they do not replace the OMB 17-row lane ledger or open
solver inputs.

Treasury MTS Table 8 now adds FY2025 federal-fund and trust-fund context. Those
values are context only; federal funds are broader than the general fund and do
not create a general-fund annual path.

Treasury MTS Tables 4 and 5 now add FY2025 Medicare HI receipt/outlay anchor
context. Those values are fiscal-year anchors only; they do not create a
calendar-to-fiscal conversion, FY2025-FY2035 Medicare HI fiscal-year path,
income-category crosswalk, solver input, or rate bridge.

Treasury MTS Tables 4 and 5 now also add FY2025 transportation trust-fund
receipt/outlay anchor context. Those values do not create transportation
trust-fund income/outgo reconciliation, fund-balance reconciliation, Function
400 mapping, solver input, or rate bridge.

OMB PBD User's Guide FY2027 custody now documents the three Public Budget
Database data files and the FY2031 file horizon. It does not create
FY2032-FY2035 OMB 17-row ledger values or solver input.

CBO revenue-detail context now adds FY2026-FY2035 receipt-category values for
the revenue-solvency overlay. Those values are context only; they are not
matched legal/economic bases, incidence or distribution models, current-law or
reform yields, rate bridges, or solver inputs.

CBO February 2026 health-insurance PDF/spreadsheet raw custody and Table 2
rowmap context now support health baseline source custody for that publication.
Those files are context only; they do not create a current-law health solver
path, policy score, rate calculation, savings estimate, or balanced-budget
claim.

Treasury average-interest-rate context now adds latest-month rate evidence for
net interest. It is not a fiscal-year rate path, CBO/OMB projection bridge, or
solver input.

## Boundary

There are eight required current-law paths in this rollup. Zero are complete.
The rollup is not a complete FY2025-FY2035 baseline, not solver-ready, not rate-ready, not savings-ready, and not balanced-budget-ready.

No solver input, solver run, target cost, statutory rate, effective rate, public
rate card, savings estimate, waste finding, fraud finding, department-cut
instruction, technology-savings claim, or balanced-budget claim is published.
