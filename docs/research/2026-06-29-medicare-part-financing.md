# Medicare Part Financing Split

## Decision Supported

Medicare should not be represented as one payroll-funded lane. The 2026 Medicare
Trustees Report gives the source-specific CY2025 financing split needed to keep
Hospital Insurance, Part B, and Part D separate.

## CY2025 Trust-Fund Operations

| Part | Alignment | Income | Expenditures | Main financing signal |
|---|---|---:|---:|---|
| HI / Part A | Contributory earned benefit | $462.4B | $444.2B | Dedicated HI financing; closest Medicare component to pay-in/pay-out. |
| SMI / Part B | Premium plus general support | $580.5B | $584.3B | $150.3B premiums plus $422.2B government contributions. |
| SMI / Part D | Premium plus general support | $183.317B | $181.531B | $14.862B premiums, $148.844B government contributions, and $19.087B state payments. |

## What This Fixes

This confirms the earlier design rule:

- HI / Part A can be discussed as the contributory Medicare lane.
- Part B and Part D are not payroll-funded lanes; they are premium-plus-general
  support lanes.
- A public receipt must split Medicare before claiming "proportional to what
  goes in."

## Basis Warning

These are calendar-year trust-fund financial operations from the Trustees
report. They are not the same basis as FY2025 OMB outlay rows. TAXLANE can use
them for financing alignment, but it should not directly replace the OMB FY2025
outlay allocation until a calendar-year/fiscal-year bridge is built.

## Remaining Blocker

The denominator layer is still incomplete. Part A/B/D per-enrollee or
per-beneficiary claims remain blocked until CMS or Trustees enrollment counts
are extracted and registered.
