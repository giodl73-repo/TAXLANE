# Lane Crosswalk Role Review

## Scope

This review covers the first public-purpose lane crosswalk:

| Artifact | Role |
|---|---|
| `data/derived/lane_crosswalk/lane_crosswalk.omb-fy2027-v1.2026-06-22.draft.jsonl` | Draft lane mapping records. |
| `data/derived/lane_crosswalk/README.md` | Method and public-use boundary. |
| `docs/research/2026-06-22-public-purpose-lane-crosswalk-note.md` | Crosswalk decision note. |
| `docs/data/outlays-lanes-schema.md` | `lane_crosswalk` schema and method values. |

## Decision

The crosswalk is acceptable as a draft mapping input for later visibility
receipt design.

This review does not approve a taxpayer receipt, legal dedication claim,
program-level tracing claim, statutory tax lane, or tax-rate recommendation.

## Findings

| Role | Result |
|---|---|
| Taxpayer Advocate | Pass with caution: public labels are legible, but the artifact must remain behind a receipt view that shows allocation labels at display time. |
| Budget Accountant | Pass: the crosswalk separates mapping from allocation and keeps net interest and offsetting receipts visible. |
| Source Custodian | Pass: source IDs point to recorded OMB FY2027 historical tables and budget-concept sources. |
| Public Goods Steward | Pass: lanes connect to public purposes without saying every purpose needs a dedicated tax. |
| Program Beneficiary | Pass with caution: Medicare, Social Security, veterans, and income-security lanes remain distinct enough for later beneficiary-impact review. |
| Compliance Burden | Pass: the crosswalk does not add taxpayer filing, withholding, employer, or preparer requirements. |
| Fiscal Sustainability | Pass: every lane requires deficit context, and net interest remains its own lane. |
| Reform Skeptic | Pass: the notes preserve fungibility and modeled-not-legal boundaries. |

## Required Guardrails

### P1: Do not use the crosswalk as a receipt by itself

- **Roles**: Budget Accountant, Reform Skeptic, Taxpayer Advocate.
- **Evidence**: The crosswalk maps OMB functions and subfunctions to TAXLANE
  lane labels, but it does not include taxpayer amounts, allocation shares, or
  receipt-source treatment.
- **Risk**: A reader could interpret a lane label as "my tax legally paid for
  this" if the crosswalk is displayed without method labels.
- **Decision**: Any public receipt prototype must add allocation method, legal
  status, fiscal-year basis, and deficit context at the point of display.

### P2: Keep offsetting and financing rows visible

- **Roles**: Budget Accountant, Fiscal Sustainability Reviewer.
- **Evidence**: The crosswalk uses `display_separately` for net interest and
  undistributed offsetting receipts.
- **Risk**: Rolling those rows into service lanes would hide financing cost or
  budget netting mechanics.
- **Decision**: Later rollups may simplify labels, but they must preserve a
  visible debt-service lane and a visible offsetting/adjustment treatment.

### P2: Review public rollups before shortening the lane set

- **Roles**: Taxpayer Advocate, Public Goods Steward, Reform Skeptic.
- **Evidence**: The first crosswalk is function-aligned and has 17 rows. Public
  receipt displays may need fewer top-level lanes.
- **Risk**: A smaller display could collapse unlike funding modes, such as
  Social Security trust-fund financing and ordinary income-tax general receipts.
- **Decision**: Any future rollup needs a separate role review before it becomes
  the default public receipt view.

## Follow-Up

- Build the first visibility receipt prototype only as a modeled scenario with
  placeholder or ordinary individual income-tax amounts.
- Keep dynamic Treasury, USAspending, and CBO sources out of the receipt
  prototype until query snapshot rules exist.
