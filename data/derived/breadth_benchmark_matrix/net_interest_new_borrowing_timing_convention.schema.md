# NET new-borrowing timing convention schema

The convention applies only to incremental borrowing relative to an admitted
current-law baseline. Positive primary-balance improvement reduces financing.
The midpoint rail is central, with early- and late-year timing boundaries.

Current-year interest financing is solved in closed form as `k * financing /
(1-k)`. Arithmetic remains in millions with at least six decimal places and is
rounded only for display. A zero primary delta must return zero interest and
debt deltas on every rail.

This closes the timing-rule input only. It does not provide the borrowing rate
path, allocate debt to maturity buckets, roll debt into future years, admit the
mechanical example, or authorize a solver, savings, or rate result.
