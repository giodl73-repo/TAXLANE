# TAXLANE Showcase Guide

This is the short path for showing TAXLANE to a first-time reader, reviewer, or
potential collaborator. It favors defensible public artifacts over the full
research backlog.

For the shortest public handoff, start with [SHOWCASE.md](../SHOWCASE.md).
For a live walkthrough, use the companion [demo script](demo-script.md).
For the current machine-readable showable-state summary, start with
[Taxlane Showcase Readiness Summary](reading/taxlane-showcase-readiness-summary.md).

## One-Sentence Pitch

TAXLANE turns the federal income-tax debate into a source-backed visibility
system: every receipt, funding lane, benchmark, and reform proposal must label
what is current law, what is modeled allocation, what is legal dedication, and
what is still only a proposal.

## Ten-Minute Walkthrough

1. Start with [SHOWCASE.md](../SHOWCASE.md). Use the one-minute example and the
   taxpayer-question table to make the project understandable before opening
   implementation records.
2. Open [Honest Federal Tax Receipt](reading/honest-federal-tax-receipt.md).
   Point out that the receipt does not pretend income-tax dollars are legally
   tagged to every displayed category. It labels the allocation basis.
3. Open [Taxlane Showcase Readiness Summary](reading/taxlane-showcase-readiness-summary.md).
   Use it to set the current status: TAXLANE is demo-ready as a source-custody
   and readiness guardrail system. Taxlane now has a completed independent
   analytical rate recommendation, while official rates, savings, and
   balanced-budget claims remain blocked.
4. Open [Lane Full Coverage Matrix](reading/lane-full-coverage-matrix.md).
5. Open [Data Acquisition Eight Gap Status](reading/data-acquisition-eight-gap-status.md).
   Show the nine full-coverage gates across all 15 lanes and point out that
   Wave C has completed the public-explainer gate for all lanes, while
   transportation is only the deepest pilot, not a complete solver/rate lane.
5. Open [Budget-Area Funding Explainer](reading/budget-area-funding-explainer.md).
   Use it as the plain-language summary of the six-paper research program:
   health is expensive, old-age financing has a base problem, defense is a
   strategic band, family support is thin, and total revenue is low relative to
   peer systems.
6. Open [Current Versus Benchmark Scoreboard](reading/current-versus-benchmark-scoreboard.md).
   Show the evidence firewall: matched benchmark gaps, topline-only areas,
   breadth gaps, improper payments, fraud, and recoverable savings are separate
   categories.
7. Open [Payment Integrity: What The Public Evidence Shows](reading/payment-integrity-bounded-factual-examples.md).
   Show how the repo allows exact public numbers while blocking unsupported
   fraud, waste, and savings claims.
8. Open [docs/papers](papers/README.md), then start with
   `0+legible-federal-funding.pdf`. Explain that the markdown papers are the
   source of truth and the PDFs are convenience renders.
9. Open the
   [adaptive-rate wave frontier](../context/waves/2026-07-18-adaptive-rate-performance-system/WAVE.md#current-frontier).
   Show how the repo translates the research thesis into machine-readable gates
   that keep rates, savings, and balanced-budget claims blocked until evidence
   closes.
10. Run the validation command:

   ```powershell
   cargo run -p taxlane-tools -- income-tax-outlay validate
   ```

   This checks derived records, chart specs, public packets, and claim
   guardrails.

## What Is Ready To Show

- A reader-facing tax receipt standard that distinguishes current law,
  allocation models, and reform proposals.
- A live [demo script](demo-script.md) that turns the artifact sequence into a
  five- or fifteen-minute walkthrough.
- Six accepted, panel-reviewed research papers with PDF renderings and markdown
  source files.
- Public packets for funding lanes, spending categories, payment integrity,
  cost-down evidence queues, and benchmark gaps.
- A compact showcase readiness summary that says what is demo-ready and what is
  still blocked.
- An adaptive-rate operating frontier that names current source custody,
  context-only artifacts, and completed internal rate evidence without
  presenting the analytical schedule as law or an official score.
- Source custody through `docs/sources/source-version-ledger.md`.
- A Rust validation harness that checks the derived records and public artifact
  contracts.

## Active Implementation Frontier

The current wave is
[Adaptive Rate and Performance System](../context/waves/2026-07-18-adaptive-rate-performance-system/WAVE.md).
It is the best place to show how TAXLANE turns the policy thesis into guarded
implementation work.

Current frontier:

- The controlling corpus plan separates extensible shared `CORE-*`
  infrastructure from the repeatable `TRN-A` through `TRN-F` transportation
  recipe. CORE-G through CORE-N and TRN-A through TRN-F are complete. TRN-C
  closes with a conditional cost-only H.R. 2247 score. TRN-D-01, HLT-A-01,
  and EDU-A-01 are complete, as are TRN-D-03, HLT-A-02, and EDU-A-02. The
  TRN-D-04, HLT-A-03, and EDU-A-03 financing/incidence slice is complete. The
  TRN-D-05/06, HLT-A-04/05, and EDU-A-04/05 two-level bundle is also complete;
  CORE-N and TRN-F are complete. Fourteen named tracks remain at bounded E.
  H.R. 2247 is published only as a conditional cost note. REV Level 1 is active
  at the unmatched legal/economic base bridge; zero solver runs or rate
  calculations occurred. PAY and REV remain non-additive overlays and NET
  remains endogenous.
- Pulses 82 through 201 created rate, risk, modernization, public-card,
  pilot, simulator, current-law, lane-depth, receipt-base, distribution,
  reserve, net-interest, closure-gate, and showcase-readiness artifacts.
- The income-security/family lane now gives the cleanest concrete demo trail:
  FY2025 federal account-perimeter custody is narrowly ready, OECD comparator
  context is displayable, and CBO, Census, HHS/ACF, and USDA source gaps are
  documented.
- The Social Security lane now has official SSA Trustees source progress:
  browser-verified 2026 OASDI context values, table locations, and a bounded
  FY2025-FY2035 combined OASDI fiscal-year path, plus CY2025-CY2035 taxable
  payroll and wage-base context. The OMB FY2025 receipt anchor is compared to
  SSA CY2025 taxable-payroll yield context as a boundary only, and the source-
  capture rollup now marks current-law baseline and receipt/rate bridge gates
  partial. Local raw-byte custody, OASI/DI split paths, fiscal receipt
  reconciliation, solver/rate/savings claims are still blocked.
- CBO official open data now supplies FY2032-FY2035 top-line budget, debt,
  net-interest, selected trust-fund balance, and major outlay-category context,
  plus FY2026-FY2035 revenue-detail context. This reduces real source gaps, but
  it is not an OMB 17-row lane ledger, not a Taxlane lane baseline, not matched
  receipt-base evidence, and not solver input.
- IRS SOI Publication 1304 Table 1.1 TY2023 now supplies source-custodied
  individual-income AGI, taxable-income, and income-tax-after-credits context.
  The IRS listing check found Table 1.1 listed through TY2023; this is not a
  matched FY2025 assigned base, rate bridge, or solver input.
- OMB Historical Table 2.1 now supplies FY2025-FY2031 receipt-category context
  for individual income taxes, corporation income taxes, social insurance,
  excise taxes, other receipts, and total receipts. This is fiscal receipt
  context only, not assigned bases, rate denominators, or solver input.
- OMB Historical Table 2.4 now supplies FY2025-FY2031 social-insurance,
  retirement, and excise receipt detail, including OASDI, Hospital Insurance,
  transportation excise, and airport/airway excise context. This is still fiscal
  receipt context only, not taxable payroll, HI income split, statutory
  user-fee bases, rates, or solver input.
- OMB Historical Table 2.2 now supplies FY2025-FY2031 receipt-source share
  context for the same major receipt families. This is percentage composition
  context only, not receipt amounts, bases, rates, or solver input.
- An OMB Table 2.1/Table 2.2 amount/share reconciliation now verifies that
  FY2025-FY2031 receipt amounts and one-decimal receipt shares align within
  rounding tolerance. This is diagnostic context, not base, yield, rate, or
  solver evidence.
- The transportation pilot has scaffold and partial source context, but no
  publishable simulator run, rate, target cost, savings claim, or balanced-budget
  claim.
- Wave C has completed public explainers for all 15 analytical lanes, but it
  does not complete current-law paths, floor values, policy scenarios, solver
  mapping, receipt/rate bridges, savings, or balanced-budget claims.
- The lane full coverage matrix now exposes aggregate gate counts: current-law
  baseline and source custody are partial for all 15 lanes; transition models
  are partial for 2 lanes and missing for 13; solver mapping is partial for 7
  lanes and missing for 8; receipt/rate bridges are partial for 4 lanes and
  missing for 11.
- Wave D has completed one source-custodied threshold and baseline anchor for
  all 15 lanes; complete component floors and lower-cost admissibility remain
  blocked.
- Wave E has 15 role-reviewed current-policy reference calibrations with
  identity-projected policy values, synthetic adverse stress values, and
  deterministic comparator results. No real reform scenario, federal effect,
  solver input, rate, savings, or balanced-budget claim is ready.
- Wave F has a solver/rate blocker audit: zero prerequisites are ready for a
  deterministic dry run, solver output, public rate, public card, savings, or
  balanced-budget claim.
- The active income-security/family queue remains source-capture closure:
  CBO baseline/take-up, Census child poverty/income, childcare, food/nutrition,
  federal/state/local translation, and broader international comparator lineage
  remain open.

## What To Say Carefully

- "TAXLANE is demo-ready as a source-backed civic visibility system."
- "This is a civic visibility and reform-design repo."
- "Ordinary individual income tax is mostly a general fund receipt; TAXLANE
  models allocations unless legal dedication is cited."
- "Improper payment, fraud, and recoverable savings are different quantities."
- "Rate recommendations are reform proposals and value judgments, not current
  law and not personal tax advice."
- "The adaptive-rate system is a gated implementation surface; the current
  frontier is source lineage and readiness, not published rates."
- "Taxlane is demo-ready as a readiness system, not as a rate or savings model."
- "AI-simulated reviewers are review lenses, not real people or endorsements."

## What Not To Claim

The readiness walkthrough can now show the common ten-gate F contract, fifteen
audited start decisions, and two advancement levels per track. It must describe
TRN-F as a completed cost note, not a rate card, and REV Level 1 as unmatched
context, not a calculated rate.

It can also show that the later internal two-level wave completed for all
fifteen tracks and that the PAY-NET-REV reconciliation preserved zero admitted
FY2026 spending reduction. The 21/23/33/35/43/46/48 schedule remains a
Taxlane analytical result, not enacted law or an official score. The next work
tests a rate grid and returns through PAY-NET-REV and affected track
dependencies; no official request or external certification is planned.

It can also show CORE-M's typed candidate dossiers: cost-only modernization,
revenue instruments, non-additive integrity overlays, and endogenous fiscal
effects no longer share a false universal output profile.

- Do not say TAXLANE calculates a person's tax liability.
- Do not say the receipt proves where a specific person's dollars legally went.
- Do not turn spend size into a fraud, waste, abuse, or savings finding.
- Do not call benchmark gaps recoverable savings unless the source chain proves
  recoverability, access floors, confidence, and implementation path.
- Do not imply the adaptive-rate wave has produced public rates, solver outputs,
  savings, or a balanced-budget plan.
- Do not treat the paper PDFs as separate edited publications; markdown remains
  the source tier.

## Best First Issues

- Add a missing primary-source row to `docs/sources/source-version-ledger.md`.
- Improve a reading packet by tightening claim labels and source IDs.
- Add a focused depth card for a budget area that still has a breadth gap.
- Reproduce a derived artifact and report whether validation passes.
- Draft a role-review note for a taxpayer-facing display.

Use [CONTRIBUTING.md](../CONTRIBUTING.md) before opening an issue or pull
request.
