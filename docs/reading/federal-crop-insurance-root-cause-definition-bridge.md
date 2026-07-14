# Federal Crop Insurance Root-Cause Definition Bridge

Machine record:
`data/derived/breadth_benchmark_matrix/federal_crop_insurance_root_cause_definition_bridge.fy2024.v1.draft.json`.

This bridge uses the FCIC section of USDA's FY2024 Agency Financial Report to
close one additional methodology field internally: `data-access
outside-agency-control root-cause definition`.

## What The AFR Adds

The AFR reports FCIC's FY2024 estimated improper-payment rate as 2.43% and its
improper dollars as $579.36M. More importantly for this bridge, it explains the
two data-access root-cause categories rather than merely naming them.

`Failure to access data/information` covers cases where the information
existed, was obtained, and was used in the payment calculation, but an
administrative or calculation error still made the payment improper.

`Inability to access data/information` is described as arising primarily from
certification errors. The certifications establish policy insurance amounts,
premiums, and indemnities, so errors in that information can produce improper
payments.

The source evidence is on printed pages 216-217. Those are PDF pages 220-221
in the canonical USDA rendition and pages 250-251 in the checksum-fixed
GovInfo custody rendition.

## Closure Decision

The two category explanations are sufficiently distinct and program-specific
to close the root-cause-definition field for internal methodology tracking.
Combined with Pulse 24, FCIC now has three closed fields and five open fields.
The open fields are sample design, payment universe, estimation method,
exclusion rules, and recoverable-savings basis.

This closure does not establish how sampled errors project to the full payment
universe or how any estimated improper amount relates to debt, collections,
prevention, or control cost.

## Claim Firewall

The department-wide Do Not Pay section begins immediately after the FCIC
discussion. Its search activity, dollar matches, and user figures cover USDA
programs collectively; none may be assigned to FCIC.

The AFR's corrective-action descriptions are not quantified savings evidence.
Program scoring and public claims about
fraud, recoveries, prevented loss, or net savings remain blocked.
