# Health CBO source custody gap

Machine record:
`data/derived/breadth_benchmark_matrix/health_cbo_source_custody_gap.v1.draft.json`

Pulse 181 records a specific health/Medicare source gap: CBO source IDs appear
in derived health context artifacts, but local raw CBO health-baseline custody
is not ready in this packet.

Referenced but not custody-ready:

- `SRC-CBO-LTBO`
- `SRC-CBO-COMMERCIAL-PROVIDER-PRICES`

Required before CBO can populate federal health policy translation or score
context fields:

- raw artifact path;
- raw byte count;
- raw SHA-256;
- metadata path;
- retrieval date;
- health baseline table lineage;
- behavior and incidence table lineage.

CBO source IDs appear in derived health context artifacts, but local raw CBO health baseline custody is not ready in this packet. This is not CBO source capture, not federal health policy translation, not behavior modeling, not incidence modeling, not pass/fail findings, not lower-cost scenario admissibility, not target-cost selection, not gross savings, not net savings, not solver input, not rate calculation, not a public rate card, not a technology-savings claim, and not a balanced-budget claim.
