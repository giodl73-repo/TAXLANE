# Contribution Alignment

## Purpose

This draft family records whether major TAXLANE lanes are aligned to what people
pay in, who benefits, and which denominator a future per-person receipt must use.

It is a design and question-routing layer. It is not a legal allocation claim and
does not calculate individual tax liability.

## Artifacts

| Artifact | Role |
|---|---|
| `contribution_alignment.fy2025.v1.draft.jsonl` | Draft lane alignment rows. |
| `medicare_source_boundary.fy2025.draft.jsonl` | OMB Table 3.2 vs Table 8.5 Medicare scope boundary check. |
| `medicare_part_financing.cy2025.cms-trustees-2026.draft.jsonl` | CMS Trustees CY2025 Medicare HI, Part B, and Part D financing split. |
| `contribution_alignment.schema.md` | Field contract for alignment rows. |

## Validation

```powershell
git diff --check
```
