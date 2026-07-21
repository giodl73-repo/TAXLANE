# Health NHE source custody gap

Machine record:
`data/derived/breadth_benchmark_matrix/health_nhe_source_custody_gap.v1.draft.json`

Pulse 180 records a specific health/Medicare source gap: CMS NHE source IDs
appear in derived health sensitivity artifacts, but local raw NHE custody is not
ready in this packet.

Referenced but not custody-ready:

- `SRC-CMS-NHE-TABLES-2024`
- `SRC-CMS-NHE-2024`

Required before NHE can populate any health floor source field:

- raw artifact path;
- raw byte count;
- raw SHA-256;
- metadata path;
- retrieval date;
- table scope and cell lineage.

CMS NHE source IDs appear in derived health sensitivity artifacts, but local raw NHE custody is not ready in this packet. This is not NHE source capture, not health floor threshold selection, not observed floor values, not pass/fail findings, not lower-cost scenario admissibility, not target-cost selection, not gross savings, not net savings, not solver input, not rate calculation, not a public rate card, not a technology-savings claim, and not a balanced-budget claim.
