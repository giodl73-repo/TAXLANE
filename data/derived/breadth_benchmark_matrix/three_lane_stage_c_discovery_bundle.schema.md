# Three-lane stage-C discovery bundle schema

This contract closes `ISF-C`, `VET-C`, and `AGR-C` only as bounded candidate
screens. It requires each lane to pin `CORE-I` through `CORE-L`, reconcile four
work packages, record candidate evidence states, complete role review, and keep
candidate-dependent outputs null when no reform is admitted.

Required closure fields are `track_wave_id`, `package_reconciliation`,
`candidate_screen`, `closure_decision`, `blocked_outputs`, and
`claim_booleans`. Every package status must be `complete` or `not_required`.
`closure_decision.output_admission` must remain false for a bounded screen.

The aggregate bundle requires exactly three lane evidence rows, all three C
completion claims, zero newly admitted reforms, zero D-stage permissions, and
zero solver runs. Discovery completion never substitutes for a legislative
instrument, signed annual score, applicable-floor result, target cost, rate, or
savings estimate.
