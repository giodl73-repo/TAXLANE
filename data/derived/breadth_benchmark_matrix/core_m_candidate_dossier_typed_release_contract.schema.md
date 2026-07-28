# CORE-M candidate dossier and typed release contract schema

CORE-M binds a candidate objective profile to source-supported financing roles,
candidate-scoped gate dispositions, and permitted public outputs.

Every gate is exactly one of `required_ready`, `required_blocked`, or
`reviewed_not_applicable`. A blocked gate requires an owner and blocker. A
reviewed-not-applicable gate requires a rationale and carries no synthetic
evidence, value, pass result, or handoff.

Cost-only modernization cannot emit target-cost, savings, or assigned-rate
outputs. Revenue rates require a source-supported receipt or fee base. PAY-like
integrity effects remain non-additive, and NET-like effects remain endogenous.
