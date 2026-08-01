# NET Treasury MSPD snapshot reconciliation schema

This record reconciles the 2026-06-30 Treasury MSPD marketable-debt snapshot
without promoting it to an annual rollover model.

Required invariants:

- Table 1 and Table 3 amounts are millions of dollars.
- Table 5 amounts are thousands of dollars and are divided by 1,000.
- Table 5 is a STRIPS decomposition of Table 3 notes, bonds, and TIPS and is
  never added to Table 3.
- Converted Table 5 grand total equals the Table 3 notes/bonds/TIPS future-
  maturity total.
- Table 3 future-maturity detail plus Federal Financing Bank and the explicit
  matured/unallocated residual equals Table 1 total marketable debt.
- The seven snapshot buckets sum to the Table 3 future-maturity detail total.
- Treasury actual debt held by the public and CBO fiscal-year-end projected
  debt remain separated by date and an explicit residual.

The snapshot seed is useful for maturity diagnostics but does not allocate
buckets between public and intragovernmental holders, define annual rollover,
map bucket rates, or authorize interest feedback, savings, solver, or rate
claims.
