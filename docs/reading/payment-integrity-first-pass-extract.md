# Payment Integrity First-Pass Extract

## Purpose

This packet records the first committed extract from the cost-down evidence
queue: the PaymentAccuracy.gov homepage agency trend probe for the eligibility
accuracy work item.

Machine rows:
`data/derived/efficiency_pressure/extracts/payment_integrity_eligibility_first_pass.jsonl`.

## What It Shows

| Row group | Source meaning |
|---|---|
| Highest performing agencies | Homepage agency-level rows with reported 0.000% improper-payment percentage. |
| Lowest performing agencies | Homepage agency-level rows with reported improper-payment percentages from 6.265% to 9.353%. |

## What It Does Not Show

- It is not a program-level improper-payment extract.
- It does not separate overpayments, underpayments, unknown payments, fraud,
  eligibility error, documentation error, or methodology differences.
- It does not estimate savings.
- It does not justify blocking eligible people from lawful benefits.

## Next Extract

Download and lock the program-year PaymentAccuracy data with fiscal year,
agency, program, methodology, root-cause, target, corrective-action,
overpayment, underpayment, and unknown-payment fields.

## Use Rule

Use this as a portal probe only. It is not a savings estimate and not a finding
of fraud, waste, abuse, poor performance, or legal dedication of income-tax
dollars.
