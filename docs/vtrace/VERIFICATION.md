# TAXLANE Verification

| Requirement ID | Method | Command / Inspection | Expected Result | Status | Evidence |
|---|---|---|---|---|---|
| REQ-TAX-001 | inspection | Inspect `docs/reading/` and `reviews/` | Allocation claims carry modeled/current/reform boundaries. | current | EVID-TAX-001 |
| REQ-TAX-002 | command + inspection | `cargo run -p taxlane-tools -- income-tax-outlay validate`; inspect `docs/design/` | Chart sync passes and display handoffs keep financing context visible. | current | EVID-TAX-001 |
| REQ-TAX-003 | inspection | Inspect mockup checklist and role reviews | Calculator-shaped controls remain blocked. | current | EVID-TAX-001 |
| REQ-TAX-004 | inspection | Future accountability schema/review | Fraud/waste indicators are evidence records, not allegations. | planned | EVID-TAX-002 |
| REQ-TAX-005 | command | `cargo run -p taxlane-tools -- income-tax-outlay validate` | Validation passes. | current | EVID-TAX-001 |
| REQ-TAX-006 | command + architecture review | Future `cargo test` after crate split | Domain crate exists and CLI orchestration stays thin. | planned | EVID-TAX-003 |
