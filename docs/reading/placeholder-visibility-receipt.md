# Placeholder Visibility Receipt

## What This Is

This is a design prototype for a TAXLANE visibility receipt. It uses a
placeholder `$1,000` ordinary individual income-tax payment and FY2025 OMB
outlay shares.

It is not tax advice, legal advice, accounting advice, a taxpayer calculator, or
evidence that current income-tax dollars are legally assigned to these lanes.

## Method

The prototype uses:

```text
placeholder ordinary income-tax payment
  x FY2025 TAXLANE lane share of OMB Table 3.2 outlays
```

The lane shares come from the draft lane crosswalk and the FY2025 subfunction
model. Borrowed-share context comes from the broad FY2025 income-tax outlay
model.

## Placeholder Result

| Lane | Share | Placeholder amount |
|---|---:|---:|
| Social Security | 22.55% | $225.45 |
| Medicare | 14.22% | $142.16 |
| Health and Care | 13.96% | $139.57 |
| Debt Service and Past Commitments | 13.84% | $138.36 |
| National Defense and Security | 13.07% | $130.67 |
| Income Security and Family Support | 10.01% | $100.07 |
| Veterans and Service Commitments | 5.38% | $53.80 |
| Transportation and Mobility Infrastructure | 2.07% | $20.73 |
| Constitutional Government and Justice | 1.76% | $17.64 |
| Natural Resources, Energy, and Environment | 1.58% | $15.77 |
| Community, Disaster, and Regional Development | 1.17% | $11.75 |
| Education, Work, and Social Services | 1.03% | $10.28 |
| Agriculture and Food Production | 0.68% | $6.77 |
| International Affairs and Diplomacy | 0.64% | $6.44 |
| Science, Space, and Civic Capacity | 0.60% | $5.99 |
| Commerce, Housing, and Market Stability | -0.40% | -$4.01 |
| Undistributed Offsetting Receipts | -2.14% | -$21.43 |

Rounded lane dollars may differ from exactly `$1,000` by a cent because each
lane is rounded independently.

## Financing Context

For FY2025, the broad model reports:

| Context | Value |
|---|---:|
| Borrowed share of outlays | 25.31% |
| Individual income-tax coverage of outlays | 37.88% |

That means this receipt must not be shown alone as if current income taxes paid
for all FY2025 outlays. Borrowing covered part of current spending, and net
interest is a financing cost rather than a program service.

## Required Wording

Use:

> This placeholder receipt models how an ordinary income-tax payment would look
> if allocated by FY2025 outlay shares. It is not legal dedication or dollar
> tracing.

Avoid:

> Your income taxes paid $225.45 for Social Security.

That wording hides the modeled allocation method and the distinct financing
structure of Social Security.

## Artifact

Canonical draft:

`data/derived/taxpayer_receipt_model/taxpayer_receipt_model.placeholder-1000.fy2025.omb-fy2027-v1.draft.json`
