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
   and readiness guardrail system, while solver inputs, rates, savings, public
   cards, and balanced-budget claims remain blocked.
4. Open [Budget-Area Funding Explainer](reading/budget-area-funding-explainer.md).
   Use it as the plain-language summary of the six-paper research program:
   health is expensive, old-age financing has a base problem, defense is a
   strategic band, family support is thin, and total revenue is low relative to
   peer systems.
5. Open [Current Versus Benchmark Scoreboard](reading/current-versus-benchmark-scoreboard.md).
   Show the evidence firewall: matched benchmark gaps, topline-only areas,
   breadth gaps, improper payments, fraud, and recoverable savings are separate
   categories.
6. Open [Payment Integrity: What The Public Evidence Shows](reading/payment-integrity-bounded-factual-examples.md).
   Show how the repo allows exact public numbers while blocking unsupported
   fraud, waste, and savings claims.
7. Open [docs/papers](papers/README.md), then start with
   `0+legible-federal-funding.pdf`. Explain that the markdown papers are the
   source of truth and the PDFs are convenience renders.
8. Open the
   [adaptive-rate wave frontier](../context/waves/2026-07-18-adaptive-rate-performance-system/WAVE.md#current-frontier).
   Show how the repo translates the research thesis into machine-readable gates
   that keep rates, savings, and balanced-budget claims blocked until evidence
   closes.
9. Run the validation command:

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
  context-only artifacts, missing evidence, and active closure gates without
  pretending a rate proposal is ready.
- Source custody through `docs/sources/source-version-ledger.md`.
- A Rust validation harness that checks the derived records and public artifact
  contracts.

## Active Implementation Frontier

The current wave is
[Adaptive Rate and Performance System](../context/waves/2026-07-18-adaptive-rate-performance-system/WAVE.md).
It is the best place to show how TAXLANE turns the policy thesis into guarded
implementation work.

Current frontier:

- Pulses 82 through 201 created rate, risk, modernization, public-card,
  pilot, simulator, current-law, lane-depth, receipt-base, distribution,
  reserve, net-interest, closure-gate, and showcase-readiness artifacts.
- The income-security/family lane now gives the cleanest concrete demo trail:
  FY2025 federal account-perimeter custody is narrowly ready, OECD comparator
  context is displayable, and CBO, Census, HHS/ACF, and USDA source gaps are
  documented.
- The transportation pilot has scaffold and partial source context, but no
  publishable simulator run, rate, target cost, savings claim, or balanced-budget
  claim.
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
