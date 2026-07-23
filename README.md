# TAXLANE

TAXLANE is a public-policy knowledge repo about the income tax: how it works
today, how it came to exist, what taxes are supposed to fund, and whether the
single income-tax label should be replaced or explained as a set of distinct,
program-linked taxes.

## Show TAXLANE First

If you are evaluating the project, start here:

1. Read the [Taxlane showcase readiness summary](docs/reading/taxlane-showcase-readiness-summary.md)
   for the current showable state: demo-ready as a source-custody and readiness
   guardrail system, not solver-ready or rate-ready.
2. Read the [showcase guide](docs/showcase.md) for a 10-minute walkthrough.
3. Use the [demo script](docs/demo-script.md) when presenting the repo live.
4. Read the [honest federal tax receipt](docs/reading/honest-federal-tax-receipt.md)
   to see the core before/after product idea.
5. Read the [budget-area funding explainer](docs/reading/budget-area-funding-explainer.md)
   for the paper-backed public-policy thesis.
6. Check the active [adaptive-rate wave frontier](context/waves/2026-07-18-adaptive-rate-performance-system/WAVE.md#current-frontier)
   to see how the repo turns the thesis into guarded machine-readable gates.
7. Open the compiled paper set in [docs/papers](docs/papers/README.md), starting
   with `0+legible-federal-funding.pdf`.
8. Run `cargo run -p taxlane-tools -- income-tax-outlay validate` to check the
   derived records, chart specs, public packet links, and claim guardrails.

TAXLANE is ready to show as a source-backed civic research artifact, not as tax,
legal, accounting, investment, or national-security advice. The public promise is
legibility: show the payer base, beneficiary base, financing relationship,
denominator, and claim status before asking a reader to trust a tax receipt,
funding lane, or reform proposal.

## The Case In One Page

A taxpayer receipt should not merely turn one income-tax payment into a tidy
list of programs. It should disclose what goes in, who benefits, how each lane
is financed, which denominator is used, and whether the displayed amount is
legally dedicated, modeled, or illustrative.

The flagship [honest federal tax receipt](docs/reading/honest-federal-tax-receipt.md)
shows the difference. It splits Medicare Part A from Parts B and D, treats
defense as a collective public good, identifies interest as the cost of past
borrowing, and keeps redistribution distinct from personal pay-in/pay-out.

The immediate proposal is truthful labeling on public tax receipts. Formal
program-linked income taxes remain a separate design hypothesis, not a result
the repo assumes.

For the portfolio-wide evidence check, use the
[current-versus-benchmark scoreboard](docs/reading/current-versus-benchmark-scoreboard.md).
It shows which major lanes have matched comparisons, which only have federal
toplines, which remain breadth gaps, and why international efficiency gaps,
improper payments, fraud, and recoverable savings are different quantities.
For the clearest public examples behind that firewall, read [Payment Integrity:
What The Public Evidence Shows](docs/reading/payment-integrity-bounded-factual-examples.md).
It distinguishes the FY2024 $148.971B classified-improper total from $12.570B
unknown payments and labels $161.540B as improper plus unknown.
Use the [headline-number selection guide](docs/reading/headline-number-selection-guide.md)
when choosing among gross/net interest, defense function/subfunction, or
federal/system-wide health measures.

The active implementation frontier is the
[adaptive-rate wave](context/waves/2026-07-18-adaptive-rate-performance-system/WAVE.md#current-frontier).
It shows what is already source-custodied, what remains partial, and which gates
must close before target costs, solver inputs, rates, savings, or
balanced-budget claims can be published.
For the shortest current-status view, use the
[Taxlane showcase readiness summary](docs/reading/taxlane-showcase-readiness-summary.md).
It records two narrow income-security/family contexts ready, four capture gaps
documented, and all solver, rate, savings, public-card, department-cut,
technology-savings, and balanced-budget outputs blocked.

Alternative fiscal institutions live under [experiments](experiments/README.md).
The first experiment simulates an annual computer ballot on which every voter
must allocate exactly 100 budget points, followed by state averaging and an
Electoral College-weighted national aggregation.

## Thesis

The income tax is usually presented as one tax on income, but public trust and
fiscal legibility improve when taxpayers can see which public purposes a tax is
meant to cover. TAXLANE studies three layers:

1. The historical layer: why the modern income tax was passed, which problems it
   was meant to solve, and how its legal and political role changed over time.
2. The current layer: how taxable income, rates, deductions, credits,
   withholding, compliance, appropriations, and general revenue interact today.
3. The reform layer: whether an income-tax system should expose distinct
   funding lanes for programs such as defense, courts, infrastructure, health,
   retirement, debt service, and social insurance.

The repo starts from the practical question: if a tax is supposed to pay for
public goods and lawful public obligations, should income tax be treated as one
general revenue stream or as a transparent portfolio of program taxes?

## Working distinction

A tax is a compulsory public charge used to fund legitimate public obligations.
It is not the same as a user fee, fine, loan, or insurance premium, even when a
tax is politically described that way. Dedicated or earmarked taxes can make
program costs visible, but they can also create false precision because money is
fungible and legislatures still control appropriations.

TAXLANE therefore treats program-linked taxation as a design hypothesis, not
as an assumed answer.

## First questions

- What did supporters and opponents say the income tax was supposed to cover
  when it was created?
- Which parts of today's government are actually funded by income-tax receipts,
  payroll taxes, excise taxes, borrowing, fees, and other revenue?
- Where does the current system hide program costs from taxpayers?
- Which program-linked taxes already exist, such as payroll taxes for social
  insurance?
- What would be clearer, fairer, or riskier about splitting income tax into
  visible funding lanes?

## Design process

The first design pass is source-first:

1. Build a rates timeline from inception.
2. Build receipts records by source and fund group.
3. Build outlay records by function, subfunction, agency, and program.
4. Explain the accounting rules that connect receipts, outlays, trust funds,
   borrowing, and deficits.
5. Only then prototype taxpayer receipts or program-linked tax lanes.

Every public allocation claim must say whether it is legally dedicated,
proportionally allocated, modeled, or illustrative.

The current source-backed finding is that ordinary income tax is mostly a
general fund receipt, so "what your income tax paid for" should be presented as
an allocation model unless a legal dedication is cited.

The first advocacy position is that taxpayers should see public-purpose lanes,
not only a total tax number. Those lanes should map to traditional budget
functions and mandatory/discretionary categories while clearly labeling whether
the lane is legally dedicated, proportional, deficit-inclusive, or proposed.

## Repository shape

- `PRODUCT_PLAN.md` records the product thesis, non-goals, and wave plan.
- `context/waves/` records implementation waves and pulses.
- `data/` holds raw, metadata, extracted, and derived records under custody
  rules.
- `docs/data/` defines data dictionaries before raw source extraction.
- `docs/sources/` records source versions before data extraction.
- `docs/research/` holds cited research notes and decision records.
- `docs/reading/` holds public reading packets derived from the research notes.
- `.roles/` defines the review panel for taxpayer, accounting, source custody,
  public-goods, beneficiary, compliance, fiscal, and reform-skeptic interests.
- `.claude/skills/` contains repo-local wave, pulse, and research workflows.

## Reader packets

For a curated public path, start with
[docs/reading/taxlane-showcase-readiness-summary.md](docs/reading/taxlane-showcase-readiness-summary.md),
then use [docs/showcase.md](docs/showcase.md). For a live walkthrough, use
[docs/demo-script.md](docs/demo-script.md). For the complete packet index, use
[docs/reading/README.md](docs/reading/README.md).

Start with `docs/reading/honest-federal-tax-receipt.md` for the one-page public
case and prototype. It is the shortest route to TAXLANE's central contribution:
every displayed amount must carry its payer base, beneficiary base, financing
relationship, denominator, and allocation status.

Start with `docs/reading/income-tax-foundation.md` for the public foundation
packet. It explains the income-tax history, current mechanics, budget accounting
labels, and program-linked tax lane hypothesis without treating modeled
allocations as legal dedication.

Then use `docs/reading/modeled-income-tax-outlays.md` for the broad modeled
allocation of ordinary individual income-tax receipts by outlay share. Use
`docs/reading/modeled-income-tax-subfunction-outlays.md` only as a drilldown
after the broad financing context is visible.

Use `docs/reading/accountability-public-brief.md` for the current
accountability handoff. It explains which public questions are safe to ask now
and why current draft accountability records are not fraud, waste, abuse, or
performance findings.

For the comparative budget-area findings, read
`docs/reading/budget-area-funding-explainer.md` — the plain-language distillation
of the research program below.

For the spending-side view, read
`docs/reading/where-federal-money-goes.md`. It shows the largest FY2025 outlay
subfunctions, the borrowed-share and income-tax coverage context, and safe
accountability questions without treating spend size as a fraud or waste finding.
Then use `docs/reading/cost-down-backlog.md` for the evidence-first work queue:
which levers could drive costs down over time, what proof is missing, and which
outcome floors must be preserved before any savings claim is allowed.

## Research papers

`research/` holds a panel-reviewed research program with one paper per budget
area, reviewed by a standing panel of AI-simulated public-finance lenses (see
`research/REVIEWERS.md`; not real people, no endorsement implied). All six papers
are accepted and rendered to PDF in `docs/papers/` (build: `pwsh docs/papers/build.ps1`):

| Track | Paper | Finding (direction) |
|---|---|---|
| T0 | Legible Federal Funding (synthesis) | No single "too high/low"; lane-specific + under-collected |
| T1 | The American Health Funding Premium | ~2x peer cost for worse outcomes (fix price, not rate) |
| T2 | The Old-Age Tax and the Cap | Peer-normal spend, below-peer rate on a capped base (fix the cap) |
| T3 | The Defense Tax in Allied Perspective | Above the European norm; a strategic band (2.0–3.5% of GDP) |
| T4 | The Thin Safety Net | Family support below the OECD norm (strengthen + stabilize) |
| T5 | A Low-Tax Country with a Borrowing Habit | ~8.5 pts of GDP below the OECD tax average (raise revenue) |

Inventory and status: `research/RESEARCH.md`. Every figure is sourced in
`docs/sources/source-version-ledger.md`; reform directions are proposals and
value judgments, labeled as such.

## Validation

This is a docs-first repo with a Rust validator for derived artifacts. For the
full source/derived-record guardrail run:

```powershell
cargo run -p taxlane-tools -- income-tax-outlay validate
```

For documentation-only changes, also run:

```powershell
git diff --check
```

The compiled paper PDFs can be rebuilt with:

```powershell
pwsh docs/papers/build.ps1
```

## License

[MIT](LICENSE) — © 2026 Gio Della-Libera.
