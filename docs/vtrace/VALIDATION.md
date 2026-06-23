# TAXLANE Validation

| ID | User / Actor | Scenario | Success Criteria | Evidence | Status |
|---|---|---|---|---|---|
| VAL-TAX-001 | Citizen reader | Understand a placeholder income-tax receipt without believing ordinary income-tax dollars are legally dedicated to displayed programs. | Display packet, placement spec, checklist, and reviews keep modeled-not-legal and financing context visible. | EVID-TAX-001 | current |
| VAL-TAX-002 | Accountability researcher | Prepare to examine waste, fraud, abuse, or nonperformance without making unsupported allegations. | Accountability model names source evidence, anomaly class, review status, and allegation boundary. | EVID-TAX-002 | current |
| VAL-TAX-004 | Accountability researcher | Inspect a draft accountability evidence record before public use. | Record validates through Rust, cites ledgered source IDs, and states that it is not an allegation, fraud signal, waste signal, or performance score. | EVID-TAX-004 | current |
| VAL-TAX-005 | Accountability researcher | Review a possible fraud, waste, or abuse signal before public use. | Rust validation blocks draft possible-misconduct signals and public accusation wording without official/adjudicated status. | EVID-TAX-005 | current |
| VAL-TAX-006 | Accountability researcher | Decide whether an accountability evidence record can be used publicly. | Rust readiness classification distinguishes evidence-only, needs-role-review, and public-claim-eligible records. | EVID-TAX-006 | current |
| VAL-TAX-007 | Accountability researcher | Scan accountability evidence readiness without reading raw JSONL. | Generated report lists records, readiness states, summaries, and public-use guardrail. | EVID-TAX-007 | current |
| VAL-TAX-008 | Accountability researcher | Record that performance evidence is missing before making a performance claim. | Evidence-only record shows source-backed spending exists but reviewed performance baseline is not attached. | EVID-TAX-008 | current |
| VAL-TAX-009 | Accountability researcher | Decide the next review action for an evidence record. | Readiness report states whether role review, performance evidence, or public wording preparation is the next step. | EVID-TAX-009 | current |
| VAL-TAX-010 | Accountability researcher | Work a queue of accountability evidence records by review task. | Action queue groups records by next task and names the public-use blocker without publishing allegations. | EVID-TAX-010 | current |
| VAL-TAX-011 | Citizen reader | Ask for performance evidence before accepting a claim about public money. | Performance demand packet turns each blocker into a question and repeats the claim boundary. | EVID-TAX-011 | current |
| VAL-TAX-012 | Maintainer | Reuse accountability workflow logic from Rust crate surfaces. | Core crate owns next-action, demand-question, and blocker helpers with tests; CLI reports call those helpers. | EVID-TAX-012 | current |
| VAL-TAX-013 | Product implementer | Feed accountability workflow into a future UI or API. | Generated JSONL exposes readiness, action, question, blocker, and public-claim allowance per evidence record. | EVID-TAX-013 | current |
| VAL-TAX-003 | Maintainer | Extend Rust checks without turning `taxlane-tools` into a monolithic CLI. | Domain crate owns reusable models and CLI owns orchestration. | EVID-TAX-003 | current |
