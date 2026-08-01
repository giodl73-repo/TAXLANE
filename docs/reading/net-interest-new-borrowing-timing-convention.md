# NET new-borrowing timing convention

Machine record:
`data/derived/breadth_benchmark_matrix/net_interest_new_borrowing_timing_convention.v1.draft.json`.

NET now has an explicit rule for when an admitted primary-balance change affects
current-year borrowing interest. The central rail assumes cash changes occur
evenly through the year, giving one-half-year exposure. Early-year and late-
year rails bound the timing sensitivity.

The formula solves for the interest needed to finance the current-year interest
itself, retains unrounded million-dollar arithmetic, and produces zero debt and
interest change for a zero primary delta. A $100 billion example at 3.404% is a
mechanical test only: the midpoint current-year interest response is
-$1.731470 billion, bounded by $0 and -$3.523955 billion.

This makes the new-borrowing timing input ready, raising NET formula readiness
to 4/8 and baseline completion to 5/9. The borrowing rate path, maturity
rollover, and future-year feedback remain blocked, so there is no admitted
interest saving or rate change.
