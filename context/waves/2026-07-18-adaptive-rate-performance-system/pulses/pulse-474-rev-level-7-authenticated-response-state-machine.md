# Pulse 474 — REV Level 7 authenticated-response state machine

## Outcome

The external-response intake is now transition-ready rather than empty-state
only. It defines and validates three states: no submission or response,
official response received pending review, and authenticated response ready for
rate recertification.

## Admission boundary

- Receipt lineage must match the sealed bundle and carry hashed evidence.
- Every supplied response asset must identify its official office and document
  and must match its recorded SHA-256 digest.
- Rate recertification requires FY2026-FY2035 conventional revenue values,
  Legislative Counsel text, annual score data, resolved scope checks, and
  independent role review.
- The state machine does not authorize a submission, invent a receipt, treat a
  synthetic fixture as official, or certify a rate or balanced budget.

## Next trigger

Populate the received state only after an authorized transmission and genuine
receipt. Populate the authenticated state only after official-source custody
and role review. Then run the targeted PAY-NET-REV and affected-track
reconciliation before any rate recertification.
