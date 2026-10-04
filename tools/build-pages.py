"""Build the hypothetical budget explorer; preserve canonical research artifacts."""
import argparse
import json
import shutil
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
LEDGER = ROOT / "data/derived/program_lane_rate_model/program_lane_rate_model.fy2025.omb-fy2027-v1.draft.jsonl"


def run(*args):
    subprocess.run(args, cwd=ROOT, check=True)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", type=Path, default=Path("dist/TAXLANE"))
    args = parser.parse_args()
    output = args.output.resolve()
    allowed = ROOT / "dist"
    if output == allowed or allowed not in output.parents:
        raise ValueError("Output must be a directory inside dist")
    version = subprocess.check_output(["wasm-bindgen", "--version"], text=True).strip()
    if version != "wasm-bindgen 0.2.127":
        raise ValueError(f"Expected wasm-bindgen 0.2.127, found {version}")
    rows = [json.loads(line) for line in LEDGER.read_text(encoding="utf-8").splitlines() if line.strip()]
    if len(rows) != 17 or any(row["fiscal_year"] != 2025 for row in rows):
        raise ValueError("Expected 17 FY2025 budget rows")
    contexts = [row["solvency_context"] for row in rows]
    if any(context != contexts[0] for context in contexts):
        raise ValueError("Budget rows have inconsistent accounting context")
    context = contexts[0]
    def integer(value):
        if int(value) != value:
            raise ValueError("Baseline amounts must be integer millions")
        return int(value)
    baseline = {
        "version": 1, "fiscal_year": 2025,
        "receipts_musd": integer(context["total_receipts_musd"]),
        "outlays_musd": integer(context["total_outlays_musd"]),
        "lanes": [{"id": row["lane_id"], "label": row["public_label"],
                   "amount_musd": integer(row["current_cost_amount_musd"]),
                   "adjustable": row["current_cost_amount_musd"] >= 0 and row["lane_id"] != "net-interest",
                   "sources": row["source_ids"]} for row in rows],
    }
    output.mkdir(parents=True, exist_ok=True)
    path = output / "baseline.json"
    path.write_text(json.dumps(baseline, ensure_ascii=False, separators=(",", ":")), encoding="utf-8")
    run("cargo", "run", "--locked", "--quiet", "-p", "taxlane-scenario", "--bin", "validate-baseline", "--", str(path))
    run("cargo", "build", "--locked", "--release", "--target", "wasm32-unknown-unknown", "-p", "taxlane-scenario", "--lib", "--features", "wasm")
    metadata = json.loads(subprocess.check_output(["cargo", "metadata", "--no-deps", "--format-version", "1"], cwd=ROOT))
    wasm = Path(metadata["target_directory"]) / "wasm32-unknown-unknown/release/taxlane_scenario.wasm"
    run("wasm-bindgen", str(wasm), "--target", "web", "--out-dir", str(output / "pkg"))
    for name in ("index.html", "style.css", "app.js", "worker.js"):
        shutil.copy2(ROOT / "web" / name, output / name)
    (output / ".nojekyll").touch()
    size = sum(path.stat().st_size for path in output.rglob("*") if path.is_file())
    if size >= 5_000_000:
        raise ValueError("Budget explorer exceeds its 5 MB delivery budget")
    print(f"Pages explorer: {size:,} bytes at {output}")


if __name__ == "__main__":
    main()
