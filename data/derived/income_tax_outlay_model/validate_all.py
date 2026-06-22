"""Run all income-tax outlay model validation checks."""

from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[3]
PYTHON = sys.executable

MODEL_CHECKS = [
    [
        PYTHON,
        "data/derived/income_tax_outlay_model/build_income_tax_outlay_model.py",
        "--check",
    ],
    [
        PYTHON,
        "data/derived/income_tax_outlay_model/build_decade_summary.py",
        "--check",
    ],
    [
        PYTHON,
        "data/derived/income_tax_outlay_model/export_chart_views.py",
        "--check",
    ],
    [
        PYTHON,
        "data/derived/income_tax_outlay_model/build_manifest.py",
        "--check",
    ],
]

CHART_SPECS = [
    "docs/charts/income-tax-outlay-model/annual-stacked-area.vl.json",
    "docs/charts/income-tax-outlay-model/decade-stacked-bar.vl.json",
    "docs/charts/income-tax-outlay-model/annual-financing-context-lines.vl.json",
    "docs/charts/income-tax-outlay-model/decade-financing-context-lines.vl.json",
]


def run_command(command: list[str]) -> int:
    print("+ " + " ".join(command), flush=True)
    completed = subprocess.run(command, cwd=ROOT)
    return completed.returncode


def validate_chart_specs() -> int:
    for spec in CHART_SPECS:
        print(f"+ parse {spec}", flush=True)
        with (ROOT / spec).open(encoding="utf-8") as f:
            json.load(f)
    print(f"validated {len(CHART_SPECS)} chart specs")
    return 0


def main() -> int:
    for command in MODEL_CHECKS:
        code = run_command(command)
        if code:
            return code
    return validate_chart_specs()


if __name__ == "__main__":
    raise SystemExit(main())
