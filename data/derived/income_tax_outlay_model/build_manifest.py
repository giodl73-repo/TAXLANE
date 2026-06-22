"""Build an artifact manifest for the income-tax outlay model."""

from __future__ import annotations

import argparse
import csv
import hashlib
import json
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[3]
MODEL_DIR = ROOT / "data" / "derived" / "income_tax_outlay_model"
MANIFEST_PATH = MODEL_DIR / "MANIFEST.md"

ARTIFACTS: list[dict[str, Any]] = [
    {
        "path": "data/derived/income_tax_outlay_model/income_tax_outlay_model.omb-fy2027.2026-06-21.draft.jsonl",
        "role": "Canonical annual model rows",
        "grain": "fiscal year by broad category",
        "kind": "jsonl",
        "canonical": "yes",
    },
    {
        "path": "data/derived/income_tax_outlay_model/income_tax_outlay_model.omb-fy2027.2026-06-21.decade-summary.jsonl",
        "role": "Canonical decade summary rows",
        "grain": "decade by broad category",
        "kind": "jsonl",
        "canonical": "yes",
    },
    {
        "path": "data/derived/income_tax_outlay_model/income_tax_outlay_model.omb-fy2027.2026-06-21.annual-wide.csv",
        "role": "Chart-ready annual wide view",
        "grain": "fiscal year",
        "kind": "csv",
        "canonical": "no",
    },
    {
        "path": "data/derived/income_tax_outlay_model/income_tax_outlay_model.omb-fy2027.2026-06-21.decade-wide.csv",
        "role": "Chart-ready decade wide view",
        "grain": "decade",
        "kind": "csv",
        "canonical": "no",
    },
    {
        "path": "data/derived/income_tax_outlay_model/README.md",
        "role": "Model method and schema note",
        "grain": "documentation",
        "kind": "markdown",
        "canonical": "supporting",
    },
    {
        "path": "data/derived/income_tax_outlay_model/source-profile.md",
        "role": "Source coverage and reconciliation sample",
        "grain": "documentation",
        "kind": "markdown",
        "canonical": "supporting",
    },
    {
        "path": "data/derived/income_tax_outlay_model/reconciliation-review.md",
        "role": "Generated-row reconciliation review",
        "grain": "documentation",
        "kind": "markdown",
        "canonical": "supporting",
    },
    {
        "path": "data/derived/income_tax_outlay_model/decade-summary.md",
        "role": "Human-readable decade summary",
        "grain": "documentation",
        "kind": "markdown",
        "canonical": "supporting",
    },
    {
        "path": "docs/reading/modeled-income-tax-outlays.md",
        "role": "Reader-facing packet",
        "grain": "documentation",
        "kind": "markdown",
        "canonical": "supporting",
    },
    {
        "path": "docs/charts/README.md",
        "role": "Chart catalog",
        "grain": "documentation",
        "kind": "markdown",
        "canonical": "supporting",
    },
    {
        "path": "docs/charts/income-tax-outlay-model/annual-stacked-area.vl.json",
        "role": "Annual allocation chart spec",
        "grain": "visualization spec",
        "kind": "json",
        "canonical": "view",
    },
    {
        "path": "docs/charts/income-tax-outlay-model/decade-stacked-bar.vl.json",
        "role": "Decade allocation chart spec",
        "grain": "visualization spec",
        "kind": "json",
        "canonical": "view",
    },
    {
        "path": "docs/charts/income-tax-outlay-model/annual-financing-context-lines.vl.json",
        "role": "Annual financing context chart spec",
        "grain": "visualization spec",
        "kind": "json",
        "canonical": "view",
    },
    {
        "path": "docs/charts/income-tax-outlay-model/decade-financing-context-lines.vl.json",
        "role": "Decade financing context chart spec",
        "grain": "visualization spec",
        "kind": "json",
        "canonical": "view",
    },
]


def sha256(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as f:
        for chunk in iter(lambda: f.read(1024 * 1024), b""):
            h.update(chunk)
    return h.hexdigest()


def count_rows(path: Path, kind: str) -> int | str:
    if kind == "jsonl":
        lines = path.read_text(encoding="utf-8").splitlines()
        for line in lines:
            json.loads(line)
        return len(lines)
    if kind == "csv":
        with path.open(newline="", encoding="utf-8") as f:
            return sum(1 for _ in csv.DictReader(f))
    return "n/a"


def build_manifest() -> tuple[str, list[str]]:
    errors: list[str] = []
    rows: list[dict[str, Any]] = []
    for artifact in ARTIFACTS:
        path = ROOT / artifact["path"]
        if not path.exists():
            errors.append(f"missing artifact: {artifact['path']}")
            continue
        rows.append(
            {
                **artifact,
                "rows": count_rows(path, artifact["kind"]),
                "sha256": sha256(path),
            }
        )

    lines = [
        "# Income-Tax Outlay Model Artifact Manifest",
        "",
        "## Purpose",
        "",
        "This manifest records the artifact chain for the modeled allocation of",
        "ordinary individual income-tax receipts by broad OMB outlay share.",
        "",
        "The annual and decade JSONL files are canonical model outputs. CSV files,",
        "Markdown notes, and chart specs are derived or supporting views.",
        "",
        "## Model",
        "",
        "- Model ID: `individual-income-tax-proportional-outlays-v1`",
        "- Coverage: fiscal years 1940-2025 for annual actual-year rows",
        "- Projection treatment: FY2026-FY2031 excluded",
        "- Legal status: modeled allocation, not legal dedication",
        "",
        "## Artifacts",
        "",
        "| Path | Role | Grain | Rows | Canonical | SHA-256 |",
        "|---|---|---|---:|---|---|",
    ]
    for row in rows:
        lines.append(
            f"| `{row['path']}` | {row['role']} | {row['grain']} | "
            f"{row['rows']} | {row['canonical']} | `{row['sha256']}` |"
        )
    lines.extend(
        [
            "",
            "## Regeneration Order",
            "",
            "1. `build_income_tax_outlay_model.py`",
            "2. `build_decade_summary.py`",
            "3. `export_chart_views.py`",
            "4. `build_manifest.py`",
            "",
            "Run validation after regeneration:",
            "",
            "```powershell",
            "python data/derived/income_tax_outlay_model/build_income_tax_outlay_model.py --check",
            "python data/derived/income_tax_outlay_model/build_decade_summary.py --check",
            "python data/derived/income_tax_outlay_model/export_chart_views.py --check",
            "python data/derived/income_tax_outlay_model/build_manifest.py --check",
            "git diff --check",
            "```",
            "",
        ]
    )
    return "\n".join(lines), errors


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()

    text, errors = build_manifest()
    if errors:
        for error in errors:
            print(error)
        return 1
    if not args.check:
        MANIFEST_PATH.write_text(text, encoding="utf-8", newline="\n")
    print("validated income-tax outlay artifact manifest")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
