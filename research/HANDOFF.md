# Research Program — Handoff / Pickup Notes

_Last updated 2026-06-24 (T3 Defense accepted). Everything below is committed and pushed
to `giodl73-repo/TAXLANE` (and the TRACKER submodule pointer is current)._

## Where things stand

A panel-reviewed research program (`research/`) with one **track per budget area**.
Four of six papers are **accepted** (panel avg ≥ ~3.5/4, all gates cleared):

| # | Paper (dir `research/publications/<slug>/`) | Track | Stage | Score |
|--:|---|---|---|---:|
| 1 | `health-funding-premium` | T1 Health | accepted | 3.64 |
| 5 | `low-tax-country-borrowing-habit` | T5 Revenue/Solvency | accepted | 3.57 |
| 2 | `old-age-tax-and-the-cap` | T2 Social Security | accepted | 3.64 |
| 3 | `defense-tax-in-allied-perspective` | T3 Defense | accepted | 3.71 |
| 4 | `the-thin-safety-net` | T4 Income Security | **NOT STARTED (next)** | — |
| 0 | `legible-federal-funding` | T0 Synthesis | not started | — |

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

## Done since last handoff: T3 Defense (accepted 3.71)

Slug `defense-tax-in-allied-perspective`. Thesis landed: US defense ~3.0% GDP (OMB
function-050) / ~3.2–3.4% (SIPRI/NATO) is above the European NATO norm (~2.0%) and far
below US Cold-War levels; the Hague 5%-by-2035 (3.5% core + 1.5% broader) is the live
driver; the "fair rate" is a **policy band (2.0–3.5% of GDP, NATO/SIPRI basis)** set by
strategic judgment, not a fiscal finding. Round 1 failed PF-2's gate on basis-mixing;
round 2 fixed it (single-basis peer tables + band), quantified the borrowed share
(~26% of outlays / ~$235B, generational incidence), and cleared at 3.71.
Both new sources are registered: `SRC-NATO-HAGUE-2025`, `SRC-OMB-HIST-6-1-FY2027`.
**Lesson for the remaining papers:** when a figure exists on more than one definitional
basis (function vs SIPRI/NATO; %outlays vs %GDP; federal vs all-government), never rank
or band across bases — give one table/interval per basis and label the gap definitional.

## Next: T4 Income Security & Family — "the thin safety net"

Slug `the-thin-safety-net`. Thesis: US family/income-security funding is **below** the
OECD norm; the modernization case is to **strengthen, not cut** (direction: increase).
Primary comparator already in the ledger: `SRC-OECD-SOCX` (public social + family-benefit
spend % GDP; US net-total incl. private differs — label scope). Likely also useful:
`SRC-OMB-HIST-3-2-FY2027` (function 600 income security outlays), `SRC-OMB-HIST-1-2-FY2027`
(GDP denominator), `SRC-CENSUS-P60-288` (poverty/coverage context, already in ledger).
Bake in the same lessons: scope labels (public vs net-total; federal vs all-government),
conditional framing, a distributional/incidence subsection, a forward trajectory, and
every figure SRC-cited. Then the **T0 synthesis** (`legible-federal-funding`), then
optional **LaTeX/PDF promotion** of the accepted papers (MiKTeX works here; mirror
BISECT's `docs/papers/`).

**Still to source for T4** (the only likely gap): a clean OECD family-benefit-%-GDP
figure and a US income-security %-GDP figure on matched scope; check whether `SRC-OECD-SOCX`
already carries the family-benefit breakout or whether a sub-source is needed.

## Permissions (the friction)
Settings load at session start, so the `bypassPermissions` default set in
`C:\src\TRACKER\.claude\settings.local.json` only applies after a **restart** of Claude
in this folder — or press **Shift+Tab** to toggle "bypass permissions" live. Also keep
subagent fan-out small (1–2 per task): each subagent's web/shell calls were the main
source of prompts.

## Auth reminder
Before GitHub ops: `Remove-Item Env:GH_TOKEN; gh auth switch -h github.com -u giodl73-repo`
(or `unset GH_TOKEN` in bash). Push child repo, then update the TRACKER pointer.
