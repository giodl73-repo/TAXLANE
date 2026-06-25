# Research Program — Handoff / Pickup Notes

_Last updated 2026-06-25 (T0 Synthesis accepted — **program complete, 6/6 tracks**).
Everything below is committed and pushed to `giodl73-repo/TAXLANE` (and the TRACKER
submodule pointer is current)._

## Where things stand

A panel-reviewed research program (`research/`) with one **track per budget area**.
**All six papers are accepted** (panel avg ≥ ~3.5/4, all gates cleared):

| # | Paper (dir `research/publications/<slug>/`) | Track | Stage | Score |
|--:|---|---|---|---:|
| 1 | `health-funding-premium` | T1 Health | accepted | 3.64 |
| 5 | `low-tax-country-borrowing-habit` | T5 Revenue/Solvency | accepted | 3.57 |
| 2 | `old-age-tax-and-the-cap` | T2 Social Security | accepted | 3.64 |
| 3 | `defense-tax-in-allied-perspective` | T3 Defense | accepted | 3.71 |
| 4 | `the-thin-safety-net` | T4 Income Security | accepted | 3.64 |
| 0 | `legible-federal-funding` | T0 Synthesis | accepted | 3.64 |

Inventory + tracks: `research/RESEARCH.md`. Reviewer panel: `research/REVIEWERS.md`
(7 AI-simulated public-finance lenses PF-1..PF-7; gate = PF-2 + PF-6 + PF-7).

## The process (per paper)

1. Create `research/publications/<slug>/` with `plan.md`, `_panel.yaml` (stage draft),
   `paper.md` (markdown tier), `reviews/`.
2. **Draft** `paper.md` — sections: what the US funds today · history · peer benchmark ·
   all-factors→the rate (cut/increase) · needs + why-US-struggles + modernization ·
   reform design · distribution · discussion · conclusion.
3. **Panel review**: run the 7 PF reviewers → `reviews/REVIEW-PF-*.md`, `SYNTHESIS.md`,
   score in `_panel.yaml`.
4. **Revise** per `REVISION-PLAN.md` → re-review (`ROUND2-REVIEW-*.md`,
   `ROUND2-SYNTHESIS.md`) until gate clears and avg ≥ ~3.0.
5. Update `RESEARCH.md` inventory + the active wave `context/waves/2026-06-24-budget-area-research-program/`.
6. **Commit child first, then update the TRACKER submodule pointer** (snapshot policy).

**Lessons baked in (write these in from the start to clear round 1 faster):** label
scope on every ratio (federal vs all-government; tax-year vs fiscal-year); present
savings as *conditional* ceilings, not bankable; quantify outcomes, not just spending;
show a forward/aging trajectory; include a *distributional* incidence subsection
(annual vs lifetime); every figure must cite a `SRC-` ID that exists in
`docs/sources/source-version-ledger.md`; frame normative claims ("the fair lever is X")
as value judgments, not findings.

## Done since last handoff: T0 Synthesis (3.64) — PROGRAM COMPLETE

**T0 `legible-federal-funding` (3.64).** The meta paper ties all five tracks into one
frame via two tables: **Table A** internal budget shares (additive, OMB function, % of
FY2025 outlays — Health+Medicare 28.2%, Social Security 22.5%, **net interest 13.8%**,
Defense 13.1%, Income Security 10.0%; ~87.6% of outlays) and **Table B** international
comparisons (per-basis, **not additive**). Thesis: the US over-funds health on price,
under-funds family support, funds defense by strategy, runs a structurally-capped old-age
lane, and under-collects overall — so the reform is lane-specific and the contribution is
a legibility standard (level · basis-correct benchmark · borrowed share · outcome). Round 1
failed PF-2's gate (Table A had a %-GDP net-interest cell in the additive %-outlays
column); round 2 fixed it (net interest function 900 = **$970,065M = 13.8% of outlays**,
> defense), reframed "internally inconsistent" → "uneven against peer/outcome benchmarks,"
and kept the paper diagnostic, not prescriptive. No new sources. Cleared at 3.64.

**The budget-area research program is complete — all six tracks accepted** (T1 3.64,
T5 3.57, T2 3.64, T3 3.71, T4 3.64, T0 3.64).

## Optional next steps (program is otherwise done)

- **LaTeX/PDF promotion — DONE.** All six accepted papers are rendered to PDF in
  `docs/papers/` (`0+legible-federal-funding.pdf` … `5+low-tax-country-borrowing-habit.pdf`)
  via `docs/papers/build.ps1` (pandoc + MiKTeX xelatex; Cambria mainfont for the Unicode
  symbols). Re-run the script after editing any `paper.md`.
- **Public reading packet** under `docs/reading/` distilling the six papers for a lay
  audience (the repo's existing reading packets are the model).
- **Exact-extraction backfill**: a few figures are labeled "approximate / OECD-reported,
  extraction pending" (the T3 §2 defense-%-GDP history from Table 6.1; the T4 OECD SOCX
  family-benefit values). A data-layer pass could pin them to extracted values.
- If new budget areas are ever wanted (education 500, veterans 700, international 150),
  the same lifecycle + the two reusable lessons below apply.

**Two reusable lessons (carried through all six papers):** (1) when a figure exists on more
than one definitional basis (function vs SIPRI/NATO; %outlays vs %GDP; federal vs
all-government; SOCX public vs net-total vs TBSP; additive budget shares vs non-additive
%-GDP comparisons), never rank/compare/sum across bases — one table per basis, label the
gap definitional, and net in tax-code delivery before stating a cross-country gap. (2) Treat
single dramatic policy episodes (the 2021 CTC) as "clearest single evidence, not a
controlled experiment" — name confounders, attribute a share not the whole.

## Permissions (the friction)
Settings load at session start, so the `bypassPermissions` default set in
`C:\src\TRACKER\.claude\settings.local.json` only applies after a **restart** of Claude
in this folder — or press **Shift+Tab** to toggle "bypass permissions" live. Also keep
subagent fan-out small (1–2 per task): each subagent's web/shell calls were the main
source of prompts.

## Auth reminder
Before GitHub ops: `Remove-Item Env:GH_TOKEN; gh auth switch -h github.com -u giodl73-repo`
(or `unset GH_TOKEN` in bash). Push child repo, then update the TRACKER pointer.
