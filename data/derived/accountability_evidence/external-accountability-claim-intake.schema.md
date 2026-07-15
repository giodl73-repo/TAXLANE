# External Accountability Claim Intake Schema

## Purpose and status

This contract quarantines externally published accountability claims before
they enter TAXLANE's reviewed evidence model. It is an internal inventory, not
an allegation feed, public reader, finding, score, or savings estimate.

One JSONL row records one independently assessable amount atom. Publication
custody can establish that a claim was published; it does not establish that
the underlying claim is true.

## Required fields

Each row requires stable record and group IDs, `record_family`, schema version,
atom order, claimant, one or more publications, a neutral claim atom, one amount
assertion, evidence and response state, claim and legal status, review state,
comparison basis, due-process caveat, the exact use rule, and all claim gates.
`record_family` is `external_accountability_claim_intake`.

`exact_text_verified` remains false for the four URL-observed originating
claims. The separate House testimony atom may set it true because the official
copy is captured and the neutral paraphrase is verified against PDF page 1.
`url_observed_not_captured` publications have null custody path and SHA-256;
`official_copy_captured` requires both fields and matching metadata. Every
referenced URL requires its own source-ledger ID.

Publication dates may use source-available `YYYY`, `YYYY-MM`, or `YYYY-MM-DD`
precision but must be real calendar values. Their possible date interval cannot
fall after the full observation date; observation cannot fall after
`status_as_of`. The validator also binds every publication's source ID, URL,
publisher, date, kind, and evidence relation to the corresponding ledger row
and expected intake contract.

## Claim atomization

One row has one subject, predicate, object, geography/period, and independently
assessable amount. Split observations from inferences; split programs, named
entities, periods, and amounts when the source supplies separable bases. Keep
related atoms under `claim_group_id`. Do not apportion a combined amount across
programs without source-stated components.

Neutral paraphrases replace exact quotations. A source hosted by an official
body may `supplies_context`; hosting does not convert testimony or an external
allegation into an official finding.

An official audit, case, plea, charge, review, or takedown concerning the same
sector or jurisdiction also remains `supplies_context` until an evidence map
connects it to the exact atom's program, entities or transactions, period,
universe, and amount. Sector similarity alone does not populate corroboration
or counterevidence, establish overlap, or change the row's legal status.

## Controlled vocabularies

- `claimant_type`: `journalist_or_commentator`, `witness`, `whistleblower`,
  `beneficiary`, `vendor_or_recipient`, `agency_official`,
  `inspector_general`, `law_enforcement`, `court`, `elected_official`, `other`.
- `publication_kind`: `original_video`, `social_post`, `article`, `interview`,
  `written_testimony`, `hearing_video`, `agency_release`, `audit_report`,
  `court_record`, `dataset`, `response_letter`, `correction`, `other`.
- `claim_type`: `direct_observation`, `identity_or_affiliation`,
  `site_or_service_operation`, `eligibility_or_enrollment`,
  `payment_or_billing`, `award_or_contract`, `duplicate_or_overlap`,
  `data_quality`, `control_failure`, `aggregate_improper_payment_allegation`,
  `aggregate_fraud_allegation`, `performance_allegation`, `debt_allegation`,
  `recovery_assertion`, `prevention_assertion`, `savings_assertion`,
  `official_response`, `correction_or_retraction`, `other`.
- `evidence_relation`: `claim_origin`, `supports_claim_atom`,
  `corroborates_part`, `contradicts_part`, `supplies_context`,
  `official_response`, `correction`, `supersession`.
- `custody_status`: `url_observed_not_captured`, `captured_hash_verified`,
  `official_copy_captured`, `unavailable`, `superseded`.
- `claim_status`: `intake_unverified`, `source_custodied`,
  `evidence_mapping_in_progress`, `attributed_claim_supported`,
  `partially_corroborated`, `independently_corroborated`, `contested`,
  `unable_to_verify`, `corrected`, `retracted`, `superseded`.
- `legal_or_administrative_status`: `none_established`,
  `agency_review_reported`, `audit_opened`, `referred_for_review`,
  `investigation_reported`, `civil_complaint_filed`,
  `criminal_charge_filed`, `official_finding`, `settlement_no_admission`,
  `settlement_with_admission`, `plea_entered`, `adjudicated`, `dismissed`,
  `overturned`, `closed_without_finding`, `unknown`.
- `review_status`: `draft`, `source-reviewed`, `accountability-reviewed`,
  `role-reviewed`, `superseded`, `retired`.

## Amount semantics

Permitted semantics include `source_stated_total`, `program_outlays`,
`award_or_contract_ceiling`, `paid_amount`, `billed_amount`, `questioned_cost`,
`statistical_improper_payment_estimate`, `unknown_payment_status`,
`alleged_fraud_exposure`, `charged_loss`, `court_confirmed_fraud`,
`settlement_amount`, `identified_overpayment`, `established_debt`,
`collectible_amount`, `recovered_cash`, `restitution_ordered`,
`restitution_paid`, `prevented_loss_estimate`, `source_stated_savings_total`,
`gross_savings_estimate`, `control_cost`, `offset_or_leakage`, and
`net_savings_estimate`.

Amounts require value or bounds, currency, unit, semantic, assertion status,
period, universe, derivation, aggregation method, overlap group, overlap state,
summability, and lineage IDs. “More than” is stored as a lower bound, not an
exact amount. Initial rows are `not_summable`. Alleged exposure is not an
improper-payment estimate, charged loss, debt, collection, or savings. Mixed
periods, universes, semantics, or unresolved overlap cannot be added, divided,
ranked, allocated, or netted.

## Evidence, response, and lifecycle

Corroborating and counterevidence IDs are separate. The response object records
whether a response was requested, from whom, when, and which ledgered sources
contain it. A denial is not proof that a claim is false, and absence of a reply
is not corroboration.

`intake_unverified` may move to `source_custodied` only after ledgered,
hash-verified capture. `attributed_claim_supported` means the source supports
only that the claimant made the paraphrased claim; it does not verify an
underlying payment or its accuracy. Independent corroboration
must come from separately owned evidence mapped to the atom; reposting and
syndication do not qualify. Complaints, referrals, investigations, and charges
remain distinct from official findings and adjudication.

Corrections, retractions, dismissals, and supersessions are append-only. Keep
the historical row, add the status and source, create a successor when needed,
and link `supersedes_record_id`. Never silently rewrite claim history.

## Public gates

`attributed_claim_reporting_allowed` is independent of every substantive gate.
It requires captured custody, accurate attribution, status date, due-process
and response state, and approved exact wording. Even when true, it authorizes
only reporting that the claimant made a claim.

`underlying_factual_claim_allowed`, `misconduct_signal_claim_allowed`,
`official_finding_claim_allowed`, `performance_claim_allowed`,
`fraud_claim_allowed`, `waste_claim_allowed`, `debt_claim_allowed`,
`collectibility_claim_allowed`, `recovery_claim_allowed`,
`prevention_claim_allowed`, and `savings_estimate_allowed` require their own
reviewed evidence. No gate opens another automatically. All gates remain false
in the five-row inventory, including the custody-backed testimony atom.

The three intake artifacts are internal routes. Validation requires them in the
accountability-evidence index and rejects their appearance in the root README or
public reading index.

## Exact use rule

> Internal quarantine use only: record that an attributed claim was published, preserve source and amount semantics, and request corroborating or counterevidence; do not present the underlying allegation as fact or infer fraud, waste, debt, collectibility, recovery, prevention, performance, or savings.
