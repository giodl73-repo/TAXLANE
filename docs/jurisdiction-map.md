# TAXLANE jurisdiction map

TAXLANE is the portfolio's public-finance coordinator. It owns federal receipt,
outlay, deficit, financing, crosswalk, and claim-status semantics inside
TAXLANE artifacts. It does not acquire justice, defense, health, benefit,
infrastructure, election, or other domain authority merely because those
subjects appear in a budget lane.

This map covers the 17 active rows in
`data/derived/lane_crosswalk/lane_crosswalk.omb-fy2027-v1.2026-06-22.draft.jsonl`.
Every row has a fiscal owner and either a named domain owner or an explicit
portfolio ownership gap.

## Coordination contract

TAXLANE may:

- map source-custodied OMB functions and subfunctions to public labels;
- reconcile receipts, outlays, offsets, net interest, and borrowed share;
- label an amount as observed, legally dedicated, modeled, illustrative, or
  proposed;
- receive a versioned domain evidence pack whose authority flags remain intact;
  and
- hold a rate, allocation, savings, or release claim when required evidence is
  absent.

TAXLANE may not:

- turn a domain finding into a tax rate, allocation, savings claim, procurement
  choice, benefit change, or public release without TAXLANE admission;
- turn a TAXLANE budget amount into evidence of service quality or outcomes;
- reinterpret a held domain pack as approved;
- own individual eligibility, legal, clinical, operational, engineering,
  election, or diplomatic decisions; or
- fill an unassigned domain with generic portfolio authority.

## Active-lane ownership

`TAXLANE` is the fiscal owner for every row below. The domain column owns only
the named subject semantics; it does not own TAXLANE allocation or rate
semantics.

| Active lane ID | Domain owner or gap | Evidence TAXLANE may consume | Explicit non-ownership boundary |
|---|---|---|---|
| `national-defense` | [BASTION](https://github.com/giodl73-repo/BASTION) | Public, aggregate, unclassified readiness and lifecycle evidence in a held pack. | TAXLANE does not plan forces, procure, operate, or infer readiness from spending. |
| `international-affairs` | [ENVOY](https://github.com/giodl73-repo/ENVOY) | Assistance obligation, disbursement, output, outcome, safeguard, and local-ownership evidence. | TAXLANE does not issue awards, diplomatic instructions, country rankings, or delivery claims from budget totals. |
| `science-space-technology` | **Domain owner gap.** TAXLANE owns only the OMB crosswalk and fiscal labels. | OMB function records and separately admitted source packets. | No portfolio repository currently owns science or space program performance; outcome or target claims remain held. |
| `environment-energy-natural-resources` | [PYLON](https://github.com/giodl73-repo/PYLON) for transmission, [BASIN](https://github.com/giodl73-repo/BASIN) for water, and [DRAIN](https://github.com/giodl73-repo/DRAIN) for sanitation and receiving-water impacts. | Domain-scoped service, gap, resilience, access, and cost evidence. | None of these repositories owns the whole OMB lane; TAXLANE must preserve the narrower domain and cannot infer environmental or energy outcomes from spending. |
| `agriculture` | **Domain owner gap.** TAXLANE owns only the OMB crosswalk and fiscal labels. | OMB agriculture-function records and separately admitted source packets. | No portfolio repository currently owns federal agriculture-program semantics; production, nutrition, conservation, or farm-income claims remain held. |
| `commerce-housing-credit` | **Domain owner gap.** TAXLANE owns only the OMB crosswalk and fiscal labels. | OMB commerce, housing-credit, postal, deposit-insurance, and offset records. | No portfolio repository currently owns this mixed lane; TAXLANE must not convert net or credit accounting into market-performance or housing-outcome claims. |
| `transportation` | [ROUTE](https://github.com/giodl73-repo/ROUTE) for roads, [GAUGE](https://github.com/giodl73-repo/GAUGE) for rail, [TARMAC](https://github.com/giodl73-repo/TARMAC) for aviation, and [HARBOR](https://github.com/giodl73-repo/HARBOR) for maritime gateways. | Mode-specific corpus, service-promise, gap, access, resilience, and cost evidence. | TAXLANE does not design, approve, build, operate, or collapse unlike modes into one performance score. |
| `community-regional-development` | [ZONES](https://github.com/giodl73-repo/ZONES) owns civic-boundary evidence only; the broader program domain is **unassigned**. | Boundary, jurisdiction, and geography evidence plus OMB program records. | ZONES does not own disaster relief, insurance, housing, or regional-program outcomes; those claims remain held pending an owner. |
| `education-training-employment-social-services` | [SLATE](https://github.com/giodl73-repo/SLATE) owns education-access and pathway evidence; employment and social-service semantics remain **unassigned**. | Aggregate education access, capacity, workforce, affordability, continuity, and equity evidence. | TAXLANE and SLATE do not make student decisions, accreditation findings, pedagogy claims, or general employment/social-service outcome claims. |
| `health` | [SHIELD](https://github.com/giodl73-repo/SHIELD) | Aggregate care-access, capacity, referral, affordability, continuity, quality, equity, and resilience evidence. | TAXLANE does not make clinical, payer, licensing, accreditation, Certificate-of-Need, or patient decisions. |
| `medicare` | [SHIELD](https://github.com/giodl73-repo/SHIELD) owns care-delivery evidence; TAXLANE retains financing distinctions. | Aggregate service-network evidence and source-custodied Medicare financing records. | SHIELD does not own Medicare eligibility or payer policy; TAXLANE cannot infer care quality from trust-fund or outlay records. |
| `income-security` | [LIFELINE](https://github.com/giodl73-repo/LIFELINE) | Aggregate earnings, benefits, access, adequacy, notice, appeal, incidence, delivery, and resource-cliff evidence. | TAXLANE does not determine eligibility or relabel lost benefits, reduced participation, or shifted household burden as savings. |
| `social-security` | [ANCHOR](https://github.com/giodl73-repo/ANCHOR) | Aggregate retirement/disability service, adequacy, payment-realization, accounting, distribution, and delivery evidence. | TAXLANE does not determine benefits, adjudicate claims, or treat a balanced ledger as proof of adequacy or access. |
| `veterans` | [COVENANT](https://github.com/giodl73-repo/COVENANT) | Aggregate referral, intake, claims, notice, appeal, access, continuity, and durable-outcome evidence. | TAXLANE does not make clinical, claims, eligibility, appeal, or service-delivery decisions. |
| `justice-general-government` | [TRIBUNAL](https://github.com/giodl73-repo/TRIBUNAL) owns justice caseflow and rights-floor evidence; general-government performance remains **unassigned**. | Aggregate workload, resolution, counsel, notice, liberty, disparity, and capacity evidence. | TAXLANE does not decide cases, optimize away rights, or infer justice quality from funding or throughput. |
| `net-interest` | [TAXLANE](https://github.com/giodl73-repo/TAXLANE), using source-custodied Treasury and OMB accounting. | Interest outlays, debt and deficit context, time basis, and accounting definitions. | This is financing cost, not a service or benefit outcome and not evidence for a domain repository. |
| `undistributed-offsetting-receipts` | [TAXLANE](https://github.com/giodl73-repo/TAXLANE), using source-custodied OMB accounting. | Offsetting-receipt amounts, account roles, sign treatment, and reconciliation evidence. | Offsets must not be assigned to service lanes or relabeled as efficiency, fraud reduction, or program savings without separate evidence. |

An ownership gap is a release constraint, not permission for TAXLANE to fill the
domain. A new owner requires a repository-level scope, evidence contract, and
review before this map changes.

## Cross-cutting experiment boundaries

| TAXLANE surface | External owner | TAXLANE ownership | Boundary |
|---|---|---|---|
| Annual budget ballot | [RATIFY](https://github.com/giodl73-repo/RATIFY) owns citizen-lawmaking and consent-rule research. [RCOUNT](https://github.com/giodl73-repo/RCOUNT) would own an adopted count/audit contract. | Lane definitions, fiscal closure, financing labels, and budget inputs. | The current Electoral College-weighted run is synthetic institutional simulation, not measured opinion, an election forecast, or an RCOUNT adoption. |
| Geographic plan or state-weight context | [RPLAN](https://github.com/giodl73-repo/RPLAN) owns adopted plan/context document semantics. | Fiscal aggregation only after a plan is admitted. | TAXLANE does not draw districts, certify geography, or silently treat National Archives weights as an RPLAN document. |
| REV Level-3 calculator | Tax-Calculator and `behresp` own their external model contracts. | Versioned assumptions, bounded execution, custody, and interpretation of TAXLANE experiment outputs. | Results are model estimates, not JCT/CBO scores, fiscal-year cash receipts, forecasts, or individual tax advice. |

## Handoff requirements

A domain handoff must identify:

1. provider repository and domain;
2. schema or artifact version;
3. source and observation status;
4. authority flags for admission, allocation, savings, rate, and release;
5. time, population, geography, and accounting basis;
6. unresolved floors or ownership gaps; and
7. the provider command that reproduces the pack.

TAXLANE preserves those fields and adds its own fiscal review. It cannot broaden
the provider's claim by renaming or aggregating it.

The role-review decision for this map is recorded in
`reviews/2026-08-17-jurisdiction-map-role-review.md`.
