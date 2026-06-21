# Rates Timeline Schema

## Purpose

The `rates_timeline` record family captures federal individual income-tax rate
schedule history by tax year. It is designed to explain rate layers, statutory
authority, filing-status context, and tax-base caveats without turning top
marginal rates into average-rate claims.

This file defines the extraction schema only. It does not import IRS or statute
data.

## Source anchors

| Source ID | Role |
|---|---|
| `SRC-IRS-SOI-HT23` | Default historical rates and brackets spine. |
| `SRC-GOVINFO-1913-REV-ACT` | Statutory starting point for post-Sixteenth Amendment income tax. |
| `SRC-FRASER-1913-UNDERWOOD` | Accessible copy for 1913 statutory cross-check. |
| `SRC-GOVINFO-1954-IRC` | Code consolidation anchor. |
| `SRC-GOVINFO-1986-TRA` | Modern reform anchor. |
| `SRC-IRS-RATES-BRACKETS` | Current-year IRS bracket explainer and marginal-rate wording. |

Historical statutes before 1913 may appear in `history_event` records first and
only become `rates_timeline` rows after exact statutory table extraction.

## Record identity

Each row should use a deterministic `record_id`:

```text
rates:{tax_year}:{filing_status}:{schedule_part}:{bracket_index}
```

Examples:

- `rates:1913:single:normal-tax:001`
- `rates:2025:married-filing-jointly:ordinary-income:005`

If the source does not expose filing status, use `filing_status = "not_stated"`
and explain the historical context in `tax_base_note`.

## Required fields

| Field | Type | Required | Meaning |
|---|---|---:|---|
| `record_id` | string | Yes | Stable row identifier. |
| `record_family` | string | Yes | Always `rates_timeline`. |
| `tax_year` | integer | Yes | Tax year, not fiscal year. |
| `source_ids` | string list | Yes | Source ledger IDs backing the row. |
| `source_table` | string | Yes | IRS table, statute page/section, or current IRS page section. |
| `source_row_ref` | string | Yes | Human-readable row, page, section, or cell anchor. |
| `filing_status` | string | Yes | Filing status, historical class, or `not_stated`. |
| `income_type` | string | Yes | `ordinary_income`, `capital_gain`, `surtax`, `normal_tax`, `alternative_minimum`, `mixed`, or `unknown`. |
| `schedule_part` | string | Yes | Rate schedule part, for example `ordinary-income`, `normal-tax`, `surtax`. |
| `bracket_index` | integer | Yes | 1-based order within the schedule. |
| `bracket_floor` | decimal or null | Yes | Lower threshold in nominal dollars when available. |
| `bracket_ceiling` | decimal or null | Yes | Upper threshold in nominal dollars when available. |
| `threshold_units` | string | Yes | Usually `nominal_usd`; use explicit value if different. |
| `rate_percent` | decimal | Yes | Rate for the bracket layer. |
| `rate_basis` | string | Yes | `marginal`, `normal`, `surtax`, `effective`, or `unknown`. |
| `tax_base_note` | string | Yes | Exemptions, deductions, normal/surtax split, or other caveat. |
| `authority_event_id` | string or null | Yes | Link to related `history_event` when available. |
| `status` | string | Yes | `draft`, `extracted`, `reviewed`, `superseded`, or `retired`. |
| `observed_date` | date | Yes | Date the source was observed or fetched. |
| `notes` | string | No | Additional extraction caveat. |

## Controlled values

### `filing_status`

Use lower-case stable labels:

- `single`
- `married-filing-jointly`
- `married-filing-separately`
- `head-of-household`
- `surviving-spouse`
- `estate-or-trust`
- `not_stated`
- `historical-other`

If a historical table uses a different category, preserve the source wording in
`notes` and map to the closest stable label only after review.

### `income_type`

Use:

- `ordinary_income`
- `capital_gain`
- `surtax`
- `normal_tax`
- `alternative_minimum`
- `mixed`
- `unknown`

Do not merge normal tax and surtax rows into one effective rate unless a
separate modeled record is explicitly created and labeled.

### `rate_basis`

Use:

- `marginal`
- `normal`
- `surtax`
- `effective`
- `unknown`

TAXLANE public readers should normally use marginal rates. Effective-rate rows
need a separate method note before publication.

## Extraction rules

1. Record `SRC-IRS-SOI-HT23` as the source spine for historical brackets unless
   the row is directly extracted from a statute.
2. For a statutory row, include both the statute source ID and the table source
   ID if the IRS table is being used as a cross-check.
3. Preserve nominal thresholds. Inflation-adjusted thresholds belong in a
   separate derived record family or explicitly labeled derived field.
4. Represent an unbounded top bracket with `bracket_ceiling = null`.
5. Use `bracket_floor = 0` only when the source clearly means the first dollar
   above zero. If the source uses exemptions, put the exemption context in
   `tax_base_note`.
6. Do not infer filing status from modern categories when a historical source
   does not state one.
7. Do not collapse multiple rate schedules into one row.
8. Do not publish a "top rate" chart without linking the top bracket row and
   explaining that it is marginal.

## Validation checks

Before a rates table is marked `reviewed`, check:

1. `tax_year` is an integer and `year_basis` is `tax_year`.
2. Every row has at least one source ID from the ledger.
3. `rate_percent` is greater than or equal to zero.
4. Within a tax year/status/schedule part, `bracket_index` is unique.
5. Bracket floors are nondecreasing within a schedule.
6. Each non-null `bracket_ceiling` is greater than its `bracket_floor`.
7. The final bracket may have null ceiling; earlier brackets should not unless
   the source is incomplete and the row remains `draft`.
8. Any row marked `effective` has a method note.
9. Any historical row with `filing_status = "not_stated"` has a note explaining
   why status is unavailable.

## Public wording rules

| Data condition | Public wording |
|---|---|
| Bracket row | "Income in this layer was taxed at..." |
| Top marginal row | "The top marginal rate was..." |
| Effective-rate derived row | "Under this model, the effective rate was..." |
| Source lacks filing status | "The source does not separate modern filing statuses." |
| Normal/surtax split | "The source used separate normal tax and surtax schedules." |

## Open questions

- Whether to create a separate `rates_derived` family for inflation-adjusted
  thresholds, average-rate models, and sample taxpayer scenarios.
- Whether estate/trust and capital-gain schedule rows should live in this
  family or a later specialized family.
- How to represent years where statutory changes applied midyear or through
  transition rules.
