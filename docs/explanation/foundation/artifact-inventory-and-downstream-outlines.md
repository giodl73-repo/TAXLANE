# EXPL-A artifact inventory and downstream outlines

## Canonical evidence inventory

| Purpose | Reader | Machine source |
|---|---|---|
| terminal track result | `docs/reading/fifteen-track-terminal-disposition.md` | `fifteen_track_terminal_disposition.v1.draft.json` |
| integrated fifteen-track rerun | `docs/reading/fifteen-track-integrated-dependency-admission-rerun.md` | `fifteen_track_integrated_dependency_admission_rerun.v1.draft.json` |
| spending-rate decision | `docs/reading/targeted-spending-rate-decision.md` | `targeted_spending_rate_decision.v1.draft.json` |
| internal rate result | `docs/reading/rev-internal-rate-analysis-completion.md` | `rev_internal_rate_analysis_completion.v1.draft.json` |
| PAY–NET–REV identity | `docs/reading/pay-net-rev-post-fifteen-track-reconciliation.md` | `pay_net_rev_post_fifteen_track_reconciliation.v1.draft.json` |
| explanation program | `docs/reading/taxlane-final-result-explanation-program.md` | `taxlane_final_result_explanation_program.v1.draft.json` |

All paths are below `data/derived/breadth_benchmark_matrix/` unless shown
otherwise. Downstream materials link to reader and machine layers.

## Custody and vintage

| Record | Pulse | Analysis date / status |
|---|---:|---|
| internal rate completion | 477 | 2026-07-27; completed internal recommendation |
| targeted spending-rate decision | 478 | 2026-07-27; HLT/DEF zero admission |
| fifteen-track terminal disposition | 479 | 2026-07-27; internal portfolio complete |
| explanation program | 480 | 2026-07-27; repository-only program |

Underlying sources have their own fiscal, calendar, or tax-year vintages. A
display preserves each source's basis rather than treating the analysis date as
the data year.

## Citizen-guide outline

1. The question: what must be financed after responsible spending evidence?
2. The short answer.
3. Why fifteen tracks exist.
4. Why a headline saving is not automatically usable.
5. What zero admission means.
6. How the remaining revenue target was modeled.
7. What marginal bracket rates mean—and do not mean.
8. Central, contingency, and stress tiers.
9. What could change the result.
10. Limits, evidence routes, and reproduction.

The strengthened canonical caveat appears beside the first rate schedule, not
only at the end.

## Synthesis-paper outline

1. Research question and contribution.
2. Fifteen-track architecture.
3. Shared accounting and evidence-admission method.
4. Candidate decisions and the zero-admission result.
5. PAY, OAS, and NET boundary treatment.
6. Revenue model and sensitivity design.
7. Preferred result and alternative tiers.
8. Distribution, administration, and uncertainty.
9. Limitations and reopening conditions.
10. Implications for legible fiscal analysis.

## Presentation narrative spine

Every deck follows: question → method → fifteen-track result → evidence-gate
example → accounting identity → rate tiers → what would change the result →
caveats and evidence. Shorter decks omit depth, not boundaries.

Speaker notes repeat the fund boundary, beneficiary-floor rationale,
compliance-burden limitation, one-year/long-run distinction, and Taxlane's lack
of legal or appropriations authority.

## Local HTML information architecture

- `/index.html`: result and caveat.
- `/tracks.html`: fifteen-track matrix and reopening triggers.
- `/rates.html`: marginal-rate explainer and three-tier ladder.
- `/method.html`: admission gates and PAY–NET–REV identity.
- `/evidence.html`: reader/machine routes and reproduction commands.
- `/glossary.html`: controlled terms.

All routes are local relative links. There is no deployment configuration,
analytics, form submission, remote asset, or tracking behavior.

## Review matrix

| Surface | Required `.roles` lenses | Special gate |
|---|---|---|
| canonical claims and numbers | T-3, then T-2 and T-7 | source, basis, fund, debt |
| citizen guides | T-1, T-4, T-5, T-6, T-8 | comprehension and non-overclaim |
| papers | T-1 through T-8 plus standing PF paper lenses | PF-2, PF-6, PF-7 acceptance |
| presentations | T-1 through T-8 | slide/notes parity |
| local HTML | T-1 through T-8 | accessibility and no release mechanism |
| integrated bundle | T-1 through T-8 | numerical parity and release control |

Each wave receives a round-one review, applies every P1 and P2, records P3
dispositions, and receives a round-two acceptance review before the next wave.
