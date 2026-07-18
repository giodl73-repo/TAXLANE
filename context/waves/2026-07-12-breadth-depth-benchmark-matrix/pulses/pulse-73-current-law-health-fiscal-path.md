# Pulse 73 — Current-Law Health Fiscal Path

## Decision

Create the initial current-law health fiscal scenario path for FY2025-FY2036
without inventing a federal reform score or collapsing Medicare HI into combined
Medicare.

## Added

- A machine-readable `health_fiscal_scenario_path` record.
- A schema note and public reader.
- Validator coverage for the horizon, component split, FY2025 fixtures, source
  custody, null missingness, and zero current-law reform deltas.

## Boundary

The branch uses only local official-source custody. OMB Table 3.2 supports
non-Medicare health and combined Medicare context through FY2031. It does not
provide a fiscal-year HI/SMI split, so Medicare HI and SMI component outlays
remain null. FY2032-FY2036 health component values also remain null because no
local official annual component source is captured for those years.

No interpolation, reform score, stress score, federal savings claim, or solver
eligibility is opened.
