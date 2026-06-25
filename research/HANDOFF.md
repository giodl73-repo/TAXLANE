# Research Program — Handoff / Pickup Notes

_Last updated 2026-06-24. Everything below is committed and pushed to
`giodl73-repo/TAXLANE` (and the TRACKER submodule pointer is current)._

## Where things stand

A panel-reviewed research program (`research/`) with one **track per budget area**.
Three of six papers are **accepted** (panel avg ≥ ~3.5/4, all gates cleared):

| # | Paper (dir `research/publications/<slug>/`) | Track | Stage | Score |
|--:|---|---|---|---:|
| 1 | `health-funding-premium` | T1 Health | accepted | 3.64 |
| 5 | `low-tax-country-borrowing-habit` | T5 Revenue/Solvency | accepted | 3.57 |
| 2 | `old-age-tax-and-the-cap` | T2 Social Security | accepted | 3.64 |
| 3 | `defense-tax-in-allied-perspective` | T3 Defense | **NOT STARTED (next)** | — |
| 4 | `the-thin-safety-net` | T4 Income Security | not started | — |
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

## Next: T3 Defense — data already gathered (don't re-fetch)

Slug `defense-tax-in-allied-perspective`. Thesis: US defense (~3% of GDP) exceeds the
European norm (~2%) and the 2% floor; the 2025 Hague 5%-by-2035 commitment is the live
driver; the "fair rate" is a **policy band (2.0–3.5% of GDP)** set by strategic/threat
judgment, not a fiscal finding — be honest that the factors *bound* but don't *determine*
the level.

Figures (sourced; SRC IDs already in the ledger unless marked NEW):
- US defense FY2025 cost **$916,140M = 13.1% of outlays** (`SRC-OMB-HIST-3-2-FY2027`, function 050).
- US **~3.0% of GDP** (OMB function basis) / **3.2–3.4%** (SIPRI/NATO basis) — flag the
  definitional difference (`SRC-SIPRI-MILEX-2024`, `SRC-NATO-DEFEXP-2025`).
- NATO-Europe aggregate **~2.0%**; Germany 2.0, France 2.05, UK 2.25–2.40, Poland 3.79–4.48,
  Japan 1.4, China ~1.7, Russia ~7.1 (SIPRI 2024 / NATO 2025).
- NATO **2% guideline** (2006; 2014 Wales; 2023 Vilnius "at least 2%"); **Hague 25 Jun 2025:
  5% by 2035 = 3.5% core + 1.5% broader security**.
- Cold-War peak **~14.2% of GDP (1953)**; Reagan ~6.7% (FY1983–85); post-Cold-War low ~3% (2000).

**Still to source (the rejected research agent was going to fetch these):**
- Add a ledger source `SRC-NATO-HAGUE-2025` for the Hague Summit Declaration text.
- Source the historical US defense-%-GDP series properly (OMB Historical Table 6.1 or CBO)
  → add e.g. `SRC-OMB-HIST-6-1` before publishing the Cold-War-peak table.

Then T4 (US family/income-security funding **below** OECD norm — `SRC-OECD-SOCX`; modernization =
strengthen, not cut), then the T0 synthesis, then optional **LaTeX/PDF promotion** of the
accepted papers (MiKTeX works here; mirror BISECT's `docs/papers/`).

## Permissions (the friction)
Settings load at session start, so the `bypassPermissions` default set in
`C:\src\TRACKER\.claude\settings.local.json` only applies after a **restart** of Claude
in this folder — or press **Shift+Tab** to toggle "bypass permissions" live. Also keep
subagent fan-out small (1–2 per task): each subagent's web/shell calls were the main
source of prompts.

## Auth reminder
Before GitHub ops: `Remove-Item Env:GH_TOKEN; gh auth switch -h github.com -u giodl73-repo`
(or `unset GH_TOKEN` in bash). Push child repo, then update the TRACKER pointer.
