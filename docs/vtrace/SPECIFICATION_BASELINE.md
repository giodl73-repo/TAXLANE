# TAXLANE Specification Baseline

| Spec ID | Parent REQ IDs | Type | Current / Target / Deprecated / Unknown | Specification Statement | Verification Method | Validation Method | Owner | Risk | Status |
|---|---|---|---|---|---|---|---|---|---|
| SPEC-TAX-001 | REQ-TAX-001 | content | current | Reading packets and display handoffs use modeled-not-legal wording for ordinary income-tax allocation claims. | inspect docs/reading and reviews | role review | Taxpayer Advocate | high | accepted |
| SPEC-TAX-002 | REQ-TAX-002 | display | current | Placeholder receipt display artifacts require the lane chart, financing context, offset caveats, and dedicated-financing caveats to travel together. | inspect docs/design and chart specs | mockup checklist review | Fiscal Sustainability Reviewer | high | accepted |
| SPEC-TAX-003 | REQ-TAX-003 | product boundary | current | Placeholder receipt artifacts remain static and block taxpayer input, filing, refund, credit, withholding, and liability fields. | role review inspection | compliance review | Compliance Burden Reviewer | high | accepted |
| SPEC-TAX-004 | REQ-TAX-004 | accountability | current | Accountability records distinguish performance indicators, anomaly indicators, source evidence, review status, and allegation boundaries. | schema/research inspection | source-custody review | Source Custodian | high | accepted |
| SPEC-TAX-005 | REQ-TAX-005 | tooling | current | `taxlane-tools income-tax-outlay validate` validates model outputs, manifest coverage, chart specs, and placeholder receipt chart sync. | command | validation run | Maintainer | medium | accepted |
| SPEC-TAX-006 | REQ-TAX-006 | architecture | current | Rust workspace includes `taxlane-core` for reusable domain contracts and keeps file IO/report orchestration in `taxlane-tools`. | architecture review | cargo test | Maintainer | medium | accepted |
