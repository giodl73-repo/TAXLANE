# Accountability Response Intake Evidence Rules Review

## Scope

Reviewed response intake evidence/class consistency before future importer work updates response logs.

## Findings

- Complete and partial evidence response intake rows now require at least one `evidence_received` item.
- Process-only and no-evidence response intake rows remain blocked from listing received evidence.
- The intake schema now states both evidence-required and evidence-empty class rules.
- Public-claim and no-finding guardrails remain unchanged.

## Decision

Accepted as tightened response intake evidence rules. It prevents an empty partial response from being treated as an evidence response.
