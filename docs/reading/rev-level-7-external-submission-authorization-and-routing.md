# REV Level 7 external submission authorization and routing

## Routing boundary

JCT is the official revenue estimator for legislation amending the Internal
Revenue Code. The submission must therefore originate through an authorized
congressional requester, normally tax-writing committee or Member staff. CBO's
published guidance says congressional staff may request technical assistance
at `costestimates@cbo.gov`, but also directs Internal Revenue Code requests to
JCT. That address must not be used by Taxlane as an unaffiliated public filing
channel.

Primary route: authorized House Ways and Means, Senate Finance, Budget
Committee, leadership, or Member staff to JCT for legislative drafting and
revenue scoring. Secondary route: the same authorized requester coordinates
with CBO for direct-spending interactions, administration where applicable,
debt service, and integrated budget effects.

Official references:

- JCT revenue-estimating process: <https://www.jct.gov/publications/2025/revenue-estimating-process-january-2025/>
- CBO cost-estimate FAQ: <https://www.cbo.gov/cost-estimates/faqs>
- CBO process description: <https://www.cbo.gov/about/processes>

## Authorization checklist

All fields must be completed and independently verified before any send:

1. Requesting Member, committee, or leadership office.
2. Named congressional staff contact and official contact information.
3. Authority basis and approval date.
4. Authorized signer and exact cover message.
5. Selected JCT route and any coordinated CBO route.
6. Approved bundle ID and SHA-256 digest.
7. Confirmation that no Taxlane planning number is labeled official.
8. Confirmation that the nonofficial discussion draft is not Legislative
   Counsel text.
9. Confirmation that public release, confidentiality, and response-custody
   rules are understood.
10. Explicit authorization for the exact outbound action.

## Receipt and response

Record the transmitted bundle digest, timestamp, sender, recipient, channel,
and receipt identifier immediately after an authorized send. Import no score
until the responding office, legislative text, baseline, vintage, scope, and
annual tables are authenticated in the response-intake record.
