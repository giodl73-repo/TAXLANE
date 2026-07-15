# Pulse 33: Federal Crop Insurance Cohort Disposition Request Specification

## Result

Converted Pulse 32's public evidence ceiling into a precise, privacy-aware
request specification for existing records covering the 326-policy FY2024/
Reinsurance Year 2022 payment-integrity sample.

The specification identifies likely CARS, Regional Compliance Office, and OCFO
records; requests native electronic exports, reports, query outputs, data
dictionaries, and reconciliations if they already exist; and defines the
minimum disposition fields needed to distinguish findings, administrative
review, established debt, setoff, cash collection, and write-off.

## Decision Gate

Pass for a submission-ready internal request specification and unsent template.

Fail for submission. No email, portal request, fee commitment, or external
communication is authorized or executed by this pulse. Requester identity,
contact details, fee limit, and any fee-waiver basis remain placeholders.

Fail for an assertion that RMA must create a new record. The request targets
existing records and accepts reasonably segregable, deidentified, statistical,
or aggregate versions when direct case records cannot be released.

Fail for a methodology-field closure or any claim about actual findings, debt,
appeals, collections, prevention, control cost, or savings. FCIC remains four
closed and four open.

## Authority And Channel

- RMA publishes its FOIA portal, service center, public liaison, postal address,
  and `SM.FP.FOIA@usda.gov` contact.
- 7 U.S.C. 1502(c)(1) protects producer-furnished information.
- 7 U.S.C. 1502(c)(2)(A) permits public disclosure after transformation into
  statistical or aggregate form that does not identify the supplier.
- 7 CFR 1.3 and 1.5 support electronic submission, a reasonably described
  request, preferred electronic format, and release of reasonably segregable
  nonexempt portions. Fee handling remains governed by 7 CFR 1.12.

These provisions support a narrowly designed request; they do not guarantee
responsive records or release.

## Integration Status

Custody, metadata, machine specification, unsent request template, reader,
depth card, ledger, READMEs, Rust validator, and manifest are integrated. Zero
fields close; FCIC remains four closed and four open.

## Next Bounded Action

Owner review should supply requester/contact information, choose a fee limit
and fee-waiver position, and explicitly authorize submission. Until then, keep
the template unsent and do not infer that requested records exist.
