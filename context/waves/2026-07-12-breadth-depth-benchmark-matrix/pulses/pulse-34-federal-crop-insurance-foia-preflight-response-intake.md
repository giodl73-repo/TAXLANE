# Pulse 34: Federal Crop Insurance FOIA Preflight And Response Intake

## Result

Added an owner preflight checklist and a machine-readable response-intake
contract for the unsent FY2024/Reinsurance Year 2022 cohort-disposition request.
The contract preserves the administrative lifecycle from submission through
acknowledgment, clarification, fee handling, processing, production, adverse
determination, and possible appeal.

Every state has an evidence boundary. An acknowledgment is not a production; a
fee estimate is not proof that responsive records exist; a no-records response
is a search result rather than zero findings or collections; and a denial or
redaction is not proof of nonexistence.

## Decision Gate

Pass for internal preflight and response-intake readiness.

Fail for request submission, fee authorization, expedited-processing claim, or
administrative appeal. Owner identity, contact details, requester category,
fee limit, fee-waiver position, submission channel, and explicit authorization
remain unresolved.

Fail for a methodology-field closure or any scoring, debt, collection,
recovery, prevention, control-cost, or savings claim. FCIC remains four closed
and four open.

## Administrative Timing

The contract records, but does not itself adjudicate, the USDA timing rules:

- ordinary initial determinations are generally due within 20 working days,
  subject to authorized extensions;
- unusual-circumstances notices must give an estimated completion date, and an
  extension exceeding 10 working days triggers an opportunity to narrow or
  arrange an alternative period plus liaison and OGIS notice;
- an administrative appeal must be transmitted or postmarked within 90
  calendar days of the adverse determination; and
- appeal decisions are generally due within 20 working days, subject to an
  authorized extension.

These are intake and monitoring fields, not legal conclusions about exhaustion,
delay, or entitlement.

## Integration Status

Custody, metadata, response contract, blank intake template, preflight
checklist, reader, depth card, ledger, READMEs, Rust validator, and manifest are
integrated. Zero fields close; FCIC remains four closed and four open.

## Next Bounded Action

Stop at the approval gate. The owner must complete the preflight, choose one
submission channel, authorize the fee position, and explicitly authorize the
external submission before any request is sent.
