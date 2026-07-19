# Wave 5 fiscal-control overlay depth packets schema

Draft schema for `wave5_fiscal_control_overlay_depth_packets.v1.draft.json`.

Required top-level fields:

- `record_id`, `record_family`, `version`, `status`, and `pulse`.
- Links to the lane work-order plan, lane-depth tracker, target-cost contract,
  assigned receipt-base inventory, balanced-rate readiness gate,
  payment-integrity depth card, and net-interest formula contract.
- `wave` with exactly the overlay ids `revenue-solvency`,
  `payment-integrity`, and `net-interest`.
- `overlay_packets`, exactly one packet for each overlay.
- `integration_review`.
- aggregate `claim_booleans`.
- `plain_english_status`.

Each overlay packet requires:

- `overlay_id`, `public_label`, `what_it_does`, `who_is_affected`,
  `overspending_underfunding_boundary`, and `technology_transition_boundary`.
- `what_taxpayers_pay_now` with `value: null` until public-use values are
  fully modeled.
- overlay-specific boundaries and blockers.
- `claim_booleans`, with only `overlay_depth_packet_published: true`; all
  rate, target-cost, savings, waste, fraud, department-cut, technology-savings,
  solver, tax-proposal, public-rate-card, and balanced-budget booleans must be
  false.

Required invariants:

- Revenue-solvency and payment integrity are non-additive overlays.
- Net interest is endogenous and cannot be cut directly.
- Missing values remain `null`.
- Blocked gates remain `false`.
- Statutory rates cannot be published before matched receipt bases, behavior,
  incidence, distribution, and administration are modeled.
- Payment-integrity savings require causal prevention or same-cohort collection
  lineage.
- Improper-payment estimates cannot imply fraud.
- Technology changes are transition paths, not automatic savings.
- No solver, rate, tax, target-cost, savings, waste/fraud, department-cut,
  technology-savings, or balanced-budget claim may be opened by this packet.
