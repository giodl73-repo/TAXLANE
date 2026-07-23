# TAXLANE Research — Compiled Papers

PDF renderings of the six accepted papers from the panel-reviewed **budget-area research
program**. The markdown tier — `research/publications/<slug>/paper.md` — is the **source
of truth**; these PDFs are generated from it and are a convenience rendering, not a
separate edited version.

All six papers cleared the standing panel's acceptance gate (PF-2 comparability +
PF-6 reform-skeptic + PF-7 sourcing) over two rounds. Scores are panel averages on the
0–4 scale; reviewers are **AI-simulated public-finance review lenses**, not real people,
and no external endorsement is implied (see `research/REVIEWERS.md`).

For current implementation readiness, use the
[Taxlane showcase readiness summary](../reading/taxlane-showcase-readiness-summary.md).
The papers establish the public-policy thesis; the showcase summary states what
the repo can demonstrate today and which solver, rate, savings, public-card, and
balanced-budget outputs remain blocked.

| PDF | Source | Track | Title | Score |
|---|---|---|---|---:|
| [0+legible-federal-funding.pdf](0+legible-federal-funding.pdf) | [paper.md](../../research/publications/legible-federal-funding/paper.md) | T0 Synthesis | Legible Federal Funding | 3.64 |
| [1+health-funding-premium.pdf](1+health-funding-premium.pdf) | [paper.md](../../research/publications/health-funding-premium/paper.md) | T1 Health | The American Health Funding Premium | 3.64 |
| [2+old-age-tax-and-the-cap.pdf](2+old-age-tax-and-the-cap.pdf) | [paper.md](../../research/publications/old-age-tax-and-the-cap/paper.md) | T2 Social Security | The Old-Age Tax and the Cap | 3.64 |
| [3+defense-tax-in-allied-perspective.pdf](3+defense-tax-in-allied-perspective.pdf) | [paper.md](../../research/publications/defense-tax-in-allied-perspective/paper.md) | T3 Defense | The Defense Tax in Allied Perspective | 3.71 |
| [4+the-thin-safety-net.pdf](4+the-thin-safety-net.pdf) | [paper.md](../../research/publications/the-thin-safety-net/paper.md) | T4 Income Security | The Thin Safety Net | 3.64 |
| [5+low-tax-country-borrowing-habit.pdf](5+low-tax-country-borrowing-habit.pdf) | [paper.md](../../research/publications/low-tax-country-borrowing-habit/paper.md) | T5 Revenue/Solvency | A Low-Tax Country with a Borrowing Habit | 3.57 |

Start with [0+legible-federal-funding.pdf](0+legible-federal-funding.pdf) (the
synthesis/overview), then read the tracks in any order.

## Building

```powershell
pwsh docs/papers/build.ps1
```

Requires **pandoc** and a **LaTeX engine** (MiKTeX `xelatex`). The build uses a
Unicode-capable mainfont (Cambria) because the papers contain symbols such as `→`, `×`,
`≈`, `≥`, and `≠`; the default Latin Modern font drops these. Re-run the script after
editing any `paper.md` to refresh the PDFs.

## Honesty note

Figures are sourced to `docs/sources/source-version-ledger.md`. Rate and reform
recommendations are **proposals** and, where applicable, **normative/value judgments**
labeled as such; projections (Trustees, CBO, OECD) are labeled; no tax, legal, or
national-security advice is given.
