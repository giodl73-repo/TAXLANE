# TAXLANE Data Dictionary

## Purpose

This dictionary defines the record families TAXLANE needs before extracting
rates, receipts, outlays, and public-purpose lane records. It is a schema
contract for future data work, not a dataset.

## Record family map

| Family | Year basis | First sources | Public use |
|---|---|---|---|
| `history_event` | Calendar/legal date | Constitutional, statutory, court, and IRS history sources | Explain authority and administrative milestones. |
| `rates_timeline` | Tax year | `SRC-IRS-SOI-HT23`, statutes | Explain rate schedules and tax-base changes over time. |
| `receipt_source` | Fiscal year | OMB Historical Tables 1.1, 2.1, 2.2, 2.4 | Show federal receipts by source. |
| `fund_group` | Fiscal year | OMB Historical Table 1.4; OMB fund concepts | Separate general fund, trust fund, special fund, and other fund treatment. |
| `budget_concept` | Source concept | OMB Analytical Perspectives; GAO budget glossary | Define budget-accounting concepts used to interpret extracted rows. |
| `outlay_function` | Fiscal year | OMB Historical Tables 3.1, 3.2 | Show spending by public-purpose function and subfunction. |
| `outlay_program` | Fiscal year | OMB Historical Tables 8.5, 8.7, 11.3; USAspending | Show mandatory, discretionary, beneficiary, and program detail. |
| `lane_crosswalk` | Fiscal year or model version | OMB functions plus TAXLANE lane taxonomy | Map public-purpose labels to budget functions. |
| `taxpayer_receipt_model` | Tax year plus fiscal year | Derived from rates, receipts, outlays, and lane crosswalks | Explain a taxpayer payment with method labels. |
| `program_lane_model` | Model version | Derived from lane model and statutory design sources | Evaluate possible program-linked tax reforms. |

## Common fields

Every extracted or modeled record should carry these fields:

| Field | Meaning |
|---|---|
| `record_id` | Stable repo-local identifier. |
| `record_family` | One of the families in this dictionary. |
| `source_ids` | One or more IDs from the source-version ledger. |
| `observed_date` | Date TAXLANE observed or fetched the source. |
| `coverage_year` | Tax year, fiscal year, legal year, or model year as applicable. |
| `year_basis` | `tax_year`, `fiscal_year`, `calendar_year`, `legal_date`, or `model_version`. |
| `status` | `draft`, `extracted`, `reviewed`, `superseded`, or `retired`. |
| `notes` | Short source or modeling caveat. |

## `history_event`

Use for legal and administrative milestones.

| Field | Required | Meaning |
|---|---|---|
| `event_date` | Yes | Exact date if known; otherwise year plus caveat. |
| `event_class` | Yes | `wartime-revenue-experiment`, `repeal-expiration`, `constitutional-constraint`, `constitutional-authorization`, `statutory-reauthorization`, `mass-tax-expansion`, `withholding-administration`, `code-consolidation`, or `code-reform`. |
| `authority_source_id` | Yes | Constitutional, statutory, court, or official-history source. |
| `tax_effect_summary` | Yes | What changed about authority, base, rates, collection, or administration. |
| `allocation_status` | Yes | Whether the event created a tax authority, receipt source, dedication, or no allocation rule. |

## `rates_timeline`

Use for tax-year income-tax rates and tax-base context.

| Field | Required | Meaning |
|---|---|---|
| `tax_year` | Yes | Tax year of the rate record. |
| `filing_status` | Yes when available | Filing status or historical equivalent. |
| `rate_percent` | Yes | Statutory or table rate. |
| `bracket_floor` | Yes when available | Lower taxable-income threshold. |
| `bracket_ceiling` | Yes when available | Upper taxable-income threshold. |
| `threshold_units` | Yes | Dollar basis, nominal versus inflation-adjusted if applicable. |
| `tax_base_note` | Yes | Exemption, deduction, surtax, normal tax, or other base context. |
| `source_table` | Yes | IRS table, statute page, or extracted table reference. |

Rules:

- Do not mix nominal and inflation-adjusted values in one field.
- Do not call top marginal rate an average rate.
- Do not publish extracted brackets until the source row and table version are
  recorded.

## `receipt_source`

Use for federal receipts by source.

| Field | Required | Meaning |
|---|---|---|
| `fiscal_year` | Yes | Federal fiscal year. |
| `receipt_category` | Yes | OMB receipt source category. |
| `amount` | Yes | Reported amount. |
| `amount_units` | Yes | Dollars, millions, billions, or percentage. |
| `actual_or_projection` | Yes | `actual`, `estimate`, or `projection`. |
| `fund_group_link` | Optional | Link to fund-group record when available. |

Rules:

- Keep individual income taxes separate from corporation income taxes and
  social-insurance receipts.
- Label projections separately from actuals.
- Do not use receipt-source records as program allocation by themselves.

## `fund_group`

Use for general fund, trust fund, special fund, and related budget treatment.

| Field | Required | Meaning |
|---|---|---|
| `fiscal_year` | Yes | Federal fiscal year. |
| `fund_group` | Yes | General fund, trust fund, special fund, revolving fund, or other. |
| `legal_dedication` | Yes | `none`, `dedicated`, `mixed`, or `unknown`. |
| `appropriation_required` | Yes when known | Whether obligation requires appropriation. |
| `budget_concept_source_id` | Yes | OMB or GAO concept source. |

Rules:

- Trust fund does not mean private trust ownership.
- Legal dedication does not imply automatic spending unless the authority says
  so.

## `budget_concept`

Use for source-grounded budget-accounting concepts that support interpretation
of receipts, fund groups, offsetting treatment, and allocation caveats.

| Field | Required | Meaning |
|---|---|---|
| `concept_key` | Yes | Stable concept key. |
| `source_ref` | Yes | Page, section, table, or other source anchor. |
| `concept_summary` | Yes | Paraphrased source-grounded concept summary. |
| `fund_group_implication` | Yes | How the concept affects fund-group interpretation. |
| `legal_dedication_implication` | Yes | What, if anything, the source supports about legal dedication. |
| `appropriation_implication` | Yes | What, if anything, the source supports about appropriation or spending availability. |
| `public_caveat` | Yes | Plain-language caveat for downstream public use. |

Rules:

- Keep concept records separate from numeric OMB historical table rows.
- Do not treat a broad OMB concept as proof for a specific statute or account.
- Use paraphrases unless a public note needs a short source excerpt.

## `outlay_function`

Use for spending by OMB function and subfunction.

| Field | Required | Meaning |
|---|---|---|
| `fiscal_year` | Yes | Federal fiscal year. |
| `superfunction` | Yes when source provides | OMB superfunction. |
| `function` | Yes | OMB function. |
| `subfunction` | Yes when source provides | OMB subfunction. |
| `amount` | Yes | Reported outlay amount. |
| `amount_units` | Yes | Dollars, millions, billions, or percentage. |
| `actual_or_projection` | Yes | `actual`, `estimate`, or `projection`. |

Rules:

- Use outlays for "what government spent money on" views.
- Keep net interest visible as its own function.
- Keep undistributed offsetting receipts labeled if present in the source.

## `lane_crosswalk`

Use to map public TAXLANE labels to OMB budget categories.

| Field | Required | Meaning |
|---|---|---|
| `lane_id` | Yes | Stable TAXLANE public-purpose lane key. |
| `public_label` | Yes | Reader-facing lane name. |
| `omb_functions` | Yes | OMB functions/subfunctions included. |
| `receipt_sources` | Yes | Receipt sources normally associated with the lane, if any. |
| `spending_control` | Yes | Discretionary, mandatory, trust-fund, net-interest, offsetting, mixed. |
| `legal_status_today` | Yes | General fund, dedicated, mixed, modeled, or non-tax context. |
| `allocation_method_allowed` | Yes | Which allocation methods can be used for this lane. |

Rules:

- A lane can be useful without being a current legal destination.
- Borrowed Share / Deficit Gap is required context, not a tax lane.

## `taxpayer_receipt_model`

Use for future public receipt prototypes.

| Field | Required | Meaning |
|---|---|---|
| `model_id` | Yes | Stable model identifier. |
| `tax_year` | Yes | Taxpayer-side year. |
| `fiscal_year` | Yes | Budget-side year used for allocation. |
| `tax_source` | Yes | Individual income tax, payroll tax, or other source. |
| `tax_amount_basis` | Yes | Actual taxpayer amount, placeholder amount, or scenario amount. |
| `allocation_method` | Yes | Legal dedication, proportional, deficit-inclusive, illustrative, or proposed. |
| `deficit_treatment` | Yes | Excluded, shown separately, allocated proportionally, or other. |
| `lane_allocations` | Yes | Linked lane allocation records. |
| `public_caveat` | Yes | Plain-language caveat shown with the receipt. |

Rules:

- A receipt without allocation labels is invalid.
- A tax-only receipt without deficit context is incomplete.
- Placeholder amounts must be labeled as placeholder amounts.

## `program_lane_model`

Use for future reform proposals.

| Field | Required | Meaning |
|---|---|---|
| `model_id` | Yes | Stable model identifier. |
| `lane_id` | Yes | Public-purpose lane being modeled. |
| `tax_base` | Yes | Proposed tax base. |
| `rate_rule` | Yes | Fixed, formula-based, annually reset, or other. |
| `receipt_account` | Yes | Proposed account or fund destination. |
| `appropriation_rule` | Yes | Annual, permanent, limited, standing, or mixed. |
| `outlay_target` | Yes | Program, function, account, or formula target. |
| `shortfall_rule` | Yes | What happens when receipts are insufficient. |
| `surplus_rule` | Yes | What happens when receipts exceed target. |
| `reserve_rule` | Yes | Balance, reserve, or investment treatment. |
| `borrowing_rule` | Yes | Borrowing permission or prohibition. |
| `override_rule` | Yes | Congressional override or reallocation path. |
| `beneficiary_impact` | Yes | Service, benefit, or continuity implications. |
| `taxpayer_burden` | Yes | Taxpayer complexity. |
| `employer_burden` | Yes | Employer or withholding complexity. |
| `preparer_burden` | Yes | Tax-preparation complexity. |
| `agency_burden` | Yes | IRS, Treasury, OMB, and program-agency complexity. |
| `role_review_status` | Yes | Required role review state. |

Rules:

- No statutory lane proposal may omit shortfall, surplus, reserve, override, or
  burden fields.
- No proposal may claim fiscal discipline without evidence.
- Role review is required before public advocacy.

## Extraction validation gates

Before importing raw data:

1. Confirm the source exists in `docs/sources/source-version-ledger.md`.
2. Confirm the record family and required fields are defined here.
3. Record whether the source is actual, estimate, projection, contextual, or
   candidate.
4. Keep raw files out of the repo unless a custody review approves location,
   naming, and update rules.
5. Run `git diff --check`.

Before publishing public claims:

1. Link the claim to a record family.
2. Link the record to source IDs.
3. Label the allocation method.
4. State tax-year versus fiscal-year basis.
5. Include deficit context when the claim allocates spending.
