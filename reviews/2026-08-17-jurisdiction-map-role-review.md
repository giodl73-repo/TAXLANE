# Jurisdiction map role review

## Scope

This review covers `docs/jurisdiction-map.md` and its mapping of all 17 active
TAXLANE lane-crosswalk rows, domain evidence owners, ownership gaps, and
cross-cutting experiment boundaries.

## Decision

Accept the map as TAXLANE's portfolio jurisdiction contract.

The decision approves coordination and held-evidence handoffs only. It does not
approve a tax rate, allocation, savings claim, taxpayer receipt, domain finding,
program decision, election rule, or public release.

## Findings

| Role | Result |
|---|---|
| Taxpayer Advocate | Pass: the map makes clear that a budget label is not proof that a service was delivered. |
| Budget Accountant | Pass: TAXLANE retains receipt, outlay, deficit, offset, net-interest, allocation-method, and claim-status ownership. |
| Source Custodian | Pass with gate: domain packs must retain source, version, observation, time, population, and geography fields. |
| Public Goods Steward | Pass: each named repository is limited to its actual domain, and mixed lanes preserve partial-owner gaps. |
| Program Beneficiary Reviewer | Pass: service, rights, adequacy, continuity, and burden evidence cannot be overwritten by fiscal aggregation. |
| Fiscal Sustainability Reviewer | Pass: net interest, borrowed share, offsets, rates, and savings remain distinct fiscal claims. |
| Reform Skeptic | Pass: unassigned domains are explicit release constraints rather than implied TAXLANE authority. |

## Required guardrails

1. A held domain pack remains held after TAXLANE ingestion.
2. TAXLANE cannot infer outcomes, efficiency, or savings from spending levels.
3. Partial owners do not inherit a mixed OMB lane's remaining semantics.
4. RATIFY, RCOUNT, and RPLAN authority begins only through an explicit adopted
   contract; the current ballot experiment does not imply adoption.
5. Ownership gaps must remain visible until a repository-level scope and evidence
   contract are reviewed.

## Coverage result

- 17 of 17 active lane IDs are listed.
- 14 rows name at least one bounded domain owner or TAXLANE fiscal owner.
- 6 rows preserve a full or partial domain-owner gap.
- 3 cross-cutting experiment interfaces state their external authority boundary.
