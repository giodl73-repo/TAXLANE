# NET MSPD public-maturity envelope schema

This record converts the reconciled June 30, 2026 MSPD Table 3 detail into an
exact fiscal-year runoff schedule for securities already outstanding and a
mathematically valid interval for the public-held portion.

Required invariants:

- source path, byte count, hash, record date, unit, and marketability perimeter
  match the validated MSPD snapshot reconciliation;
- 462 future-maturity rows sum to the reconciled Table 3 future detail;
- FY2026–FY2035 and FY2036–FY2056 partitions are exhaustive and disjoint;
- for any group amount `A`, the lower bound is
  `max(0, A - marketable_intragovernmental)` and the upper bound is
  `min(A, marketable_debt_held_public)`;
- annual intervals are explicitly non-additive; aggregate amounts must be
  summed first and bounded exactly once;
- no pro-rata holder allocation, future issuance, term mix, or rollover is
  inferred;
- the artifact may publish exact existing-stock runoff and public-holder
  envelopes, but not a point allocation, complete maturity input, savings,
  solver result, or rate effect.
