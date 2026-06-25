# Research Program — Handoff / Pickup Notes

_Last updated 2026-06-25 (T4 Income Security accepted). Everything below is committed and
pushed to `giodl73-repo/TAXLANE` (and the TRACKER submodule pointer is current)._

## Where things stand

A panel-reviewed research program (`research/`) with one **track per budget area**.
Five of six papers are **accepted** (panel avg ≥ ~3.5/4, all gates cleared):

| # | Paper (dir `research/publications/<slug>/`) | Track | Stage | Score |
|--:|---|---|---|---:|
| 1 | `health-funding-premium` | T1 Health | accepted | 3.64 |
| 5 | `low-tax-country-borrowing-habit` | T5 Revenue/Solvency | accepted | 3.57 |
| 2 | `old-age-tax-and-the-cap` | T2 Social Security | accepted | 3.64 |
| 3 | `defense-tax-in-allied-perspective` | T3 Defense | accepted | 3.71 |
| 4 | `the-thin-safety-net` | T4 Income Security | accepted | 3.64 |
| 0 | `legible-federal-funding` | T0 Synthesis | **NOT STARTED (next, last)** | — |

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

## Done since last handoff: T3 Defense (3.71) and T4 Income Security (3.64)

**T3 `defense-tax-in-allied-perspective` (3.71).** US defense ~3.0% GDP (OMB function-050)
/ ~3.2–3.4% (SIPRI/NATO), above the European NATO norm (~2.0%), far below US Cold-War
levels; Hague 5%-by-2035 (3.5% core + 1.5% broader) is the live driver; "fair rate" is a
**policy band (2.0–3.5% of GDP, NATO/SIPRI basis)**, not a fiscal finding. Round 1 failed
PF-2's gate on basis-mixing; round 2 fixed it (single-basis tables + band), quantified the
borrowed share (~$235B), cleared at 3.71. Sources added: `SRC-NATO-HAGUE-2025`,
`SRC-OMB-HIST-6-1-FY2027`.

**T4 `the-thin-safety-net` (3.64).** US public **family benefits ~0.6% GDP raw, ~1.1%
once tax-code delivery (refundable EITC ~$62B + CTC ~$65B ≈ ~0.5% GDP) is netted in, vs
OECD ~2.4%** — below the norm on family support specifically (NOT total social spend,
where US is ~19.8% vs 21.2% gross / near top net). Income security (function 600) =
$701,609M FY2025 = 10.0% of outlays. Direction: strengthen + stabilize; the 2021 CTC
episode (SPM child poverty 5.2%→12.4%) is the evidence. Round 1 failed **two** gates —
PF-2 (the raw 0.6% overstates the gap by undercounting tax-code delivery) and PF-6 (the
CTC swing overclaims causality vs stimulus/UI/inflation confounders); round 2 netted the
tax-code support (gap ~2× not ~4×) and de-overclaimed the CTC (major-not-sole driver),
cleared at 3.64. Source added: `SRC-CENSUS-P60-280`.

**Two reusable lessons for the synthesis:** (1) when a figure exists on more than one
definitional basis (function vs SIPRI/NATO; %outlays vs %GDP; federal vs all-government;
SOCX public vs net-total vs TBSP), never rank/compare across bases — one table per basis,
label the gap definitional, and **net in tax-code delivery** before stating a cross-country
gap. (2) Treat single dramatic policy episodes (the 2021 CTC) as "clearest single evidence,
not a controlled experiment" — name confounders, attribute a share not the whole.

## Next (last paper): T0 Synthesis — "legible federal funding"

Slug `legible-federal-funding`. This is the **meta/synthesis** paper that ties the five
accepted tracks together into the program's thesis: a legible, balanced, program-lane
funding system. It is **not** a new budget area — it should **synthesize**, not re-derive:

- Pull each track's headline in **one comparable frame** (each on its own correct basis,
  per lesson 1): Health (T1, above-peer **cost**, lever = price-per-outcome), Social
  Security (T2, peer-normal spend, below-peer rate on a low capped base, lever = the cap),
  Defense (T3, above European norm / policy band 2.0–3.5%, strategic not fiscal), Income
  Security (T4, **below** OECD family-support norm, strengthen + stabilize), Revenue/
  Solvency (T5, ~8.5 pts of GDP below the OECD tax average — the cross-cutting frame).
- The synthesis argument: the US over-funds some lanes (health cost), under-funds others
  (family), and under-collects overall (T5) — so "legible funding lanes" is about showing
  each lane's level, basis, borrowed share, and outcome, not picking one number.
- Reuse only figures already in the ledger (no new sourcing expected). Same lifecycle:
  draft → round-1 panel → revise → round-2 → accept; same gate (PF-2 + PF-6 + PF-7);
  same scope-label / normative-label discipline.
- After T0: optional **LaTeX/PDF promotion** of the accepted papers (MiKTeX works here;
  mirror BISECT's `docs/papers/`), and consider whether the program warrants a public
  reading packet under `docs/reading/`.

**Likely no new sources needed for T0** — it cites the existing per-track ledger entries.
If a single "all lanes on one chart" figure is wanted, derive it from the OMB function
tables already registered (`SRC-OMB-HIST-3-1/3-2-FY2027`, `SRC-OMB-HIST-1-2-FY2027`).

## Permissions (the friction)
Settings load at session start, so the `bypassPermissions` default set in
`C:\src\TRACKER\.claude\settings.local.json` only applies after a **restart** of Claude
in this folder — or press **Shift+Tab** to toggle "bypass permissions" live. Also keep
subagent fan-out small (1–2 per task): each subagent's web/shell calls were the main
source of prompts.

## Auth reminder
Before GitHub ops: `Remove-Item Env:GH_TOKEN; gh auth switch -h github.com -u giodl73-repo`
(or `unset GH_TOKEN` in bash). Push child repo, then update the TRACKER pointer.
