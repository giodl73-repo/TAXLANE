# TAXLANE Showcase

This is the shortest public handoff for TAXLANE.

TAXLANE asks a simple taxpayer question: when a federal tax receipt, budget
chart, or reform proposal shows a number, can the reader see the source, scope,
denominator, financing relationship, and claim status before being asked to
trust it?

## One-Minute Example

A conventional tax receipt can make it look like one person's income-tax
dollars were legally tagged to every displayed program. TAXLANE treats that as
too vague. The [honest federal tax receipt](docs/reading/honest-federal-tax-receipt.md)
shows the better standard: say whether a displayed amount is current law, legal
dedication, modeled allocation, civic illustration, or proposed reform.

That same rule carries into the broader research program. Health cost, old-age
financing, defense strategy, family support, payment integrity, and revenue
adequacy are different problems. TAXLANE keeps those claims separate until the
source chain supports joining them.

## Start Here

| Taxpayer question | Open this | Current status |
|---|---|---|
| What is TAXLANE ready to show today? | [Showcase readiness summary](docs/reading/taxlane-showcase-readiness-summary.md) | Demo-ready as a source-custody/readiness system; not solver-ready, rate-ready, savings-ready, or balanced-budget-ready. |
| What is the core product idea? | [Honest federal tax receipt](docs/reading/honest-federal-tax-receipt.md) | Reader-facing receipt standard with explicit allocation labels. |
| What is the public-policy thesis? | [Budget-area funding explainer](docs/reading/budget-area-funding-explainer.md) | Plain-language synthesis of the six-paper research program. |
| Which examples are safest to discuss publicly? | [Current versus benchmark scoreboard](docs/reading/current-versus-benchmark-scoreboard.md) | Separates matched benchmarks, toplines, breadth gaps, improper payments, fraud, and recoverable savings. |
| How is the evidence protected? | [Source version ledger](docs/sources/source-version-ledger.md) and `cargo run -p taxlane-tools -- income-tax-outlay validate` | Source custody, derived records, chart specs, public links, manifest hashes, and claim guardrails are checked. |

## What Is Ready

- A public receipt standard for labeling what a tax display actually means.
- Six accepted research papers, with markdown source and rendered PDFs.
- Reading packets for budget lanes, benchmark gaps, payment integrity,
  cost-down queues, and headline-number scope.
- Machine-readable readiness records that preserve nulls for blocked outputs.
- A validator that makes the evidence firewall reproducible.

## What Is Not Ready

- Personal tax calculation.
- Public rate cards.
- Solver outputs.
- Gross or net savings estimates.
- Department-cut instructions.
- Technology-savings claims.
- Balanced-budget plans.
- Legal tracing of one taxpayer's exact dollars to exact programs.

## Safe Public Language

- "TAXLANE is demo-ready as a source-backed civic visibility system."
- "The current product value is claim discipline around public fiscal numbers."
- "The adaptive-rate work is a gated implementation frontier, not a published
  rate proposal."
- "Benchmark gaps, improper payments, fraud, and recoverable savings are
  separate quantities."
- "AI-simulated reviewers are review lenses, not real people or endorsements."

## Reviewer Pack

For a five-minute walkthrough, use [docs/demo-script.md](docs/demo-script.md).
For a fuller tour, use [docs/showcase.md](docs/showcase.md). For the paper set,
open [docs/papers](docs/papers/README.md). For source custody, open
[docs/sources/source-version-ledger.md](docs/sources/source-version-ledger.md).
