# Wave: Budget Concept Extraction

## Goal

Extract OMB budget-concept records for fund groups, general fund treatment,
dedicated collections, special funds, trust funds, revolving funds, and
appropriation guardrails.

## Thesis

TAXLANE should not promote fund-group rows from `unknown` interpretation to
public claims until budget-concept sources are captured and translated into
reviewable records. OMB Analytical Perspectives Chapter 13 is the next source
because it defines how federal funds, trust funds, special funds, and revolving
funds are treated.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|---|---|---|
| 01 | AP13 source capture | done | Captured OMB AP Chapter 13 PDF, checksum, and concept extraction workspace. |
| 02 | Fund concept first draft records | done | Extracted initial concept records for federal funds, general fund, special funds, trust funds, revolving funds, and trust-fund caveats. |
| 03 | Concept source review | done | Verified AP13 anchors and promoted first concept records to source-reviewed. |
| 04 | Fund rows budget interpretation | done | Applied reviewed concepts to the first fund-group slice without relabeling Federal Funds as General Fund. |
| 05 | Income-tax general-fund note | done | Summarized what reviewed AP13 concepts support about ordinary income-tax general-fund treatment. |

## Success Criteria

- AP13 is captured under `data/raw/` with metadata and checksum.
- Concept records are separate from numeric fund-group rows.
- Concept records preserve source anchors and public-use caveats.
- Legal dedication and appropriation status are not inferred without support.
- Validation commands pass.

## Non-Goals

- Do not build taxpayer receipts in this wave.
- Do not claim a specific taxpayer payment went to a fund.
- Do not use AP13 concept text to override statute-specific requirements.
- Do not extract the entire AP13 chapter in one pulse.

## Validation

Run:

```powershell
git diff --check
```

For JSONL concept pulses, parse every record as JSON.
