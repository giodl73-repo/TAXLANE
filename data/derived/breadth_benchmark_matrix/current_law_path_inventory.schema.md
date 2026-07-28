# Current-law path inventory schema

`current_law_path_inventory.v1.draft.json` inventories the official annual paths
required before the baseline year plus ten-year deterministic solver horizon can
be assembled.

The inventory may separately identify a ready shared federal topline spine.
That readiness does not change the required/false/null state of incomplete
lane, fund, general-fund, or endogenous-interest solver paths.

Required invariants:

- `record_family` is `current_law_path_inventory`.
- `pulse` is `108`.
- solver-input inventory, solver-input readiness rollup, and target-cost
  contract paths are explicit;
- required years are FY2025 through FY2035;
- interpolation is not allowed without an explicit model;
- every path row has `required: true`, `ready: false`, `value: null`, and at
  least one missing year;
- trust funds remain separate, and Medicare HI remains separate;
- official source custody requires raw bytes, metadata, retrieval date, byte
  count, and SHA-256 before values may be populated;
- only `current_law_path_inventory_published` may be true;
- no path values, solver run, target-cost, rate, public-rate-card, tax-proposal,
  savings, waste, fraud, department-cut, technology, or balanced-budget claim may
  be published from this inventory.
