# Current-Law Source Custody Wave B Closure

Machine record:
`data/derived/breadth_benchmark_matrix/current_law_source_custody_wave_b_closure.v1.draft.json`.

Wave B is done in the bounded Taxlane sense: the existing captured official
sources have been reconciled across the current-law source-custody batches, and
the remaining unsupported values are explicitly carried forward as nulls. Wave
B is closed for existing captured official sources. It is not a claim that all
future official sources have been acquired.

The data-acquisition eight-gap status is linked as the latest acquisition-status packet, not as complete source custody.

## Closed Batches

| Batch | What is now supported | What remains blocked |
|---|---|---|
| Federal baseline and 17-row ledger | FY2025 17-row ledger, FY2025-FY2031 17-row PBD outlay context, OMB PBD User's Guide FY2027 horizon-boundary custody, FY2025-FY2031 aggregate outlays/receipts/deficit, CBO FY2032-FY2035 top-line context, CBO FY2026-FY2035 major outlay-category context, CBO FY2026-FY2035 revenue-detail context, FY2025 OMB fund-group context, and FY2025 Treasury MTS Table 8 federal-fund context. | FY2032-FY2035 OMB PBD/Historical 17-row annual values, unified OMB/CBO reconciliation, fund split, matched receipt bases, annual general-fund path, federal-fund/general-fund source-boundary reconciliation, explicit transfers, solver binding. |
| OASDI, Medicare HI, and transportation trust funds | FY2025 dedicated receipt and named trust-fund outlay anchors, Treasury MTS Tables 4 and 5 FY2025 Medicare HI receipt/outlay anchor context, transportation trust-fund custody/accounting boundaries, OMB Table 13-4 FY2025-FY2031 transportation trust-fund context values, Treasury MTS Tables 4 and 5 FY2025 transportation trust-fund receipt/outlay anchor context, and CBO FY2032-FY2035 OASI/DI, Medicare HI, Highway, and Airport/Airway trust-fund balance context. | Solver-ready annual OASDI and Medicare HI paths, calendar-to-fiscal Medicare HI conversion, transportation trust-fund income/outgo reconciliation, explicit transfers, credited offsetting collections, and fund-balance reconciliation. |
| Health current-law components | CMS Medicare Trustees CY2025 financing/enrollment context, Medicare HI CY2025-CY2035 current-law context, Treasury MTS Tables 4 and 5 FY2025 Medicare HI receipt/outlay anchor context, CMS NHE raw context custody, February 2026 CBO health-insurance PDF/spreadsheet raw custody and Table 2 rowmap context, CBO health latest-publication access-boundary evidence, partial CMS Provider Data Catalog hospital quality/access context custody, CBO FY2026-FY2035 major health-category context, and Medicare HI bridge blockers. | Fiscal-year Medicare HI path, SMI split path, non-Medicare health path, July 2026 CBO latest-publication local raw custody, complete quality/access lineage, OMB/CBO/CMS reconciliation, and health solver row. |
| Net interest and debt | OMB PBD FY2025-FY2031 net-interest context, CBO FY2032-FY2035 net-interest/debt context with local raw custody, Treasury latest-month average-interest-rate context, CBO-derived fiscal debt dynamics context, and the endogenous net-interest formula boundary. | FY2032-FY2035 OMB PBD net-interest row values, maturity bucket schedule, remaining-maturity reconciliation, fiscal-year rate path, and solver feedback fixture. |

## Boundary

Wave B closes the source-custody/current-law pass for existing captured official
sources, but not all current-law paths are complete. It is not all source
custody complete. It is not solver-ready, not rate-ready, not savings-ready, and not balanced-budget-ready.

CBO revenue-detail values are receipt context only. They do not establish
matched legal/economic receipt bases, payer universes, incidence, distribution,
administration burden, current-law or reform yields, public rates, or solver
inputs.

No solver input, solver run, target cost, statutory rate, effective rate,
public rate card, savings estimate, waste finding, fraud finding,
department-cut instruction, technology-savings claim, or balanced-budget claim
is published.
