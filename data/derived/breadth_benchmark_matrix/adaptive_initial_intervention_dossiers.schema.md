# Adaptive initial intervention dossiers schema

This record contains the first pain-aligned VET, TRN, and ISF intervention
dossiers. Each dossier names the pain, candidate family, bounded analytical
candidate, owner, funding type, diagnostic measure, reviewed official evidence,
transferability boundary, existing-candidate reconciliation, all ten gate
dispositions, scoring inputs, and allocation result.

`candidate_planning_inputs` holds candidate-specific baselines, historical unit
costs, current-law comparators, and explicit nulls needed for the next score.
Every input states its scope and exclusions so a historical per-mile price,
service baseline, or payment-accuracy measure cannot silently become a national
cost, causal effect, or savings estimate.

`official_evidence[].custody_status` distinguishes a reviewed official web
source from a raw, checksummed repository source. A historical outcome or cost
may support a partial gate without becoming a current-law score. Candidate
selection means the instrument is specific enough to investigate; it is not a
policy recommendation or funding decision.

An existing bill may not substitute for an unrelated intervention merely
because it belongs to the same lane. Ready diagnostic evidence does not make an
intervention ready. Scores and allocations remain null until every gate passes.
