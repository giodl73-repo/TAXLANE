# NET CBO average-rate feedback schema

This record admits the matching-vintage CBO annual projected average interest
rate for a bounded incremental-feedback model. It preserves the published CBO
baseline and computes only debt and net-interest deltas caused by an admitted
primary-balance change.

Required invariants:

- the raw CBO file path, byte count, hash, vintage, variable, and percent unit
  match CORE-G custody;
- FY2025–FY2035 rates match the eleven CORE-G annual rows exactly;
- baseline debt and net interest are never reconstructed from Treasury snapshot
  rates;
- the incremental interest-receipt delta defaults to zero, while any nonzero
  candidate-specific effect requires evidence;
- current-year borrowing exposure uses the separately validated midpoint
  convention and closed-form circularity;
- a zero primary delta produces zero debt and interest deltas in all years;
- the mechanical $100 billion fixture recomputes every annual row from the
  identity, including later-year compounding;
- the artifact may close the matching-vintage average-rate path, incremental
  receipt treatment, and reduced-form feedback fixture, but not maturity-bucket
  stock, rollover, bucket stress, candidate savings, solver, or rate gates.
