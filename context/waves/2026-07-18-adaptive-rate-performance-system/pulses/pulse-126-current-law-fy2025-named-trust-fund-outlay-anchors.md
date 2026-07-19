# Pulse 126 — FY2025 named trust-fund outlay anchors

## Result

Added a source-custodied FY2025 named trust-fund outlay anchor packet from the
OMB Public Budget Database outlay workbook.

## Data populated

- OASI outlay anchor: `$1,421.591B`.
- DI outlay anchor: `$160.199B`.
- OASDI outlay anchor sum: `$1,581.790B`.
- Medicare HI outlay anchor: `$444.832B`.

## Boundaries

- These are outlay anchors only, not complete trust-fund paths.
- OASI and DI remain account-level rows and are summed only as an OASDI anchor.
- Medicare HI remains separate from SMI and other Medicare.
- Transportation remains blocked because the available PBD rows are fragmented
  across highway, mass transit, airport, interest, and offset accounts.
- Fund balances, explicit transfers, credited offsetting collections, reserves,
  solver inputs, target costs, rates, and public fiscal claims remain null or
  false.
- No external request was submitted and no agency or person was contacted.

## Validation

The validator checks raw custody, recomputes source-unit conversion, account
sums, OASDI and Medicare HI anchor totals, and verifies that all public warning
phrases remain present.
