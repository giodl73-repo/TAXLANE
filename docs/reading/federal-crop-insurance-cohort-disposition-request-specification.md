# Federal Crop Insurance Cohort Disposition Request Specification

Machine record:
`data/derived/breadth_benchmark_matrix/federal_crop_insurance_cohort_disposition_request_specification.fy2024.v1.draft.json`.

## Result

Pulse 32 showed that public Manager's Reports stop preserving the 326-policy
FY2024/Reinsurance Year 2022 cohort after publishing the 2.43-percent rate.
This specification turns the remaining evidence gap into a bounded request for
existing RMA records.

The request targets existing CARS, Regional Compliance Office, compliance-
management, and OCFO exports, reports, ledgers, query outputs, dictionaries,
and reconciliations. It asks for a stable deidentified case key and the state
transitions needed to distinguish Initial and Final Findings, administrative
review, corrected amounts, established debt, repayment discretion, setoff,
cash receipt, and noncollection dispositions.

## Privacy and process boundary

Producer-furnished information is protected by 7 U.S.C. 1502(c)(1), while
7 U.S.C. 1502(c)(2)(A) permits statistical or aggregate disclosure that does not
identify the supplier. The specification therefore excludes names, addresses,
policy numbers, farm identifiers, and identifying free text. It accepts an
existing deidentified row-level record or an existing statistical or aggregate
record and asks for all reasonably segregable nonexempt portions.

The request does not ask RMA to create a new record. Preferred delivery is CSV,
XLSX, JSON, or another readily reproducible native electronic format.

## Decision

The request template is draft and unsent. No email, portal submission, or fee
commitment occurred. Owner authorization, requester details, a fee limit, and
the fee-waiver position are required before submission.

This closes zero methodology fields. FCIC remains four closed and four open.
No score or claim about findings, debt, collection, fraud, prevention, control
cost, recovery, or savings is allowed.
