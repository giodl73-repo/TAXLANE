# Contribution Alignment Schema

One JSON object per line.

| Field | Type | Meaning |
|---|---|---|
| `record_id` | string | Stable row ID. |
| `record_family` | string | Always `contribution_alignment`. |
| `fiscal_year` | number | Fiscal-year context. |
| `lane_id` | string | TAXLANE lane or sub-lane ID. |
| `public_label` | string | Human label. |
| `alignment_type` | string | `contributory_earned_benefit`, `premium_plus_general_support`, `redistributive_transfer`, `public_good`, `financing_cost`, or `mixed_requires_split`. |
| `payer_base` | string | Who pays or which receipt base funds the lane. |
| `beneficiary_base` | string | Who directly benefits, or `public`. |
| `per_person_denominator_needed` | array[string] | Denominators needed before per-person display. |
| `current_alignment_status` | string | Current alignment verdict. |
| `required_fix` | string | What would make the lane more aligned or more honest. |
| `legal_dedication_status` | string | Legal dedication or modeled allocation status. |
| `source_ids` | array[string] | Current source IDs. |
| `public_claim_status` | string | Current value: `blocked_design_surface_only`. |

## Rule

Per-person display is blocked until denominators are extracted and labeled.

## Medicare Source Boundary Rows

Medicare boundary checks use `record_family = medicare_source_boundary`.

| Field | Type | Meaning |
|---|---|---|
| `record_id` | string | Stable row ID. |
| `record_family` | string | Always `medicare_source_boundary`. |
| `fiscal_year` | number | Fiscal year. |
| `year_basis` | string | Always `fiscal_year`. |
| `public_label` | string | Reader-facing label. |
| `table_3_2_medicare_subfunction_outlays_musd` | number | OMB Table 3.2 Medicare subfunction outlays in millions of dollars. |
| `table_8_5_medicare_mandatory_program_outlays_musd` | number | OMB Table 8.5 Medicare mandatory-program outlays in millions of dollars. |
| `difference_musd` | number | Table 3.2 minus Table 8.5, in millions of dollars. |
| `difference_percent_of_subfunction` | number | Difference as a percent of the Table 3.2 value. |
| `source_ids` | array[string] | Source ledger IDs. |
| `interpretation` | string | Boundary interpretation. |
| `public_use_rule` | string | Guardrail for display. |
| `next_source_needed` | string | Source required to finish the Part A/B/D split. |
| `status` | string | Draft/source status. |

## Medicare Part Financing Rows

Medicare part-financing records use `record_family = medicare_part_financing`.

| Field | Type | Meaning |
|---|---|---|
| `record_id` | string | Stable row ID. |
| `record_family` | string | Always `medicare_part_financing`. |
| `calendar_year` | number | Calendar year. |
| `year_basis` | string | Always `calendar_year`. |
| `program_part` | string | `HI`, `Part B`, or `Part D`. |
| `public_label` | string | Reader-facing part label. |
| `alignment_type` | string | Contribution alignment classification. |
| `income_busd` | number | Total income in billions of dollars. |
| `expenditures_busd` | number | Total expenditures in billions of dollars. |
| `net_change_busd` | number | Net trust-fund/account change in billions of dollars. |
| `end_assets_busd` | number | Trust-fund/account assets at end of year in billions of dollars. |
| `premium_income_busd` | number or null | Premium income in billions of dollars when applicable. |
| `government_contribution_busd` | number or null | General-fund/government contribution in billions of dollars when applicable. |
| `state_payments_busd` | number or null | State payments in billions of dollars when applicable. |
| `source_ids` | array[string] | Source ledger IDs. |
| `source_table_refs` | array[string] | Trustees report table references. |
| `public_use_rule` | string | Guardrail for display. |
| `status` | string | Draft/source status. |
| `notes` | string | Basis caveats. |
