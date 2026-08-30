# TAXLANE Principles

These entries summarize durable TAXLANE decision rules for public-finance
claims, evidence gates, and reader-facing outputs.

## TAX-P-01: Evidence Precedes Rates

**Status:** ACTIVE

**Statement:** Spending claims must pass source, accounting, beneficiary,
implementation, distribution, and overlap gates before they influence any rate
or savings result.

**Rationale:** TAXLANE reverses headline arithmetic: the financing schedule
follows admitted evidence instead of desired rates or guessed savings.

**Decision rule:** A failed or incomplete gate produces a typed zero, null, or
blocked state rather than a placeholder savings percentage.

**Evidence:** `README.md`, `PRODUCT_PLAN.md`, `docs/vtrace/REQUIREMENTS.md`,
and `docs/vtrace/SPECIFICATION_BASELINE.md`.

## TAX-P-02: Accounting Objects Stay Separate

**Status:** ACTIVE

**Statement:** Program spending, payment-integrity signals, net interest,
dedicated solvency, transportation cost notes, and revenue modeling remain on
their own rails.

**Rationale:** Mixing trust-fund, debt, improper-payment, spending, and revenue
objects creates double counting and misleading policy conclusions.

**Decision rule:** A result may not add PAY, NET, OAS, TRN, REV, or program
owner effects together unless the owning evidence path explicitly admits the
same accounting object.

**Evidence:** `README.md`, `docs/reading/wave-lane-depth-scaffold-rollup.md`,
`docs/reading/wave5-fiscal-control-overlay-depth-packets.md`, and
`docs/vtrace/TRACE.md`.

## TAX-P-03: Public Claims Are Permissioned Outputs

**Status:** ACTIVE

**Statement:** Reader-facing claims are published only when the required claim
gate, role review, caveat language, and source route are present.

**Rationale:** TAXLANE is public-facing and legally sensitive; a validated
record is not automatically a safe public claim.

**Decision rule:** Calculator, tax-advice, legal-allocation, savings,
department-cut, fraud, waste, endorsement, or balanced-budget language remains
blocked until the relevant gate closes.

**Evidence:** `README.md`, `.roles/ROLE.md`, `docs/vtrace/REQUIREMENTS.md`,
and `docs/reading/taxlane-showcase-readiness-summary.md`.

## TAX-P-04: Source Custody Beats Convenience

**Status:** ACTIVE

**Statement:** Primary-source capture, metadata, checksums, source rights, and
review status travel with fiscal, legal, rate, receipt, outlay, and
accountability claims.

**Rationale:** Public-finance readers need to audit exactly which source
supports a claim and what uncertainty remains.

**Decision rule:** Uncaptured, stale, missing, ambiguous, or rights-unclear
sources may support research questions or internal queues, but not broadened
public conclusions.

**Evidence:** `docs/sources/source-version-ledger.md`, `docs/data/dictionary.md`,
`docs/vtrace/EVIDENCE.md`, and `.roles/T-3-source-custodian.md`.

## TAX-P-05: Reopening Is Review, Not Admission

**Status:** ACTIVE

**Statement:** New enacted text, official scores, source packets, owner-attributed
effects, debt paths, model inputs, or changed objectives reopen review; they do
not guarantee savings, rates, or public claims.

**Rationale:** Zero and blocked outcomes must remain revisable without becoming
soft admissions or permanent bans.

**Decision rule:** A reopened track starts at evidence review and keeps prior
claim gates closed until new evidence satisfies the same acceptance standard.

**Evidence:** `README.md`, `context/waves/2026-07-18-adaptive-rate-performance-system/WAVE.md`,
`docs/vtrace/VERIFICATION.md`, and `.roles/ROLE.md`.
