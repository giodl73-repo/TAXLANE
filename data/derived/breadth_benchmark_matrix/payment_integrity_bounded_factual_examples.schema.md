# Payment Integrity Bounded Factual Examples Schema

## Purpose

This schema defines a compact public explanation layer over already reviewed
payment-integrity evidence. It permits exact, source-labeled facts while
preserving the boundary between statistical estimates, unknown-payment status,
court-confirmed fraud, operational recovery, debt, and savings.

## Required top-level fields

| Field | Rule |
|---|---|
| `record_id` | Stable identifier for the FY2024 bounded-example surface. |
| `record_family` | Must equal `payment_integrity_bounded_factual_examples`. |
| `status` | Must equal `bounded_source_labeled_factual_reporting_only_no_performance_fraud_recovery_or_savings_claim`. |
| `reporting_scope` | States source, period, and evidence-class boundary. |
| `headline_reconciliation_millions` | Source-reported covered-outlay and payment-class values, displayed-value rounding residual and tolerance, plus both reconciliation checks. |
| `evidence_class_legend` | Defines every evidence class used by an example. |
| `ordered_program_cards` | Exactly four cards, ordered Part D, Medicaid, PLTSS, and FCIC, with closed/open counts and one public question each. |
| `bounded_examples` | Exactly seven reviewed examples. |
| `comparison_rules` | Prohibits mixed-period and mixed-evidence-class arithmetic. |
| `use`, `avoid` | Plain-language permitted and prohibited uses. |
| `decision` | Must record zero new components and zero new fields. |
| `claim_gates` | Allows bounded factual reporting only; all established public, performance, fraud, waste, debt, collectibility, recovery, prevention, and savings gates remain false. |
| `role_review_path`, `reader_path` | Repo-relative integration paths. |

## Program-card contract

Each card requires `order`, `program_id`, `program_name`, closed and open field
counts, `sample_period`, the exact annual-row composition in millions, and one
answerable `public_question`. Counts describe internal evidence coverage only.

## Example contract

Each example requires a stable ID, title, evidence class, source IDs, source
artifacts, `allowed_wording`, `required_caveat`, and `prohibited_inferences`.
Allowed wording must preserve publisher, reporting or sample period, units, and
classification. A caveat cannot be removed when the example is reused.

## Hard validation rules

- The displayed payment-type sum is $148,970.632M, $0.001M below the
  source-reported $148,970.633M improper total, and reconciles within the
  explicit $0.001M source-precision tolerance; literal equality must not be
  claimed for the displayed values.
- `improper_plus_unknown` equals `improper_total` plus unknown payments.
- Program-card orders are exactly `1, 2, 3, 4` and counts are `3/5`, `1/7`,
  `2/6`, and `4/4` in that order; every card's `closed_fields` and
  `open_fields` arrays match those counts.
- There are exactly seven examples and all their evidence classes appear in the
  legend.
- Decision counts are `0/0`.
- `bounded_factual_reporting_allowed` is true. `public_claim_allowed`,
  `field_closure_allowed`, `scoring_allowed`, `performance_claim_allowed`,
  `fraud_claim_allowed`, `waste_claim_allowed`, `debt_claim_allowed`,
  `collectibility_claim_allowed`, `recovery_claim_allowed`,
  `prevention_claim_allowed`, and `savings_estimate_allowed` are all false.
- No source capture, methodology count, JSONL, scoring record, or outbound
  action is created by this surface.
