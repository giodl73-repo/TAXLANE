# Denominator Requirements Schema

One JSON object per line.

| Field | Type | Meaning |
|---|---|---|
| `record_id` | string | Stable row ID. |
| `record_family` | string | Always `denominator_requirements`. |
| `display_basis` | string | Public display basis. |
| `denominator_id` | string | Machine-readable denominator key. |
| `plain_label` | string | Reader-facing denominator label. |
| `required_for_lanes` | array[string] | Lanes or lane types that need this denominator. |
| `source_need` | string | Source family needed before numeric display. |
| `current_status` | string | `needed_not_extracted` or `partly_sourced`. |
| `public_use_rule` | string | Guardrail for display. |

## Rule

Any receipt or dashboard using a denominator must cite the denominator row and
source.

## Denominator Value Rows

Numeric denominator values use `record_family = denominator_values`.

| Field | Type | Meaning |
|---|---|---|
| `record_id` | string | Stable row ID. |
| `record_family` | string | Always `denominator_values`. |
| `denominator_requirement_id` | string | Requirement row that this value satisfies or partially satisfies. |
| `display_basis` | string | Public display basis. |
| `denominator_id` | string | Exact denominator key, for example `tax_returns_filed`. |
| `plain_label` | string | Reader-facing denominator label. |
| `year` | string | Year label, preserving tax-year/calendar-year/fiscal-year basis. |
| `year_basis` | string | `tax_year`, `calendar_year`, or `fiscal_year`. |
| `value` | number | Denominator value in `unit`. |
| `unit` | string | Count unit, for example `returns`, `people`, `workers`, `enrollees`, or `households`. |
| `source_ids` | array[string] | Source ledger IDs. |
| `source_table_refs` | array[string] | Optional source table references when available. |
| `status` | string | Draft/source status. |
| `public_use_rule` | string | Guardrail for display. |
| `notes` | string | Caveats, especially cross-year limits. |

## Per-Unit Display Readiness Rows

Per-unit display rows use `record_family = per_unit_display_readiness`.

| Field | Type | Meaning |
|---|---|---|
| `record_id` | string | Stable row ID. |
| `record_family` | string | Always `per_unit_display_readiness`. |
| `display_status` | string | `ready_same_source_year_basis`, `illustrative_cross_basis`, or `blocked_missing_denominator`. |
| `lane_id` | string | TAXLANE lane or sub-lane. |
| `public_label` | string | Reader-facing display label. |
| `numerator_label` | string | Numerator description. |
| `numerator_value` | number | Numerator value in `numerator_unit`. |
| `numerator_unit` | string | Unit for numerator. |
| `denominator_id` | string | Denominator key. |
| `denominator_value` | number or null | Denominator value when sourced. |
| `denominator_unit` | string | Denominator unit. |
| `computed_value_usd` | number or null | Computed dollars per denominator unit. |
| `year` | string | Year label. |
| `year_basis` | string | Basis relationship for numerator and denominator. |
| `source_ids` | array[string] | Source ledger IDs. |
| `source_record_ids` | array[string] | Derived row IDs used. |
| `public_use_rule` | string | Guardrail for display. |

## Per-Unit Receipt Card Rows

Per-unit card rows use `record_family = per_unit_receipt_cards`.

| Field | Type | Meaning |
|---|---|---|
| `record_id` | string | Stable row ID. |
| `record_family` | string | Always `per_unit_receipt_cards`. |
| `source_readiness_record_id` | string | Readiness row backing the card. |
| `card_status` | string | `source_basis_context`, `illustrative_cross_basis`, or `blocked_missing_denominator`. |
| `lane_id` | string | TAXLANE lane or sub-lane. |
| `headline` | string | Public-facing card sentence. |
| `amount_usd` | number or null | Rounded display amount when allowed. |
| `basis_label` | string | Visible numerator/denominator basis. |
| `visible_caveat` | string | Required visible caveat. |
| `allowed_public_use` | string | Where the card may be used. |
| `blocked_public_use` | string | Misuse to avoid. |
