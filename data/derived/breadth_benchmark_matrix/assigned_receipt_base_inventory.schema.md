# Assigned receipt-base inventory schema

`assigned_receipt_base_inventory.v1.draft.json` inventories candidate receipt
bases and the evidence required before any statutory or effective rate can be
calculated.

Required invariants:

- `record_family` is `assigned_receipt_base_inventory`.
- `pulse` is `105`.
- solver-input inventory, target-cost contract, balanced-rate gate, and
  rate-adjustment operating-model paths are explicit;
- required base fields include matched year, legal perimeter, economic
  perimeter, baseline amount, elasticity, avoidance/compliance, burden,
  distribution, tax interactions, current-law yield, and reform yield;
- every base row has all value fields null and `rate_ready: false`;
- rates do not need to sum to 100 percent, but resulting revenues must reconcile
  to funded requirements before publication;
- statutory rates, effective rates, solver readiness, public rate cards,
  tax proposals, savings, waste, fraud, technology, department-cut, target-cost,
  and balanced-budget claims remain blocked.
