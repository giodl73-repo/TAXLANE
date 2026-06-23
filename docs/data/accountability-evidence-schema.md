# Accountability Evidence Schema

## Purpose

This schema defines how TAXLANE should record performance, waste, fraud, and
abuse evidence before public accountability claims are made.

It is not an allegation system. It is an evidence model for surfacing reviewed
signals, open audit questions, and official findings with clear source custody
and due-process boundaries.

## Record Family

`accountability_evidence`

## Required Fields

| Field | Required | Meaning |
|---|---|---|
| `record_id` | Yes | Stable repo-local ID. |
| `record_family` | Yes | Must be `accountability_evidence`. |
| `lane_id` | Yes when applicable | TAXLANE public-purpose lane under review. |
| `program_or_account_id` | Yes when available | Program, budget account, award, contract, grant, recipient, vendor, or agency identifier. |
| `source_ids` | Yes | Source IDs from `docs/sources/source-version-ledger.md`. |
| `observed_date` | Yes | Date TAXLANE observed the source. |
| `coverage_period` | Yes | Fiscal year, award period, audit period, or other source-defined period. |
| `evidence_kind` | Yes | Indicator class from the controlled vocabulary below. |
| `indicator_value` | Optional | Numeric value used by the indicator. |
| `indicator_units` | Required when value exists | Dollars, percent, count, rate, score, or other units. |
| `comparison_basis` | Yes | Baseline, target, prior year, peer program, statutory rule, award terms, audit rule, or source-defined control. |
| `anomaly_class` | Yes | Review classification from the controlled vocabulary below. |
| `allegation_status` | Yes | Boundary between evidence signal and accusation. |
| `review_status` | Yes | Draft and review state. |
| `due_process_caveat` | Yes | Plain-language limit on what the record claims. |
| `public_summary` | Yes | Reader-facing evidence summary. |

## Controlled Vocabularies

| Field | Values |
|---|---|
| `evidence_kind` | `performance`, `spending_variance`, `duplicate_award`, `vendor_concentration`, `eligibility_mismatch`, `audit_finding`, `ig_finding`, `gao_finding`, `data_quality`, `other` |
| `anomaly_class` | `none`, `variance`, `outlier`, `missing_evidence`, `source_conflict`, `control_failure`, `possible_waste`, `possible_fraud`, `possible_abuse` |
| `allegation_status` | `not_an_allegation`, `referred_for_review`, `official_finding`, `adjudicated` |
| `review_status` | `draft`, `source-reviewed`, `accountability-reviewed`, `role-reviewed`, `superseded`, `retired` |

## Public Wording Rules

- Say `evidence signal`, `indicator`, `official finding`, or `open audit
  question` as appropriate.
- Do not say fraud, waste, or abuse occurred unless an official finding or
  adjudication source supports that wording.
- `possible_fraud`, `possible_waste`, and `possible_abuse` are review labels,
  not public accusations.
- Preserve the difference between nonperformance, data-quality gaps,
  noncompliance, waste, abuse, and fraud.
- Named vendor, recipient, award, or person-level concerns require role review
  before public use.

## Example Boundary

| Unsupported wording | Acceptable wording |
|---|---|
| "This contractor committed fraud." | "This record has a source-reviewed indicator that requires audit review; it is not an allegation or finding." |
| "Your taxes were stolen." | "This lane has an official finding or unresolved evidence signal; see source IDs and review status." |
| "The program wasted money." | "The record shows a variance against the named comparison basis and remains subject to review." |

## Validation Gate

Before publishing an accountability record:

1. Source IDs must exist in the source-version ledger.
2. The comparison basis must be explicit.
3. The allegation status must be visible.
4. The due-process caveat must be present.
5. Source Custodian and Reform Skeptic review must approve the wording.

Rust validation additionally blocks:

- `possible_fraud`, `possible_waste`, and `possible_abuse` records that remain
  in `draft` review status.
- Public accusation wording such as committed fraud, wasted money, abused
  funds, stole, or stolen unless `allegation_status` is `official_finding` or
  `adjudicated`.

Rust validation also classifies public-claim readiness:

| Readiness | Meaning |
|---|---|
| `EvidenceOnly` | Record may support internal evidence review but is not ready for public claims. |
| `NeedsRoleReview` | Record has source or accountability review and needs role review before public use. |
| `PublicClaimEligible` | Record has role review plus `official_finding` or `adjudicated` allegation status. |
