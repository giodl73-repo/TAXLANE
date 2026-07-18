# External Accountability Claim Intake

## Internal quarantine boundary

This five-row inventory tests how TAXLANE can receive fast-moving public claims
without adopting them. It is internal and is not routed from the root public
README. Four originating claims remain URL-observed and unverified. A fifth
atom uses a checksum-verified official House copy of written testimony and a
separately owned, checksum-verified DCYF provider table for context. Every atom
uses a neutral paraphrase, and every attributed and substantive public gate
remains false.

The rows cover claims attributed to Nick Shirley: more than $190 million in New
York City adult-day-care and personal-home-care fraud; more than $250 billion
in national savings attributed to his reporting; more than $110 million
allegedly uncovered in Minnesota in one day; and more than $170 million involving combined California daycare
and hospice activity. These sentences describe the quarantine inventory, not
findings that the amounts or underlying conduct are true.

The fifth row records a narrower statement from PDF page 1: in House-hosted
written testimony, Shirley claims that downtown Minneapolis's `Quality Learing
Center` received $1.9 million in Child Care Assistance Program funding for
2025. The spelling `Learing` is preserved from the source. The testimony first
describes the visited providers' amounts as 2025 funding, then gives the named
center's amount. Custody and text review support only that this statement
appears in the testimony; its data cutoff, calculation basis, and accuracy
remain unverified.

A Minnesota House-hosted DCYF provider table identifies `Quality Learning
Center Inc`, license 1087038 at 1411 Nicollet Ave, and reports $1,730,115 in
CY2024 CCAP payments and $2,150,964 in CY2025. The source-stated 2025 $1.9
million does not equal the DCYF CY2025 annual value. Because the testimony
follows a December 16 visit narrative and does not disclose whether its figure
is year-to-date, or its data cutoff or calculation basis, the official table
supplies entity identity and annual-payment context only. It is not entered as
corroboration or counterevidence and does not establish impropriety, fraud,
debt, recovery, or a recipient response.

The same official row records the license status as `Closed` with a license
inactive date of 1/6/2026. That status supplies license context only. The table
does not identify who initiated closure, the authority or reason, or any causal
relationship among closure, CCAP payments, complaints, violations, the
$69,365 assessed and repaid overpayment, or the testimony. The claim's legal
or administrative status therefore remains `none_established`.

## What the records preserve

Each row preserves one amount atom, source URL and ledger ID, date state,
geography, claim type, amount semantic, period and universe boundaries,
unresolved overlap, and response/counterevidence/corroboration state. “More than” amounts
are lower bounds. The testimony's $1.9 million is source-stated exact rather
than a lower bound. All five amounts are `not_summable`.

The House testimony is no longer attached as context to the separate more-than-
$110-million Minnesota row. It is the `claim_origin` of its own payment atom.
Official hosting and custody do not make testimony an official finding or
independent corroboration. News coverage likewise does not establish the
underlying allegation merely because it reports that the allegation was made.

The inventory also registers known official context without treating it as a
match: a separate DOJ New York adult-day-care plea release; a DOJ national
health-care-fraud takedown; an HHS-OIG Minnesota childcare-attendance audit and
DOJ Minnesota case summaries; and a CDPH location review and California DOJ
charging release. These sources concern related sectors or jurisdictions, but
none is mapped to the exact broad atom, amount, entities, transactions, period,
or universe in the attributed claims. Their publication relation is
`supplies_context`; corroboration and counterevidence arrays remain empty,
legal status remains `none_established`, and overlap is not established.

## Evidence progression

For the four URL-observed origins, the next step is custody, not publication.
For the captured testimony atom, the next step is transaction-level payment
lineage that identifies the testimony's 2025 cutoff and basis, plus a response from
the named organization or responsible program office—not a stronger inference
from the testimony, annual context table, or license closure status. Future
review may split separable
program or transaction atoms and seek independently owned supporting records,
counterevidence, and responses from affected parties. Status changes are
append-only, including corrections, retractions, dismissals, or supersession.

The internal Minnesota CCAP CY2025 existing-records request specification
defines a bounded route for that payment-lineage gap. It targets only existing
provider-level records for license 1087038 and excludes direct child, family,
recipient, caregiver, and staff identifiers. Its request template is unsent and
owner authorization is missing. Creating the draft does not record a request or
response on the fifth atom, populate an evidence array, or change status,
non-summability, or any claim gate.

The evidence chain stays explicit:

```text
URL observed
!= source captured
!= attributed claim verified
!= underlying fact corroborated
!= official finding
!= fraud, debt, recovery, or savings
```

Attributed reporting is a separate gate. Even after source custody and role
review allow “the claimant alleged X,” that narrow permission would not make X
true or open fraud, waste, performance, debt, collectibility, recovery,
prevention, or savings gates.

## Use rule

Internal quarantine use only: record that an attributed claim was published,
preserve source and amount semantics, and request corroborating or
counterevidence; do not present the underlying allegation as fact or infer
fraud, waste, debt, collectibility, recovery, prevention, performance, or
savings.
