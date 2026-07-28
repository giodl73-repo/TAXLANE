# Fifteen-lane track stage matrix schema

This matrix maps each canonical row in `lane_full_coverage_matrix` to one unique
three-letter track prefix and measures its highest reviewed lane-track stage
against the portfolio target of stage E.

The canonical prefixes are `TRN`, `HLT`, `EDU`, `OAS`, `ISF`, `VET`, `AGR`,
`DEF`, `DIS`, `JUS`, `SEE`, `INT`, `PAY`, `REV`, and `NET`. Overlay and
endogenous lanes retain their special accounting treatment; a track prefix does
not convert them into additive spending lanes.

Stage E may close without a solver run only through the canonical E contract:
the D predecessor is complete, input classification and selection are complete,
selection resolves to no candidate, run-dependent packages are `not_required`,
role review is complete, and all numeric and public outputs remain null. Such
closure does not prove numeric completion, solver readiness, rates, savings, or
full coverage.
