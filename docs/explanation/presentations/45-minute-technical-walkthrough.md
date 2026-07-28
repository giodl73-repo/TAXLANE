---
title: "Taxlane: a forty-five-minute technical walkthrough"
subtitle: "Repository-only briefing source"
date: "2026-07-27"
---

# Scope and authority

An internal evidence-and-accounting system for fifteen federal-purpose tracks.
The output is analytical and repository-only—not enacted law, personal advice,
an official score, formal balance, or authority to release. [CLM-01, CLM-02]

Speaker note: Keep descriptive findings separate from normative choices.

---

# Canonical result

15 tracks; 10 reviewed zero admissions; **$0.000B** admitted FY2026 primary
reduction; **$813.727B** remaining scoped ordinary-income target; preferred
marginal schedule **21/23/33/35/43/46/48**. [NUM-01–NUM-04, NUM-08]

Speaker note: The target is neither total receipts nor total outlays.

---

# Evidence-to-rate pipeline

Candidate → fiscal object → current source → netting/overlap → beneficiary and
service floors → implementation/compliance → admission → annual debt path →
NET → remaining REV target → behavior/stress model.

Speaker note: A missing gate produces zero admission, not a guessed haircut.

---

# Admission semantics

“Reviewed zero” is a terminal conclusion under current evidence. “Conditional
cost note,” “solvency overlay,” “measurement overlay,” “financing rail,” and
“endogenous result” are distinct types. [CLM-03, CLM-04]

Speaker note: Do not coerce these types into a single savings column.

---

# Fifteen-track matrix I

| Track | Purpose | Result | Reopening class |
|---|---|---|---|
| TRN | mobility/infrastructure | conditional cost note | enactment/appropriation |
| HLT | access/quality/health finance | reviewed zero | access, quality, behavior, overlap |
| EDU | learning/workforce | reviewed zero | current score, access, borrowing |
| OAS | retirement solvency/adequacy | separate overlay | trust-fund/cohort/adequacy |
| ISF | food/family stability | reviewed zero | hardship, take-up, burden |
| VET | earned benefits/care | reviewed zero | current net cost and beneficiary path |
| AGR | food system/resilience | reviewed zero | producer, access, disaster floors |

Speaker note: All numeric package credits here are zero or not applicable.

---

# Fifteen-track matrix II

| Track | Purpose | Result | Reopening class |
|---|---|---|---|
| DEF | readiness/deterrence | reviewed zero | FYDP/readiness/posture/score |
| DIS | mitigation/recovery | reviewed zero | affordability/equity/exposure |
| JUS | law/rights/safety | reviewed zero | rights/victim/local allocation |
| SEE | knowledge/energy/environment | reviewed zero | delivery/emissions/resilience |
| INT | diplomacy/humanitarian/security | reviewed zero | region/purpose/security effects |
| PAY | cross-owner controls | non-additive overlay | owner-attributed net evidence |
| REV | remaining financing | analytical recommendation | target/model/objective change |
| NET | debt cost | endogenous | admitted annual debt path |

Speaker note: See the guide and machine record for full trigger wording.

---

# PAY: measurement is not booking

PAY may identify error or control weakness. A program owner books only a scored,
net, non-overlapping effect after appeals, burden, implementation, distribution,
and service evidence. [CLM-05, CLM-10]

Speaker note: This blocks improper-payment and gross-error conflation.

---

# NET: debt effects are endogenous

No interest saving is booked from a headline proposal. NET changes only after
an admitted annual primary path changes debt and the debt-cost calculation is
rerun. [CLM-05, CLM-07]

Speaker note: FY2026 primary arithmetic is not a long-run debt certification.

---

# REV: a scoped residual

REV receives the frozen **$813.727B** ordinary-income model target after the
spending gate admits no FY2026 primary reduction. The target excludes claims
about total federal balance and dedicated OAS solvency. [NUM-03, NUM-04]

Speaker note: Changing the objective can change the target; that is a normative
decision, not a hidden model finding.

---

# Model vintage and test grid

Tax-Calculator 6.5.1; bundled CPS tax-unit input; tax year 2026. Fourteen
uniform-uplift candidates × three taxable-income response cases = 42 behavior
cases. [NUM-05, NUM-06]

Speaker note: The bundled microdata are tax units, not a complete household
incidence or distribution study.

---

# Central schedule

11.0-point uplift → **21/23/33/35/43/46/48** marginal bracket percentages.
Central cash proxy: **$819.220B**. Administration ceiling: **$0.077B**. Reported
one-year model gap: **$5.416B**. [NUM-07–NUM-11]

Speaker note: Preserve the artifact's gap convention; none of these values is
an official score or proof of balance.

---

# Behavior contingency

12.0-point uplift → **22/24/34/36/44/47/49**. This rail clears the modeled
taxable-income response cases; it is a contingency, not the preferred central
recommendation. [NUM-12, NUM-13]

Speaker note: Modeled response is not observed future behavior.

---

# Severe internal stress ceiling

12.6-point uplift → **22.6/24.6/34.6/36.6/44.6/47.6/49.6**. The severe-tier
worst-case gap is **$3.094B** under the artifact's convention. [NUM-14, NUM-15,
NUM-19]

Speaker note: A stress ceiling is not a statutory recommendation.

---

# Marginal-rate interpretation

Each percentage applies only within its model bracket. The top number is not an
average or effective rate and does not apply to all income. [CLM-06]

Speaker note: The analysis does not claim complete household distribution,
incidence, filing complexity, or compliance burden.

---

# Service-continuity and public-purpose floor

A candidate that affects beneficiaries or legal commitments needs an explicit
shortfall rule, continuity path, timing, and distribution boundary before an
amount can be admitted. [CLM-09]

Speaker note: Fiscal discipline and service protection are joint gates.

---

# Known limitations

- first-year result, not a ten-year official score;
- no complete trust-fund or debt-sustainability proof;
- no full household distribution/incidence study;
- no complete taxpayer compliance-burden model;
- analytical adaptation, not legal automation; and
- source vintages must be refreshed before reopening. [CLM-07, CLM-10–CLM-13]

Speaker note: Limits are part of the result, not footnotes to erase.

---

# What would change the result?

Current enacted text or scores; closed owner, beneficiary, continuity,
distribution, overlap, and implementation gates; an admitted annual debt path;
changed upstream effects or model inputs; or a changed policy objective.
[CLM-08]

Speaker note: Each trigger returns to the relevant gate and review round.

---

# Reproduction routes

1. Claims: `docs/explanation/foundation/claim-ledger.md`
2. Numbers: `docs/explanation/foundation/number-ledger.md`
3. Track record: `data/derived/breadth_benchmark_matrix/fifteen_track_terminal_disposition.v1.draft.json`
4. Rate record: `data/derived/breadth_benchmark_matrix/rev_internal_rate_analysis_completion.v1.draft.json`
5. Papers: `research/publications/*/paper.md`

Speaker note: Markdown and machine records are canonical; slides and PDFs are
convenience views.

---

# Repository-only close

The corpus may be reviewed, tested, and rendered locally. It may not be
deployed, announced, transmitted, submitted, or represented as endorsed without
new explicit owner authorization. [CLM-13]

Speaker note: Stop at repository readiness.
