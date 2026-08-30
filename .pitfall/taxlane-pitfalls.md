# TAXLANE Pitfalls

These entries capture recurring civic-finance failure classes and cite
TAXLANE's existing countermeasures.

## TAX-PF-01: Headline Arithmetic Becomes Policy

**Status:** MITIGATED

**Pattern:** A desired tax rate, spending cut, or savings number is chosen
first, then evidence is arranged to support it.

**Domain:** Rate modeling, spending reductions, public explanations, research
papers, and presentation surfaces.

**Detection difficulty:** Headline numbers are easy to explain and can look
more complete than gate failures, nulls, or typed zero results.

**Structural solution:** Require evidence gates before admission, preserve
typed zero and blocked states, and keep the rate schedule downstream of the
remaining scoped financing target.

**Evidence:** `README.md`, `research/publications/why-zero-is-a-result/paper.md`,
`docs/vtrace/REQUIREMENTS.md`, and `docs/vtrace/VERIFICATION.md`.

## TAX-PF-02: Fungibility Is Hidden

**Status:** MITIGATED

**Pattern:** A taxpayer-facing display implies that ordinary income-tax dollars
are legally dedicated to particular programs when the record supports only
modeled, proportional, deficit-inclusive, or reform allocation.

**Domain:** Receipts, taxpayer explainers, lane displays, charts, classroom
materials, and public handoffs.

**Detection difficulty:** Allocation visuals can look intuitive even when the
legal and budget-accounting basis is only illustrative.

**Structural solution:** Attach allocation-method labels, financing context,
deficit caveats, dedicated-financing caveats, and Budget Accountant plus Reform
Skeptic acceptance gates.

**Evidence:** `PRODUCT_PLAN.md`, `.roles/ROLE.md`,
`docs/vtrace/SPECIFICATION_BASELINE.md`, and
`docs/reading/where-federal-money-goes.md`.

## TAX-PF-03: Payment Integrity Becomes Fraud Or Savings

**Status:** MITIGATED

**Pattern:** Improper-payment estimates, anomalies, audit probes, or benchmark
differences are treated as fraud, waste, recoverable debt, or budget savings.

**Domain:** Accountability records, payment-integrity overlays, demand packets,
response logs, dashboards, and public questions.

**Detection difficulty:** Evidence gaps and statistical estimates are
politically salient, so readers may infer findings that the source does not
establish.

**Structural solution:** Keep evidence records non-allegatory, preserve response
and due-process status, require same-cohort collection or causal prevention
before savings, and keep public-claim gates false until review accepts wording.

**Evidence:** `docs/vtrace/EVIDENCE.md`,
`data/derived/accountability_evidence/README.md`,
`data/derived/accountability_evidence/claim-guard-report.md`, and
`crates/taxlane-core/src/lib.rs`.

## TAX-PF-04: Solver Output Escapes Its Caveats

**Status:** MITIGATED

**Pattern:** Internal analytical marginal rates are described as enacted rates,
effective rates, official scores, advice, or balanced-budget proof.

**Domain:** README summaries, final briefing bundles, papers, presentations,
local website, and rate/uncertainty explainers.

**Detection difficulty:** Once a rate schedule is concise, audiences may strip
away model basis, behavior cases, stress cases, and non-advice caveats.

**Structural solution:** Keep central, behavior-contingency, and severe-stress
rails separate, repeat marginal-model labels, and block official/legal/advice
claims in every public handoff.

**Evidence:** `README.md`, `docs/explanation/foundation/canonical-result-statement.md`,
`docs/explanation/foundation/number-ledger.md`, and
`docs/explanation/final/cross-format-consistency-report.md`.

## TAX-PF-05: Reopened Tracks Bypass Admission

**Status:** MITIGATED

**Pattern:** A new source, owner packet, external candidate, or policy scenario
is treated as admitted savings or a reopened solver input before review closes
the same gates.

**Domain:** Lane-owner intakes, source-capture queues, adaptive-rate waves,
scenario packs, and downstream portfolio handoffs.

**Detection difficulty:** Fresh evidence can look like progress even when it
only changes the review queue or names the next source to request.

**Structural solution:** Route new packets through source custody, owner
attribution, beneficiary/continuity, implementation, overlap, model-input, and
role-review gates before changing admitted savings or rates.

**Evidence:** `README.md`,
`docs/reading/anchor-bastion-sem012-intake-disposition.md`,
`context/waves/2026-07-18-adaptive-rate-performance-system/WAVE.md`, and
`docs/vtrace/VERIFICATION.md`.
