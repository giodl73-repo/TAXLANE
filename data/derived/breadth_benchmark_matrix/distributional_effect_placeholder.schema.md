# Distributional-effect placeholder schema

`distributional_effect_placeholder.v1.draft.json` defines the fields required
before a distributional result can enter any solver, public rate card, tax
proposal, or balanced-budget claim.

Required invariants:

- `record_family` is `distributional_effect_placeholder`.
- `pulse` is `106`.
- solver-input inventory, assigned receipt-base inventory, target-cost contract,
  and balanced-rate gate paths are explicit;
- required fields include income group, tax unit, baseline income, tax burden,
  benefit/service value, employer incidence, household incidence, agency burden,
  tax/transfer interactions, macro feedback, equity floor, and public language;
- every placeholder row has all value fields null and `ready: false`;
- distribution, incidence, macro feedback, interaction scoring, and equity floor
  results remain unpublished;
- no solver, rate, public-card, tax-proposal, target-cost, savings, waste,
  fraud, technology, department-cut, or balanced-budget claim may be published
  from this placeholder.
