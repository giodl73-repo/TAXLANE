# FY2024 PaymentAccuracy Annual Extraction Role Review

> This is an AI-simulated review using TAXLANE's public-interest role lenses.
> It is not external endorsement, legal advice, accounting advice, or a
> representation of any government office.

## Scope

This review covers the FY2024 PaymentAccuracy annual-workbook extraction: the
governmentwide total, 68 program-result rows, 54 court-confirmed-fraud rows,
and 59 agency-recovery rows. The separate corrected VA PLTSS scorecard probe
was already reviewed on July 13, 2026.

The source is the official OMB workbook registered as
`SRC-OMB-PAYMENTACCURACY-FY2024-DATA`. The captured file has 700,992 bytes and
SHA-256 checksum
`595369DA4C32965C457543E2695B5738BC131049318537948CD396323391E28C`.

| Artifact | Review role |
|---|---|
| `data/raw/omb/SRC-OMB-PAYMENTACCURACY/2026-07-12/FY2024_Dataset.xlsx` | Captured official workbook. |
| `data/metadata/SRC-OMB-PAYMENTACCURACY-FY2024-DATA.2026-07-12.metadata.md` | Source identity, URL, observation date, raw path, checksum, and coverage. |
| `docs/data/payment-accuracy-annual-dataset-schema.md` | Evidence classes, custody fields, and non-conversion rules. |
| `data/extracted/payment_accuracy/fy2024_extraction_summary.v1.draft.json` | Extraction counts and top-level reconciliation flags. |
| `data/extracted/payment_accuracy/fy2024_governmentwide_total.v1.draft.json` | FY2024 governmentwide total from `Improper Payment Totals` row 22. |
| `data/extracted/payment_accuracy/fy2024_program_results.v1.draft.jsonl` | 68 FY2024 `All Program Results` rows with source-row and measurement-period lineage. |
| `data/extracted/payment_accuracy/fy2024_confirmed_fraud.v1.draft.jsonl` | 54 FY2024 `Confirmed Fraud` rows with the court-confirmed definition. |
| `data/extracted/payment_accuracy/fy2024_agency_recovery.v1.draft.jsonl` | 59 FY2024 agency-level `Recovery Details` rows with nulls preserved. |
| `reviews/2026-07-13-payment-integrity-va-pltss-source-role-review.md` | Prior corrected-scorecard source and role review; linked, not reopened. |

## Reconciliation finding

The extracted governmentwide record reconciles $135,184.184 million in
overpayments, $7,863.903 million in underpayments, and $5,922.545 million in
technically improper payments to $148,970.633 million in improper payments.
Adding $12,569.500 million in unknown payments reconciles to the
$161,540.133 million combined headline. The workbook covers $4,071,860.585
million in reported outlays, not the full federal payment universe.

Program measurement periods vary. The program rows remain source-labeled and
must not be ranked or combined as though they share one tested-payment cohort.

## Validation evidence

The review checked that:

- the raw workbook byte count and SHA-256 match its metadata;
- the summary and governmentwide JSON parse and all three JSONL files parse;
- the extraction contains exactly one governmentwide row, 68 program rows, 54
  court-confirmed-fraud rows, and 59 agency-recovery rows;
- every extracted row retains FY2024, the expected record family, source ID,
  source sheet, source-row anchor, and `draft_extracted` status;
- each program row reconciles overpayment plus underpayment plus technically
  improper payment to improper payment, and improper plus unknown payment to
  the combined amount, within source precision;
- the governmentwide row passes the same two reconciliations and preserves
  $4,071,860.585 million in covered outlays;
- program measurement periods and confidence labels remain source-specific;
- every confirmed-fraud row retains the court-confirmed-cases-only definition;
  and
- every recovery row retains the agency-level scope note, with source nulls
  remaining null rather than becoming zero.

Result:

```text
raw custody ok
JSON and JSONL parse ok
rows: governmentwide 1; program 68; confirmed fraud 54; agency recovery 59
program and governmentwide payment-class reconciliations ok
measurement-period, fraud-definition, recovery-scope, and null boundaries ok
```

## Role decisions

| Role | Decision |
|---|---|
| Source Custodian | Pass for the captured workbook and draft extraction. The source ID, file custody, row counts, source sheets, source rows, units, periods, byte count, and checksum are explicit. |
| Budget Accountant | Pass for exact source-labeled composition and reconciliation. Improper payments, unknown payments, overpayments, underpayments, technical errors, confirmed fraud, agency recovery, identified debt, and net savings remain distinct quantities. |
| Program Beneficiary Reviewer | Pass for descriptive evidence only. No control or savings proposal is approved without access, continuity, error-correction, appeal, timeliness, and burden evidence at the affected program grain. |
| Reform Skeptic | Pass for the evidence firewall. The extraction does not subtract, divide, or net fraud or recovery rows against statistical estimates and does not convert an accounting category into waste or savings. |

## Decision

Pass the annual extraction for internal analysis and bounded factual reporting
that preserves the publisher, fiscal year, units, coverage qualification,
program-specific measurement periods, and evidence-class labels.

Fail unrestricted public, fraud, waste, debt, collectibility, recovery,
prevention, or savings claims. The confirmed-fraud table is limited to
court-confirmed cases and is not established as a disjoint subset of the
improper-payment estimate. The agency-recovery table uses separate operational
bases and periods and is not a direct subset of estimated program
overpayments.

This review closes the pending annual-extraction review action only. It closes
zero methodology components and zero methodology fields and changes no program
counts or claim gates.
