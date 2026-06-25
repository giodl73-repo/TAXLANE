# TAXLANE Research Program

Panel-reviewed research papers, one **track per budget area**, applying the panel
methodology used across the portfolio (see `c:\src\panel`, and BISECT's
`research/publications/`): each paper is drafted, reviewed by a standing panel of
simulated public-finance expert reviewers across scored rounds, synthesized, and
revised toward publication quality.

## What each paper argues

For its budget area, every paper makes the same shape of argument:

1. **What the US funds today** — the current rate/cost and how it is financed.
2. **History** — how the area's funding got here; what rate prevailed when the
   budget was solvent.
3. **Peer benchmark** — what comparable countries levy and spend for the same
   public purpose.
4. **All factors → a recommended rate** — whether the evidence supports a **cut**
   or an **increase**, decomposed into solvency, historical, and international
   components.
5. **The needs and the problem** — why the US is struggling in this area and what
   it would take to **modernize** the way it is funded.
6. **Reform design** — the program-linked lane, its rate, and the operating rules,
   carried from the design and operating-system waves.

## Structure (panel convention)

```
research/
  README.md            this file
  RESEARCH.md          track/paper inventory + status
  REVIEWERS.md         the standing public-finance review panel (AI-simulated)
  publications/
    <slug>/
      plan.md          the paper's research plan
      paper.md         the paper (markdown tier; promotable to LaTeX/PDF)
      _panel.yaml      panel state: stage, round, reviewers, scores, history
      reviews/         REVIEW-<reviewer>.md, SYNTHESIS.md
      REVISION-PLAN.md revision plan after each round
```

## Honesty rules (carried from the repo)

- Reviewers are **AI-simulated review lenses**, not real people; no external
  endorsement is implied.
- Every figure cites a source in `docs/sources/source-version-ledger.md`.
- Rate recommendations are **reform proposals**, labeled as such; the borrowed
  share stays visible; the anti-waste case is argued, not asserted.
