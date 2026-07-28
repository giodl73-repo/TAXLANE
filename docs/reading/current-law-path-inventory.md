# Current-law path inventory

Machine record:
`data/derived/breadth_benchmark_matrix/current_law_path_inventory.v1.draft.json`

Pulse 108 inventories the official annual current-law paths required before the
baseline year plus ten-year deterministic solver horizon can be assembled. It does not publish current-law path values.

CORE-G now supplies a source-custodied FY2025-FY2035 federal topline spine.
That shared topline is ready and is sufficient to start TRN-A, but it is not a
17-row lane ledger, named-fund path, general-fund path, or lane solver input.

Required years: FY2025 through FY2035.

Still missing or partial:

- full 17-row FY2025 ledger binding to solver rows;
- baseline plus ten-year unified horizon;
- OASDI annual fund path;
- Medicare HI annual fund path;
- transportation trust-fund annual values;
- general fund annual path;
- health fiscal current-law path, now with FY2025 context and CBO FY2026-FY2035 major health-category context, but still blocked on fiscal-year HI, SMI, non-Medicare health, and OMB/CBO/CMS reconciliation;
- net interest current-law path.

Official source custody must include raw bytes, metadata, retrieval date, byte count, and SHA-256 before any value can be populated. Interpolation is not allowed without an explicit model. Missing values remain null. Trust funds remain separate, and Medicare HI remains separate.

This is a current-law path inventory, not current-law path values, not a solver run, not target-cost selection, not rate calculation, not a public rate card, not a tax proposal, not a savings estimate, not a waste finding, not a fraud finding, not a department-cut instruction, not a technology-savings claim, and not a balanced-budget claim.
