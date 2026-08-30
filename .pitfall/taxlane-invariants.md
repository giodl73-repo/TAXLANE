# TAXLANE Invariants

These entries summarize properties that must remain true for TAXLANE evidence,
public claims, derived artifacts, and model boundaries.

## TAX-I-01: Taxpayer Allocation Claims Name Their Method

**Status:** VERIFIED

**Claim:** Every taxpayer-facing allocation claim identifies whether it is
current-law legal dedication, modeled proportional allocation,
deficit-inclusive allocation, or reform proposal.

**Why it matters:** Without the method label, readers can mistake an
illustrative or modeled view for legal tracing of income-tax dollars.

**Enforcement:** VTRACE requirement `REQ-TAX-001`, specification `SPEC-TAX-001`,
role acceptance, and artifact validation preserve the allocation-method gate.

**Evidence:** `docs/vtrace/REQUIREMENTS.md`, `docs/vtrace/SPECIFICATION_BASELINE.md`,
`docs/vtrace/VERIFICATION.md`, `.roles/ROLE.md`, and
`cargo run -p taxlane-tools -- income-tax-outlay validate`.

## TAX-I-02: Calculator-Shaped Taxpayer Flows Stay Blocked

**Status:** VERIFIED

**Claim:** TAXLANE does not accept taxpayer input, filing-status fields,
withholding, refund, credit, or liability controls until a separate calculator
and tax-advice boundary review exists.

**Why it matters:** Calculator affordances change the project from civic
explanation into personal tax advice risk.

**Enforcement:** VTRACE requirement `REQ-TAX-003`, specification `SPEC-TAX-003`,
and Compliance Burden review keep static displays separate from calculators.

**Evidence:** `README.md`, `PRODUCT_PLAN.md`, `docs/vtrace/REQUIREMENTS.md`,
`docs/vtrace/SPECIFICATION_BASELINE.md`, and `.roles/T-6-compliance-burden.md`.

## TAX-I-03: Accountability Records Are Not Allegations

**Status:** VERIFIED

**Claim:** Fraud, waste, abuse, anomaly, and performance signals remain evidence
records with source, review, response, due-process, and claim-gate status; they
do not become public findings by default.

**Why it matters:** Accountability tooling must help readers ask for evidence
without publishing unsupported accusations.

**Enforcement:** VTRACE `REQ-TAX-004`, core validation, generated claim-gate
reports, and role reviews keep current public-claim gates closed unless
explicitly accepted.

**Evidence:** `docs/vtrace/REQUIREMENTS.md`, `docs/vtrace/EVIDENCE.md`,
`data/derived/accountability_evidence/claim-guard-report.md`,
`crates/taxlane-core/src/lib.rs`, and
`cargo run -p taxlane-tools -- income-tax-outlay validate`.

## TAX-I-04: Derived Artifacts Are Reproducibly Checked

**Status:** VERIFIED

**Claim:** Canonical derived artifacts, manifests, charts, reading handoffs,
and explanation outputs are validated before closure or public-readiness claims.

**Why it matters:** TAXLANE has many generated surfaces; stale or inconsistent
artifacts can make public claims outrun the source spine.

**Enforcement:** The maintained validation block runs workspace tests,
income-tax-outlay validation, manifest checks, paper builds, and whitespace
checks.

**Evidence:** `README.md`, `docs/vtrace/VERIFICATION.md`, `docs/vtrace/EVIDENCE.md`,
and `cargo test --workspace`.

## TAX-I-05: Showable Does Not Mean Unblocked

**Status:** VERIFIED

**Claim:** TAXLANE may be repository-ready and externally showable while
calculator, savings, legal-allocation, rate-solver, publication, endorsement,
submission, and deployment claims remain blocked.

**Why it matters:** Public evaluation can proceed only if readiness language
does not imply official status, legal advice, or policy authority.

**Enforcement:** README caveats, VTRACE `REQ-TAX-007`, showcase readiness
summary, and role review keep showable surfaces separate from blocked outputs.

**Evidence:** `README.md`, `docs/reading/taxlane-showcase-readiness-summary.md`,
`docs/vtrace/REQUIREMENTS.md`, `docs/vtrace/VERIFICATION.md`, and
`docs/demo-script.md`.
