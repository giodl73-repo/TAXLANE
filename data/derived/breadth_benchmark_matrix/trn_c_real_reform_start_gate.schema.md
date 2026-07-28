# TRN-C Real Reform Start Gate Schema

TRN-C may start only after TRN-B closes. The evolved gate now records completed
candidate discovery, a conditional cost-only scenario, CORE-I extraction, and
TRN-C closure while preserving downstream claim boundaries.

Required checks:

- all five work packages are complete;
- H.R. 2247 and its $18 million conditional CBO outlay score are admitted;
- enactment, financing, savings, lower target cost, rate, and public-card fields
  remain null/false; and
- `trn_c_done` and `trn_d_may_start` are true.
