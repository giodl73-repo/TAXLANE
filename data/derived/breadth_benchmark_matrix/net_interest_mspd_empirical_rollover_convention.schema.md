# NET MSPD empirical marginal-rollover convention schema

This record turns twelve Treasury MSPD Table 3 monthly snapshots into an
empirical original-term mix and a deterministic rollover convention for
**incremental policy debt deltas only**.

Required invariants:

- source path, byte count, hash, record window, and units match local custody;
- the record-month filter includes the first calendar day and yields 434
  distinct issue/reopening rows totaling `$31,594,618.4275 million`;
- same-CUSIP rows remain separate when their issue date, amount, or yield
  differs, while the declared exact duplicate key has zero duplicates;
- the eight original-term buckets are exhaustive, their source amounts sum to
  gross issuance, and their integer shares sum to one billion parts;
- representative terms derive from amount-weighted original-term days under
  the declared month conversion rule;
- signed allocation and every refinancing event preserve principal exactly;
- the mechanical negative `$100 billion` fixture replays for all ten fiscal-
  year snapshots through FY2035;
- the observed mix may close the maturity input for marginal incremental
  feedback, but it may not be described as a future Treasury issuance plan, a
  full-stock forecast, candidate savings, a solver result, or a rate effect.

The `rollover_since_prior_snapshot_musd_micros` field counts repeated
refinancing of the same signed principal. It is not additive savings or an
interest amount.
