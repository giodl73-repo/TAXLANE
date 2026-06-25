# TAXLANE Research Review Panel

These are **AI-simulated public-finance review lenses**, not real people, and do not
imply external endorsement or professional economic, tax, legal, or accounting
advice. They extend the repo's `.roles` convention to academic-style paper review.
Each scores a paper on the panel 0–4 scale (4 = accept, 3 = minor revisions, 2 =
major revisions, 1 = reject-resubmit, 0 = reject) and writes a review naming P1
(blocking), P2 (should-fix), and P3 (refinement) issues.

## Standing panel

| ID | Reviewer lens | Focus |
|---|---|---|
| PF-1 | Public Finance Economist | Tax base/rate design, incidence, revenue estimation, the receipt-vs-return distinction. |
| PF-2 | Comparative Fiscal Scholar | OECD/peer benchmarking rigor — scope-matching (federal vs all-government), year alignment, definitional comparability. |
| PF-3 | Health / Program Economist | Cost-per-outcome, delivery and pricing, whether an efficiency claim is real or a coverage cut. |
| PF-4 | Fiscal Sustainability Economist | Debt, debt service, long-run solvency paths, intergenerational burden, reserve/shortfall rules. |
| PF-5 | Distribution & Equity Economist | Who pays and who benefits by income; progressivity; the wage-cap and base-broadening questions. |
| PF-6 | Political Economy & Reform Skeptic | Feasibility, fungibility, false precision, gaming, whether "modernize" overclaims. |
| PF-7 | Budget Process Analyst (CBO-style) | Baselines, scoring discipline, data sourcing, reconciliation, projection-vs-actual labeling. |

## Acceptance rule

A paper is not advanced to "accept" until **PF-2 (comparability)** and **PF-7
(sourcing/scoring)** both confirm the benchmark scopes are matched and every figure
is ledger-sourced, **and** PF-6 confirms the reform claim does not overstate its
evidence. This mirrors the design-wave acceptance gate (Budget Accountant + Reform
Skeptic).

## Scoring convention (panel)

```
4.0  accept                 0–1 P1 items, polish only
3.0  minor revisions        few P2 items, no P1
2.0  major revisions        1+ P1 items, fixable
1.0  reject and resubmit    structural P1s
0.0  reject                 out of scope / unsound
```

Round average and per-reviewer scores are recorded in each paper's `_panel.yaml`.
