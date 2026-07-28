# CORE-G Official Current-Law Solver Spine Schema

CORE-G admits exactly eleven FY2025-FY2035 federal topline rows from the CBO
February 2026 open-data vintage. FY2025 is the release's actual row and
FY2026-FY2035 are projections.

Every row must satisfy, at published source precision:

- total outlays = primary outlays + net interest;
- primary deficit = primary outlays - receipts;
- total deficit = total outlays - receipts;
- total deficit = primary deficit + net interest;
- ending debt = beginning debt + total deficit + other financing and timing;
- timing residual = exact debt-identity residual - reported other financing.

Both raw CSVs must match their recorded byte counts and SHA-256 hashes. All
eight CORE-G contract gates must pass. CORE-G completion may unlock `TRN-A`, but
does not imply CORE-H, fund-path, reform, solver, rate, savings, or
balanced-budget readiness.
