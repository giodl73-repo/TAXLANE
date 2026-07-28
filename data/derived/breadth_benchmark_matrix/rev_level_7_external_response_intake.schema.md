# REV Level 7 external response intake schema

The intake is a three-state machine:

1. `intake_ready_no_submission_or_response` requires every receipt, response,
   asset, annual-score, review, and official-output field to remain empty or
   false.
2. `official_response_received_pending_review` requires complete submission
   receipt lineage, verified responding-office identity, and SHA-256 custody
   for every supplied response asset. It cannot authorize rate
   recertification.
3. `authenticated_response_ready_for_rate_recertification` additionally
   requires FY2026-FY2035 annual rows, numeric conventional-revenue estimates,
   exact Legislative Counsel text, conventional-score and annual-data assets,
   resolved scope review, and independent role review.

Every non-null response asset is an object containing `path`, `sha256`,
`responding_office`, and `document_identifier`. Receipt evidence and response
verification evidence also require matching repository paths and SHA-256
digests. Self-declared fields do not replace source authentication or role
review.

No official score, certified rate, or balance determination may be populated
until the response is authenticated, policy and baseline are matched, every
required scope is dispositioned, and independent role review is complete.
