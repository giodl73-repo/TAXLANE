# PaymentAccuracy Annual Dataset Extraction Schema

## Purpose

Extract annual agency-reported payment-integrity fields from the official OMB
PaymentAccuracy workbook without converting improper payments into fraud,
recoverable dollars, collections, prevented loss, or net savings.

## Source grain

One workbook per reporting fiscal year. Preserve every source worksheet and
column name. Extract at the finest published agency/program/payment-type/root-
cause grain and retain the source row or cell anchor.

## Required custody fields

- source ID, workbook fiscal year, publisher filename, canonical URL;
- observed date, raw path, SHA-256 checksum, worksheet, and source row;
- agency and program identifiers and labels;
- reporting period and any sample or measurement period;
- source field name, source value, source unit, and null meaning; and
- extraction and review status.

## Controlled evidence classes

| Class | Rule |
|---|---|
| improper payment | Preserve the publisher definition and coverage universe. |
| overpayment | Extract only when explicitly typed by the source. |
| underpayment | Extract only when explicitly typed by the source. |
| unknown payment | Keep distinct from overpayment and underpayment. |
| root cause | Not a disjoint dollar class unless the source says it is. |
| confirmed fraud | Null unless linked to an adjudicated finding at matching grain. |
| recoverable/collectible | Null unless the source quantifies the applicable basis. |
| collected recovery | Keep separate from estimates and debt established. |
| control cost | Include implementation and appeal cost when published. |
| prevented future loss | Prospective estimate only, with method and period. |
| net savings | Requires benefits less control, administration, access, error, and appeal costs. |

## Validation

- Totals may reconcile only within identical workbook definitions and periods.
- Never force missing source fields to zero.
- Never sum overlapping root-cause and payment-type tables.
- Program probes with different sample periods remain separately labeled.
- Public fraud and savings fields remain blocked unless their own evidence gate
  is satisfied.
