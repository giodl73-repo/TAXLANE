# Derived Records

This directory is reserved for model outputs and transformed records derived
from reviewed extracted data.

Examples of future derived records:

- Constant-dollar versions of receipt or outlay records.
- Lane rollups from OMB function and subfunction records.
- Taxpayer receipt allocation scenarios.
- Program-linked tax design scenarios.

Derived records are blocked until their input records, derivation method,
allocation labels, and review status are documented.

## Current Derived Models

| Directory | Model | Status |
|---|---|---|
| `income_tax_outlay_model/` | Yearly individual income-tax receipts allocated across broad OMB outlay categories by proportional outlay share, with decade summaries derived from annual rows. | Draft; method and source profile documented. |
| `income_tax_outlay_subfunction_model/` | Yearly individual income-tax receipts allocated across OMB Table 3.2 subfunctions by proportional outlay share. | Draft; method and source profile documented. |
