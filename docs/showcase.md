# TAXLANE Showcase Guide

This is the short path for showing TAXLANE to a first-time reader, reviewer, or
potential collaborator. It favors defensible public artifacts over the full
research backlog.

## One-Sentence Pitch

TAXLANE turns the federal income-tax debate into a source-backed visibility
system: every receipt, funding lane, benchmark, and reform proposal must label
what is current law, what is modeled allocation, what is legal dedication, and
what is still only a proposal.

## Ten-Minute Walkthrough

1. Start with [Honest Federal Tax Receipt](reading/honest-federal-tax-receipt.md).
   Point out that the receipt does not pretend income-tax dollars are legally
   tagged to every displayed category. It labels the allocation basis.
2. Open [Budget-Area Funding Explainer](reading/budget-area-funding-explainer.md).
   Use it as the plain-language summary of the six-paper research program:
   health is expensive, old-age financing has a base problem, defense is a
   strategic band, family support is thin, and total revenue is low relative to
   peer systems.
3. Open [Current Versus Benchmark Scoreboard](reading/current-versus-benchmark-scoreboard.md).
   Show the evidence firewall: matched benchmark gaps, topline-only areas,
   breadth gaps, improper payments, fraud, and recoverable savings are separate
   categories.
4. Open [Payment Integrity: What The Public Evidence Shows](reading/payment-integrity-bounded-factual-examples.md).
   Show how the repo allows exact public numbers while blocking unsupported
   fraud, waste, and savings claims.
5. Open [docs/papers](papers/README.md), then start with
   `0+legible-federal-funding.pdf`. Explain that the markdown papers are the
   source of truth and the PDFs are convenience renders.
6. Run the validation command:

   ```powershell
   cargo run -p taxlane-tools -- income-tax-outlay validate
   ```

   This checks derived records, chart specs, public packets, and claim
   guardrails.

## What Is Ready To Show

- A reader-facing tax receipt standard that distinguishes current law,
  allocation models, and reform proposals.
- Six accepted, panel-reviewed research papers with PDF renderings and markdown
  source files.
- Public packets for funding lanes, spending categories, payment integrity,
  cost-down evidence queues, and benchmark gaps.
- Source custody through `docs/sources/source-version-ledger.md`.
- A Rust validation harness that checks the derived records and public artifact
  contracts.

## What To Say Carefully

- "This is a civic visibility and reform-design repo."
- "Ordinary individual income tax is mostly a general fund receipt; TAXLANE
  models allocations unless legal dedication is cited."
- "Improper payment, fraud, and recoverable savings are different quantities."
- "Rate recommendations are reform proposals and value judgments, not current
  law and not personal tax advice."
- "AI-simulated reviewers are review lenses, not real people or endorsements."

## What Not To Claim

- Do not say TAXLANE calculates a person's tax liability.
- Do not say the receipt proves where a specific person's dollars legally went.
- Do not turn spend size into a fraud, waste, abuse, or savings finding.
- Do not call benchmark gaps recoverable savings unless the source chain proves
  recoverability, access floors, confidence, and implementation path.
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
