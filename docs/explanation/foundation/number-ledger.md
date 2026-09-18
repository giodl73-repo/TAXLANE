# EXPL-A number ledger

This is the canonical numerical handoff for explanation artifacts. Display
rounding must not change the stored value or its meaning.

| ID | Value | Unit / basis | Meaning | Canonical evidence |
|---|---:|---|---|---|
| NUM-01 | 15 | tracks | canonical tracks with terminal dispositions | `fifteen_track_terminal_disposition.v1.draft.json` |
| NUM-02 | 10 | tracks | reviewed zero-admission dispositions | `fifteen_track_terminal_disposition.v1.draft.json` |
| NUM-03 | 0.000 | FY2026 $ billions | admitted primary spending reduction | terminal disposition; targeted decision |
| NUM-04 | 813.727 | FY2026 $ billions | frozen remaining ordinary-income rate-model target; not total receipts/outlays or a trust-fund requirement | `rev_internal_rate_analysis_completion.v1.draft.json` |
| NUM-05 | 14 | candidate schedules | uniform-uplift schedules tested | `rev_internal_rate_analysis_completion.v1.draft.json` |
| NUM-06 | 42 | behavior cases | 14 schedules × 3 taxable-income response cases | `rev_internal_rate_analysis_completion.v1.draft.json` |
| NUM-07 | 11.0 | percentage points | preferred central uniform uplift | `rev_internal_rate_analysis_completion.v1.draft.json` |
| NUM-08 | 21/23/33/35/43/46/48 | marginal model bracket-rate percentages | preferred central analytical schedule | `rev_internal_rate_analysis_completion.v1.draft.json` |
| NUM-09 | 819.220 | FY2026 $ billions | preferred schedule's central first-year cash proxy | `rev_internal_rate_analysis_completion.v1.draft.json` |
| NUM-10 | 0.077 | FY2026 $ billions | administration ceiling; not a complete compliance-burden model | `rev_internal_rate_analysis_completion.v1.draft.json` |
| NUM-11 | 5.416 | FY2026 $ billions | one-year central model gap; not formal balance | `rev_internal_rate_analysis_completion.v1.draft.json` |
| NUM-12 | 12.0 | percentage points | behavior-robust contingency uplift | `rev_internal_rate_analysis_completion.v1.draft.json` |
| NUM-13 | 22/24/34/36/44/47/49 | marginal model bracket-rate percentages | behavior-robust contingency schedule | `rev_internal_rate_analysis_completion.v1.draft.json` |
| NUM-14 | 12.6 | percentage points | severe internal stress uplift | `rev_internal_rate_analysis_completion.v1.draft.json` |
| NUM-15 | 22.6/24.6/34.6/36.6/44.6/47.6/49.6 | marginal model bracket-rate percentages | severe internal stress ceiling schedule | `rev_internal_rate_analysis_completion.v1.draft.json` |
| NUM-16 | 0.4 | FY2026 $ billions | HLT headline context, not admitted savings | `targeted_spending_rate_decision.v1.draft.json` |
| NUM-17 | 15.0 | FY2026 $ billions | DEF headline context, not admitted savings | `targeted_spending_rate_decision.v1.draft.json` |
| NUM-18 | 6 each | unresolved gates | HLT and DEF blocked gates | `targeted_spending_rate_decision.v1.draft.json` |
| NUM-19 | 3.094 | FY2026 $ billions | severe-tier worst-case model gap | `rev_internal_rate_analysis_completion.v1.draft.json` |
| NUM-20 | 0.774223895 | ratio | full-year output to first-year cash-proxy realization | `rev_internal_analysis_baseline_freeze.v1.draft.json` |
| NUM-21 | 0.15 / 0.25 / 0.35 | substitution-elasticity cases | taxable-income response assumptions; not forecasts | `rev_internal_rate_analysis_completion.v1.draft.json` |
| NUM-22 | 0 / -2.5 / -5 | percent | adverse internal macro stresses; not forecasts | `rev_internal_rate_analysis_completion.v1.draft.json` |
| NUM-23 | 10/12/22/24/32/35/37 | marginal model bracket-rate percentages | TY2026 current-law ordinary-income schedule used by the profile comparison | `rev_level_7_scorer_ready_legislative_specification.v1.draft.json` |
| NUM-24 | 11.0 | percentage points | uniform difference between the current-law and preferred Taxlane ordinary-income schedules; with unchanged thresholds, the narrow bracket-only profile difference equals 11% of taxable ordinary income | scorer-ready specification; `taxpayer_profile_scenarios.ty2026.v1.draft.json` |
| NUM-25 | 6 | illustrative profiles | bracket-only single and married-filing-jointly examples; not representative households or complete returns | `taxpayer_profile_scenarios.ty2026.v1.draft.json` |
| NUM-26 | 6 | countries | repository-captured OECD COFOG composition columns used by the local comparison; U.S. is partial because GF05 is missing | `cofog_site_spending_share_calculation.data2022.v1.draft.json` |
| NUM-27 | 139 / 513 / 572 / 490 / 10 / 198 | repository inventory | wave directories / pulse files / derived JSON or JSONL records / reading documents / research papers / review files after the adaptive-needs Pennsylvania state-screen extension; operational metadata, not impact | `repository_corpus_inventory_snapshot.2026-07-27.v1.draft.json` |
| NUM-28 | 4 | conditional profiles | focused, balanced, ambitious, and transformative peer-informed savings/reinvestment profiles | `peer_informed_savings_and_reinvestment_profiles.ty2026.v1.draft.json` |
| NUM-29 | 150 / 300 / 450 / 600 | FY2026 $ billions | assumed net-savings goals after reinvestment; policy envelopes, not estimates or admitted savings | peer-informed profiles |
| NUM-30 | 663.727 / 513.727 / 363.727 / 213.727 | FY2026 $ billions | conditional remaining ordinary-income targets after the four assumed net-savings goals | peer-informed profiles |
| NUM-31 | 8.9 / 6.8 / 4.8 / 2.8 | percentage points | smallest tested tenth-point central-model uplift clearing each conditional remaining target | peer-informed profiles |
| NUM-32 | 18.9/20.9/30.9/32.9/40.9/43.9/45.9; 16.8/18.8/28.8/30.8/38.8/41.8/43.8; 14.8/16.8/26.8/28.8/36.8/39.8/41.8; 12.8/14.8/24.8/26.8/34.8/37.8/39.8 | marginal model bracket-rate percentages | conditional central schedules for the four savings/reinvestment profiles | peer-informed profiles |
| NUM-33 | 75 / 150 / 250 / 350 | FY2026 $ billions | adaptive reinvestment reserves for focused, balanced, ambitious, and transformative profiles; not allocated to lanes | adaptive needs and balance cycle |
| NUM-34 | 15 / 0 / 0 | tracks / ranked need scores / reserve allocations | first adaptive-needs triage coverage; every track classified, none ranked or funded | adaptive needs all-lane triage |
| NUM-35 | 10 / 10 / 30 | admission gates / contingency percent / maximum single-lane share of allocatable reserve percent | adaptive allocation controls; normative planning rules, not empirical findings | adaptive needs scoring and allocation contract |
| NUM-36 | 75 / 150 / 250 / 350 | FY2026 $ billions | reserves retained unallocated after the first evidence-gate audit | adaptive profile reserve allocation readiness |
| NUM-37 | 68,207 / 1.19 and 39,254 / 14.3 and 10.35M | claims / roadway fatalities per 100M VMT and deaths / percent and children | source-custodied diagnostic anchors for the VET, TRN, and ISF dossiers; not intervention effects | adaptive initial intervention dossiers |
| NUM-38 | 20,130.978409 / 4,595.868561 / 15,535.109848 | Pennsylvania state-owned screening miles | complete same-basis PennDOT rural, two-way, two-lane, FHWA-class 3–6 denominator / existing-treatment subtraction / untreated screening remainder; not design-eligible mileage | `trn_penndot_state_owned_untreated_screen.v1.draft.json` |
| NUM-39 | 0.059033 | historical mechanical product, $ billions | untreated screening miles multiplied by a historical $3,800-per-mile input; not a current candidate cost, budget request, allocation, or upper bound | `trn_penndot_state_owned_untreated_screen.v1.draft.json` |
| NUM-40 | 2,577.521402 / 12,901.123674 / 55.676515 / 0.788258 | Pennsylvania untreated screening miles | both current paved widths at least four feet / both reported with one or both below four feet / missing either side record / ambiguous or missing current width | `trn_penndot_state_owned_untreated_screen.v1.draft.json` |
| NUM-41 | 1,631.432386 / 1,163.282197 | Pennsylvania engineering-record review miles | both reported paved widths at least four feet intersected with good/excellent OPI / intersected with 2021–2026 surface year; field-priority queues, not design eligibility | `trn_penndot_state_owned_untreated_screen.v1.draft.json` |

## Display rules

1. Show dollars with their fiscal year and billions/millions basis.
2. Never show NUM-16 or NUM-17 without “not admitted” in the same visual or
   sentence.
3. Never describe NUM-08, NUM-13, or NUM-15 as an effective rate, average rate,
   individual liability, current law, or official score.
4. Label NUM-09 as a model cash proxy and NUM-11 as a model gap.
5. Preserve three decimals for the target and central cash proxy when comparing
   them; plain-language surfaces may say “about $814B” only if the exact amount
   is directly available.
6. Test counts are operational metadata and must be refreshed from the current
   validation run rather than frozen here.
7. Every table or graphic carries the July 27, 2026 Taxlane analysis vintage and
   routes to the exact derived record.
8. Never describe NUM-24 or NUM-25 as a complete household liability, effective
   rate, gross-income example, personal estimate, or official score.
9. Never convert NUM-26 into U.S. savings, rates, efficiency rankings, or
   fairness findings without a matched-year crosswalk and an admitted policy
   candidate. Render U.S. GF05 as missing, not zero.
10. Treat NUM-27 as refreshable corpus inventory only. File counts do not prove
    correctness, impact, adoption, or endorsement.
11. Never display NUM-29 through NUM-32 without “conditional,” “goal,” or
    equivalent language in the same table or reading context. The rates are
    modeled implications; the spending amounts remain unscored and unadmitted.
12. Keep NUM-38's state-owned RMS universe distinct from the all-public HPMS
    screen. Label its untreated value as screening mileage, not design-eligible
    mileage, and never present NUM-39 as a current cost or funding amount.
13. Describe NUM-40 as reported current-paved-width availability. Four reported
    feet are not verified usable clear width beyond a proposed strip, and none
    of the bands alone establishes eligibility or ineligibility.
14. Never call NUM-41 eligible mileage. OPI and surface year prioritize missing
    engineering-record work but do not establish pavement structure, remaining
    life, programmed-project status, or approval.
