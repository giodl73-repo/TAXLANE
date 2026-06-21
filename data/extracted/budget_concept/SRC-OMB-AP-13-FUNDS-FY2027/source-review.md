# AP13 Budget Concept Source Review

## Source Custody

- Source ID: `SRC-OMB-AP-13-FUNDS-FY2027`
- Raw artifact:
  `data/raw/omb/SRC-OMB-AP-13-FUNDS-FY2027/2026-06-21/ap_13_funds_fy2027.pdf`
- SHA-256:
  `6A332E8291DB7F8E6A4252C79E444C782F5CF2D369CAE4B738FE63B7DC0D4437`
- Text check:
  `pdftotext -layout -f 1 -l 2 ...`

## Reviewed Records

| Record | Anchor | Review result |
|---|---|---|
| `concept:omb-ap13:federal-funds-group` | PDF page 1, printed page 137, `The Federal Funds Group` | Confirmed. The source supports broad federal-funds treatment and does not make the group equivalent to the general fund. |
| `concept:omb-ap13:general-fund` | PDF page 1, printed page 137, `The Federal Funds Group` | Confirmed. The source supports general-fund treatment for collections not dedicated elsewhere, including ordinary income-tax context. |
| `concept:omb-ap13:special-funds` | PDF page 1, printed page 137, `The Federal Funds Group` | Confirmed. The source supports law-dedicated special fund treatment and states that special fund receipt-account money requires appropriation before obligation and spending. |
| `concept:omb-ap13:revolving-funds` | PDF page 1, printed page 137, `The Federal Funds Group` | Confirmed. The source supports business-like activity, offsetting-collection treatment, and general availability of proceeds without further legislative action. |
| `concept:omb-ap13:trust-funds` | PDF page 1, printed page 137, `The Trust Funds Group` | Confirmed. The source supports law-designated trust funds and dedicated collections for specific purposes. |
| `concept:omb-ap13:federal-trust-fund-caveat` | PDF page 2, printed page 138, `The Trust Funds Group` | Confirmed. The source supports the public caveat that Federal trust-fund terminology differs from common private trust usage. |

## Limits

- These records review OMB budget-accounting concepts, not account statutes.
- These records do not trace any taxpayer payment to a program.
- These records do not resolve appropriation treatment for a specific trust fund,
  special fund, or program account.
- The next interpretation step may use these records to label the first
  fund-group slice more clearly, but legal dedication remains account-specific.
