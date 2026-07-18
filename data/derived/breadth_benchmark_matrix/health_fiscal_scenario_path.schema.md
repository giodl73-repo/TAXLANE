# Health Fiscal Scenario Path Schema

Machine record:
`health_fiscal_scenario_path.v1.draft.json`.

This record defines the current-law health fiscal path for FY2025-FY2036 without
creating a reform score. It must keep three components separate:

- `medicare_hi`;
- `medicare_smi_and_other_medicare`;
- `non_medicare_health_general_fund`.

Each component-year record must include gross program outlays,
implementation/admin outlays, credited offsetting collections, dedicated
receipts, explicit general-fund transfer, reserve contribution, score source,
source vintage, unrounded value, and delta from current law.

Missing values stay `null`. Current-law reform deltas are zero because the
scenario is the unchanged-policy baseline. Combined Medicare values may appear
only as context and must not populate either Medicare HI or Medicare SMI
component outlays.

Reserve contribution fields remain `null` until reserve accounting parameters
are defined. FY2032-FY2036 values remain `null` unless an official annual
component source is captured.
