# TAXLANE Verification

| Requirement ID | Method | Command / Inspection | Expected Result | Status | Evidence |
|---|---|---|---|---|---|
| REQ-TAX-001 | inspection | Inspect `docs/reading/` and `reviews/` | Allocation claims carry modeled/current/reform boundaries. | current | EVID-TAX-001 |
| REQ-TAX-002 | command + inspection | `cargo run -p taxlane-tools -- income-tax-outlay validate`; inspect `docs/design/` | Chart sync passes and display handoffs keep financing context visible. | current | EVID-TAX-001 |
| REQ-TAX-003 | inspection | Inspect mockup checklist and role reviews | Calculator-shaped controls remain blocked. | current | EVID-TAX-001 |
| REQ-TAX-004 | inspection | Inspect `docs/data/accountability-evidence-schema.md`, accountability boundary note, and role review | Fraud/waste indicators are evidence records, not allegations. | current | EVID-TAX-002 |
| REQ-TAX-004 | command + inspection | `cargo run -p taxlane-tools -- income-tax-outlay validate`; inspect `data/derived/accountability_evidence/` | Draft evidence records validate shape, source IDs, review status, and allegation boundary. | current | EVID-TAX-004 |
| REQ-TAX-004 | command + inspection | `cargo test`; inspect `crates/taxlane-core/src/lib.rs` and validator hardening review | Draft possible-misconduct signals and unsupported public accusation wording are blocked. | current | EVID-TAX-005 |
| REQ-TAX-004 | command + inspection | `cargo test`; inspect readiness classification review | Public-claim readiness is explicit and typed. | current | EVID-TAX-006 |
| REQ-TAX-004 | command + inspection | `cargo run -p taxlane-tools -- income-tax-outlay validate`; inspect readiness report review | Readiness report is generated from records and preserves non-allegation guardrails. | current | EVID-TAX-007 |
| REQ-TAX-004 | command + inspection | `cargo run -p taxlane-tools -- income-tax-outlay validate`; inspect evidence-only record review | Evidence-only records can identify missing performance baselines without implying misconduct. | current | EVID-TAX-008 |
| REQ-TAX-004 | command + inspection | `cargo run -p taxlane-tools -- income-tax-outlay validate`; inspect next-action report review | Readiness report includes operational next steps without public claim wording. | current | EVID-TAX-009 |
| REQ-TAX-004 | command + inspection | `cargo run -p taxlane-tools -- income-tax-outlay validate`; inspect action queue review | Action queue groups records into internal reviewer tasks and public-use blockers. | current | EVID-TAX-010 |
| REQ-TAX-004 | command + inspection | `cargo run -p taxlane-tools -- income-tax-outlay validate`; inspect performance-demand packet review | Packet converts blockers into questions without making allegations or scores. | current | EVID-TAX-011 |
| REQ-TAX-004 | command + inspection | `cargo test`; inspect core workflow review | Accountability workflow helpers preserve claim boundaries in reusable Rust code. | current | EVID-TAX-012 |
| REQ-TAX-005 | command | `cargo run -p taxlane-tools -- income-tax-outlay validate` | Validation passes. | current | EVID-TAX-001 |
| REQ-TAX-006 | command + architecture review | `cargo test`; inspect `crates/taxlane-core`, `tools/taxlane`, and `reviews/2026-06-23-rust-core-crate-architecture-review.md` | Domain crate exists and CLI orchestration owns file IO/report generation. | current | EVID-TAX-003 |
| REQ-TAX-006 | command + review | `cargo test`; inspect accountability validator tests | Accountability claim rules live in the core crate with unit tests. | current | EVID-TAX-005 |
| REQ-TAX-006 | command + review | `cargo test`; inspect `PublicClaimReadiness` in `taxlane-core` | Core crate exposes readiness states for future reporting/UI code. | current | EVID-TAX-006 |
| REQ-TAX-006 | command + review | `cargo run -p taxlane-tools -- income-tax-outlay validate`; inspect report generation code | CLI checks generated readiness report staleness. | current | EVID-TAX-007 |
| REQ-TAX-006 | command + review | `cargo run -p taxlane-tools -- income-tax-outlay validate` | Report generation handles multiple readiness rows. | current | EVID-TAX-008 |
| REQ-TAX-006 | command + review | `cargo run -p taxlane-tools -- income-tax-outlay validate`; inspect next-action generation | CLI generates deterministic next actions from validated records. | current | EVID-TAX-009 |
| REQ-TAX-006 | command + review | `cargo run -p taxlane-tools -- income-tax-outlay validate`; inspect action queue generation | CLI checks generated action queue staleness. | current | EVID-TAX-010 |
| REQ-TAX-006 | command + review | `cargo run -p taxlane-tools -- income-tax-outlay validate`; inspect performance-demand packet generation | CLI checks generated demand packet staleness. | current | EVID-TAX-011 |
| REQ-TAX-006 | command + review | `cargo test`; inspect `taxlane-core` workflow helper tests | Core crate exposes reusable accountability workflow wording. | current | EVID-TAX-012 |
