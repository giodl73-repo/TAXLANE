# REV internal-analysis baseline-freeze schema

The record freezes the model version, CPS input class, TY2026 baseline rates,
FY2026 revenue target, zero admitted spending contribution, first-year timing
ratio, behavior cases, candidate grid, and exact SHA-256 hashes of the prior
Taxlane artifacts from which the run proceeds.

Every frozen input must exist and match its digest. The grid must contain nine
unique ordered uplifts and three elasticity cases per candidate. External
requests and official-score claims remain false.
