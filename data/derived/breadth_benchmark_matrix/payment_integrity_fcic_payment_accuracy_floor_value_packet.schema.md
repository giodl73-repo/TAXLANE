# Payment Integrity FCIC Payment Accuracy Floor Value Packet Schema

Draft schema for
`payment_integrity_fcic_payment_accuracy_floor_value_packet.v1.draft.json`.

Required fields:

- identity fields tying the packet to the payment-integrity non-additive
  overlay, FCIC program component, quality/safety floor class, payment-integrity
  floor-definition packet, FCIC bridge, and Wave D readiness rollup;
- a threshold rationale with selected measure, rule, type, value, unit, source,
  and review status;
- baseline values with reporting period, measurement window, primary payment
  accuracy baseline, supporting improper-payment composition, source IDs, and
  source path;
- null policy values, null stress values, null pass/fail evidence, null
  downstream outputs, and false downstream claim booleans;
- a public warning preserving that the packet is not fraud, waste, savings,
  solver input, rate calculation, or a balanced-budget claim.
