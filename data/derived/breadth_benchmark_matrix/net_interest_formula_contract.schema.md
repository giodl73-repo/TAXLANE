# Net-interest formula contract schema

`net_interest_formula_contract.v1.draft.json` is a draft contract for the
endogenous net-interest formula required by the deterministic solver.

Required invariants:

- `record_family` is `net_interest_formula_contract`.
- `pulse` is `104`.
- solver-input inventory, target-cost contract, solver-accounting gate, balance
  guardrail, and rate-adjustment operating-model paths are explicit;
- formula identities include primary balance, deficit, debt[t], net interest[t],
  and an iteration rule;
- every required input has `required: true`;
- baseline debt stock, baseline net interest, explicit other financing,
  new-borrowing timing, the matching-vintage CBO average-rate path, incremental
  interest-receipt treatment, the reduced-form feedback fixture, and the
  empirical marginal maturity/rollover convention are ready;
- the maturity input is ready only for marginal incremental feedback and
  retains explicit future-total-issuance and bucket-rate blockers for the
  full-stock mode;
- net interest is endogenous and cannot be cut directly;
- any primary-balance change must recompute subsequent debt and interest;
- maturity, rate, interest-receipt, and other-financing inputs must be explicit;
- the regression fixture must point to the CBO average-rate feedback artifact
  and remain explicitly reduced-form; the separate rollover fixture proves
  marginal maturity behavior without becoming a full-stock forecast;
- only the formula contract and reduced-form feedback fixture publication flags
  may be true;
- no net-interest path, solver, rate, target-cost, savings, waste, fraud,
  technology, department-cut, tax-proposal, public-rate-card, or
  balanced-budget claim may be published from this contract.
