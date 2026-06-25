# Pulse 14: Exact-Extraction Backfill (T3 + T4) — Maintenance

Post-program maintenance pulse: replaced the two remaining "approximate / extraction
pending" figures with exact, sourced values. No re-grade — a light PF-2/PF-7 re-review
confirms both gates still pass (see each paper's `reviews/BACKFILL-REREVIEW.md`).

## Changes
- **T3 Defense — §2 history pinned to OMB Table 6.1** (`SRC-OMB-HIST-6-1-FY2027`,
  hist06z1_fy2027.xlsx, national-defense % of GDP by year): Korean War **13.8% (1953)**,
  Vietnam 9.1% (1968), post-Vietnam trough 4.5% (1979), **Reagan peak 6.0% (FY1986)**,
  post-Cold-War low 2.9% (2000), post-9/11 4.7% (2010), WWII 37.0% (1944), today 3.0%.
  **Corrected a real overstatement:** the prior "~6.5–7%" Reagan figure was high for the
  function-050 basis (actual 6.0%). Updated the abstract, §5 band ceiling, and conclusion.
- **T4 Income Security — OECD SOCX figures pinned** (`SRC-OECD-SOCX`, 2022 aggregates):
  US family benefits **0.6%**, OECD **~2.1–2.3%** (tightened from the prior ~2.4% round
  figure); US total public social 19.8% vs OECD 21.2%. Removed the "extraction pending"
  label; updated all mentions to the ~2.2% norm. The tax-adjusted ~2× gap is unchanged.
- Rebuilt all six PDFs (`docs/papers/build.ps1`); updated the reading packet's two
  affected lay figures (Reagan "about 6%", family norm "near 2.2%").

## Status
Done. Both papers' gates re-confirmed; acceptance stands. Program remains 6/6 accepted.
