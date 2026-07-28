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
| What is TAXLANE ready to show today? | [Showcase readiness summary](docs/reading/taxlane-showcase-readiness-summary.md) | Demo-ready as a source-custody/readiness system; the foundational gate remains not solver-ready, rate-ready, savings-ready, or balanced-budget-ready for official outputs. The later REV-F surface adds a public model planning rate card while official/formal certification remains blocked. |
| What is the core product idea? | [Honest federal tax receipt](docs/reading/honest-federal-tax-receipt.md) | Reader-facing receipt standard with explicit allocation labels. |
| What is the public-policy thesis? | [Budget-area funding explainer](docs/reading/budget-area-funding-explainer.md) | Plain-language synthesis of the six-paper research program. |
| Which examples are safest to discuss publicly? | [Current versus benchmark scoreboard](docs/reading/current-versus-benchmark-scoreboard.md) | Separates matched benchmarks, toplines, breadth gaps, improper payments, fraud, and recoverable savings. |
| Are all 15 lanes fully covered? | [Lane full coverage matrix](docs/reading/lane-full-coverage-matrix.md) and [Wave C public explainer promotion](docs/reading/public-explainer-wave-c-promotion.md) | Public explainers are complete for all 15 lanes; current-law baseline and source custody are partial for all 15 lanes; transition models are partial for 2 lanes and missing for 13; solver mapping is partial for 7 lanes and missing for 8; receipt/rate bridges are partial for 4 lanes and missing for 11. No lane is fully complete, solver-ready, rate-ready, savings-ready, or balanced-budget-ready. |
| Are outcome floors ready for lower-cost scenarios? | [Wave D floor-value readiness](docs/reading/outcome-floor-wave-d-value-readiness.md) | Wave D anchors are complete for all 15 lanes, but complete component floors and lower-cost admissibility remain blocked. |
| Are policy scenario packs ready? | [Wave E scenario-pack readiness](docs/reading/lane-scenario-pack-wave-e-readiness.md) | Reference calibrations are ready for all 15 lanes; real reform scenarios, federal effects, and lower-cost admissibility remain blocked. |
| Is the deterministic simulator calibrated, and are public rates ready? | [Wave F calibration](docs/reading/wave-f-transportation-deterministic-calibration.md) and [solver/rate readiness](docs/reading/solver-rate-wave-f-readiness.md) | Wave F's transportation calibration is complete and dry-run-ready. Zero substantive prerequisites are ready, so official solver outputs, public rates, cards, savings, and balanced-budget claims remain blocked. |
| What did the spending-and-rate analysis conclude? | [targeted spending-rate decision](docs/reading/targeted-spending-rate-decision.md), [completed rate analysis](docs/reading/rev-internal-rate-analysis-completion.md), and [multi-track frontier](docs/reading/adaptive-rate-multi-track-frontier.md) | Taxlane tested specific HLT imaging and DEF force-strategy candidates. Each retains six unresolved gates, so zero FY2026 savings is admitted and 21/23/33/35/43/46/48 remains the preferred central analytical schedule; 22/24/34/36/44/47/49 remains the behavior-robust contingency. |
| Are we filling source gaps? | [Eight-gap acquisition status](docs/reading/data-acquisition-eight-gap-status.md), [OMB AP13 fund-group detail](docs/reading/omb-ap13-fund-group-reconciliation-detail-fy2025-context.md), [Net-interest MSPD maturity detail](docs/reading/net-interest-treasury-mspd-maturity-detail-context.md), [Social Security source-capture rollup](docs/reading/social-security-source-capture-status-rollup.md), [OASDI FY2025-FY2035 path](docs/reading/social-security-oasdi-fy2025-2035-current-law-path.md), [taxable payroll base bridge](docs/reading/social-security-taxable-payroll-base-bridge.md), [receipt-yield boundary](docs/reading/social-security-oasdi-receipt-yield-boundary.md), [CBO FY2032-FY2035 extension context](docs/reading/cbo-open-data-fy2032-2035-current-law-extension-context.md), [CBO FY2032-FY2035 category context](docs/reading/cbo-major-outlay-category-fy2032-2035-context.md), [CBO FY2026-FY2035 revenue-detail context](docs/reading/cbo-revenue-detail-fy2026-2035-context.md), [IRS SOI TY2023 individual-income base context](docs/reading/irs-soi-pub1304-ty2023-individual-income-base-context.md), [OMB FY2025-FY2031 receipt-category context](docs/reading/omb-receipt-category-fy2025-2031-context.md), [OMB Table 2.4 FY2025-FY2031 receipt-detail context](docs/reading/omb-receipt-detail-table-2-4-fy2025-2031-context.md), [OMB Table 2.2 FY2025-FY2031 receipt-share context](docs/reading/omb-receipt-share-table-2-2-fy2025-2031-context.md), and [OMB amount/share reconciliation](docs/reading/omb-receipt-amount-share-reconciliation-fy2025-2031-context.md) | Yes, narrow gaps are moving forward: official SSA OASDI context, taxable-payroll base context, an OMB/SSA receipt boundary, official CBO FY2032-FY2035 top-line/debt/trust-fund context, CBO major outlay-category context, CBO FY2026-FY2035 receipt-category context, OMB AP13 FY2025 fund-group detail, IRS SOI TY2023 AGI/taxable-income context, OMB FY2025-FY2031 receipt context, CMS quality/access lineage context, and Treasury rate/debt/maturity-detail context are verified, while CBO spreadsheet raw custody, general-fund annual paths, OASI/DI solver paths, matched FY2025 receipt bases, yield reconciliation, floors, solver inputs, rates, and savings remain blocked. |
| How is the evidence protected? | [Source version ledger](docs/sources/source-version-ledger.md) and `cargo run -p taxlane-tools -- income-tax-outlay validate` | Source custody, derived records, chart specs, public links, manifest hashes, and claim guardrails are checked. |

## What Is Ready

- A public receipt standard for labeling what a tax display actually means.
- Six accepted research papers, with markdown source and rendered PDFs.
- Reading packets for budget lanes, benchmark gaps, payment integrity,
  cost-down queues, and headline-number scope.
- Machine-readable readiness records that preserve nulls for blocked outputs.
- A validator that makes the evidence firewall reproducible.

## What Is Not Ready

The next-work surface is now concrete: every one of the fifteen F starts has an
audited decision and a two-level advancement queue. TRN completed F only as a
typed cost note; the other fourteen starts remain blocked. REV Level 1 owns the
active legal/economic base bridge.

The subsequent internal two-level wave is now complete for all fifteen tracks.
No spending candidate entered the FY2026 package. The PAY-NET-REV rerun leaves
the revenue need at $813.727 billion and retains the 21/23/33/35/43/46/48
planning schedule. The next frontier is internal: score a candidate grid,
reconcile PAY-NET-REV, revisit affected track dependencies, and select a
Taxlane analytical recommendation. No Legislative Counsel, JCT, CBO, or other
official request is planned or required.

CORE-M is complete as the shared candidate-dossier and typed-release layer.
The demo can show why a cost-only TRN candidate, PAY/REV overlays, and NET
endogeneity admit different outputs without weakening any evidence gate.
H.R. 2247 is now the selected TRN cost-only candidate; this remains distinct
from a lower-cost, rate, or savings proposal.

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
