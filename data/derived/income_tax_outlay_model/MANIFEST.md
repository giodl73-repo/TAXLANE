# Income-Tax Outlay Model Artifact Manifest

## Purpose

This manifest records the artifact chain for the modeled allocation of
ordinary individual income-tax receipts by broad OMB outlay share.

The annual and decade JSONL files are canonical model outputs. CSV files,
Markdown notes, and chart specs are derived or supporting views.

## Model

- Model ID: `individual-income-tax-proportional-outlays-v1`
- Coverage: fiscal years 1940-2025 for annual actual-year rows
- Projection treatment: FY2026-FY2031 excluded
- Legal status: modeled allocation, not legal dedication

## Artifacts

| Path | Role | Grain | Rows | Canonical | SHA-256 |
|---|---|---|---:|---|---|
| `data/derived/income_tax_outlay_model/income_tax_outlay_model.omb-fy2027.2026-06-21.draft.jsonl` | Canonical annual model rows | fiscal year by broad category | 516 | yes | `01c0fec63836f3652fdd83b03a608159ad58f7614fc214ae6691421990f3a85e` |
| `data/derived/income_tax_outlay_model/income_tax_outlay_model.omb-fy2027.2026-06-21.decade-summary.jsonl` | Canonical decade summary rows | decade by broad category | 54 | yes | `088cb5d55cc6a37ab52dd543d8a92bf09908cda265e9c9f93324b840eb652fb3` |
| `data/derived/income_tax_outlay_model/income_tax_outlay_model.omb-fy2027.2026-06-21.annual-wide.csv` | Chart-ready annual wide view | fiscal year | 86 | no | `a385e4215a20c16f0344cc8e17982c9e4c78933caac93f274ea5b6080027d81a` |
| `data/derived/income_tax_outlay_model/income_tax_outlay_model.omb-fy2027.2026-06-21.decade-wide.csv` | Chart-ready decade wide view | decade | 9 | no | `5e243eb2912b7aaed26e22016fd1907bf4cee25d60b3855f1c2627e7f4b9a2aa` |
| `data/derived/income_tax_outlay_model/README.md` | Model method and schema note | documentation | n/a | supporting | `60ff4837eba1ec93335c6adb76decb80860a0582866743da9da39dbf5e26cfc2` |
| `data/derived/income_tax_outlay_model/source-profile.md` | Source coverage and reconciliation sample | documentation | n/a | supporting | `03ea7eee098ee6e1f3cc4a83cd89adec009f9cbb96d050f2e7118e06dcf2e70f` |
| `data/derived/income_tax_outlay_model/reconciliation-review.md` | Generated-row reconciliation review | documentation | n/a | supporting | `ab200dfc8c467700964cb23d51875411603a73f9d1ea62e1beea9073449a9c8d` |
| `data/derived/income_tax_outlay_model/decade-summary.md` | Human-readable decade summary | documentation | n/a | supporting | `47d02425c28cd8f8fd0c236b46e0c997d54a0824ddca1cb125ca0a39bcfd915f` |
| `docs/reading/modeled-income-tax-outlays.md` | Reader-facing packet | documentation | n/a | supporting | `b7cdffb161572ff8e7c862747ad4172ef7a59deaa780373c6331f474d68515f7` |
| `docs/charts/README.md` | Chart catalog | documentation | n/a | supporting | `243edecd34c1f7feae2ab4269b8979a5dcd8935c10926d4a68c2f8100c437a9b` |
| `docs/charts/income-tax-outlay-model/annual-stacked-area.vl.json` | Annual allocation chart spec | visualization spec | n/a | view | `dab4f7d15be91c7357c8595d14e8d58336048747bc7a3825cee756072dc05b54` |
| `docs/charts/income-tax-outlay-model/decade-stacked-bar.vl.json` | Decade allocation chart spec | visualization spec | n/a | view | `4e1bbc8b41ce38147477d7494e4a5be29b62985111e3c9be62de370501216969` |
| `docs/charts/income-tax-outlay-model/annual-financing-context-lines.vl.json` | Annual financing context chart spec | visualization spec | n/a | view | `509c0061b6fc1208ed2cb43f13493662647590d2a72cb8b3fe4b12ed3589a722` |
| `docs/charts/income-tax-outlay-model/decade-financing-context-lines.vl.json` | Decade financing context chart spec | visualization spec | n/a | view | `d211045006ad27f05554b1db88c858d1a4ace725d721a34f91565c5d34fbe450` |
| `data/derived/income_tax_outlay_model/validate_all.py` | One-command validation runner | script | n/a | supporting | `5ea7b69118b3d44c6b05879945c35899911e9975c921617acbc065efa2ca1ac9` |

## Regeneration Order

1. `build_income_tax_outlay_model.py`
2. `build_decade_summary.py`
3. `export_chart_views.py`
4. `build_manifest.py`

Run validation after regeneration:

```powershell
python data/derived/income_tax_outlay_model/validate_all.py
```

The validation runner expands to:

```powershell
python data/derived/income_tax_outlay_model/build_income_tax_outlay_model.py --check
python data/derived/income_tax_outlay_model/build_decade_summary.py --check
python data/derived/income_tax_outlay_model/export_chart_views.py --check
python data/derived/income_tax_outlay_model/build_manifest.py --check
git diff --check
```
