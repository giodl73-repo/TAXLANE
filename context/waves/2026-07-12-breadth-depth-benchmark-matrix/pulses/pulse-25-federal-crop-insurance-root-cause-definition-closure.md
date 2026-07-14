# Pulse 25: Federal Crop Insurance Root-Cause Definition Closure

## Result

Captured checksum-fixed official custody of USDA's FY2024 Agency Financial
Report through its GovInfo mirror after the canonical USDA download did not
complete. The FCIC section on printed pages 216-217 defines the two data-access
root causes that Pulse 24 could only identify by category and rounded amount.

Failure to access data or information means the information existed, was
obtained, and was used, but an administrative or calculation error made the
payment improper. Inability to access data or information is attributed
primarily to certification errors affecting the information used to establish
policy insurance amounts, premiums, and indemnities.

## Decision Gate

Pass for internal closure of `data-access outside-agency-control root-cause
definition`. FCIC now has three internally closed methodology fields and five
open fields: sample design, payment universe, estimation method, exclusion
rules, and recoverable-savings basis.

Fail for program scoring or public claims about fraud, identified debt,
collectibility, collections, recoveries, prevention, or savings. The AFR
defines error categories and describes corrective activity, but supplies no
amount-level recovery lineage or quantified causal effect.

## Exclusion

The USDA-wide Do Not Pay discussion immediately follows the FCIC section. Its
searches, dollar matches, and user counts are department-wide and are excluded
because the report provides no FCIC-specific attribution.

## Custody

- Source: `SRC-USDA-AFR-FY2024`
- Canonical USDA URL: `https://www.usda.gov/sites/default/files/documents/fy-2024-agency-financial-report.pdf`
- GovInfo custody URL: `https://www.govinfo.gov/content/pkg/CMR-A1-00191210/pdf/CMR-A1-00191210.pdf`
- Raw path: `data/raw/usda/SRC-USDA-AFR-FY2024/2026-07-13/fy-2024-agency-financial-report.pdf`
- Bytes: `15170759`
- SHA-256: `F573AC22DDCC64A1CE2DD9C13370EB1E02E83F2467F3A87146C3E3D521E8DE22`

## Integration Status

Complete. The custody and derived bridge now flow through the shared
methodology chain, agriculture and payment-integrity cards, breadth matrix,
scoreboard, WAVE, source ledger, manifest, and Rust validator.

## Next Bounded Action

Seek source evidence for the remaining five fields without treating
statistical improper-payment estimates, corrective actions, or department-wide
Do Not Pay activity as FCIC recoveries or savings.
