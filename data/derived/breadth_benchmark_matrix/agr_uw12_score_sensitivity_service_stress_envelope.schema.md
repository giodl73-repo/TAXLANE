# AGR UW12 score-sensitivity and service-stress schema

The artifact publishes five scenarios recomputed by the
`taxlane-agr-insurance` feature crate: current-law null, normalized historical
context, normalized market stress, full service pause, and low-return no-cut.

Money is stored as million-USD micros, shares as parts per million, and returns
as basis points of retained premium. Each scenario contains complete inputs,
complete four-part outputs, and an explicit non-admission flag. The phase-in is
25, 50, and 100 percent beginning in years one, two, and three.

The $1 billion annual retained-premium base is a normalization unit, not a
current observed federal baseline. Scenario outputs may not be multiplied by
aggregate compensation, called a score, admitted to the fiscal solver, or used
to change rates.

AGR may move to named-trigger monitoring only when the null and service-pause
paths compute to zero and all scenario results are unadmitted.
