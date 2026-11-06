"""Build the hypothetical budget explorer; preserve canonical research artifacts."""
import argparse
import json
import os
import subprocess
from pathlib import Path

from input_custody import destination, digest, overlaps, plain_path, strict_json
from rust_build import configure, finish
from source_artifacts import package
from source_custody import encoded, snapshot

ROOT = Path(__file__).resolve().parents[1]
LEDGER = "data/derived/program_lane_rate_model/program_lane_rate_model.fy2025.omb-fy2027-v1.draft.jsonl"


def run(*args):
    subprocess.run(args, cwd=ROOT, check=True)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", type=Path, default=Path("dist/TAXLANE"))
    parser.add_argument("--source-output", type=Path, default=Path("dist/source-artifacts"))
    parser.add_argument("--inputs", type=Path, required=True, help="New or verified owned Rust source input directory")
    parser.add_argument("--cargo-home", type=Path, required=True, help="New or verified owned offline Cargo home")
    parser.add_argument("--target", type=Path, default=Path(os.environ.get('CARGO_TARGET_DIR', ROOT/'target')))
    args = parser.parse_args()
    output = destination(args.output)
    source_output = destination(args.source_output)
    allowed = ROOT / "dist"
    if output == allowed or allowed not in output.parents:
        raise ValueError("Output must be a directory inside dist")
    inputs, home, target = [plain_path(p) for p in (args.inputs, args.cargo_home, args.target)]
    if any(overlaps(a, b) for a in (output, source_output) for b in (inputs, home, target)) or overlaps(output, source_output):
        raise ValueError('Output/source artifacts overlap inputs, home, target or each other')
    version = subprocess.check_output(["wasm-bindgen", "--version"], text=True).strip()
    if version != "wasm-bindgen 0.2.127":
        raise ValueError(f"Expected wasm-bindgen 0.2.127, found {version}")
    source_receipt = package(ROOT, source_output)
    files, source_manifest = snapshot(ROOT)
    if source_receipt['commit'] != source_manifest['commit'] or source_receipt['tree'] != source_manifest['tree']:
        raise ValueError('Source changed while packaging')
    runtime_relative = 'runtime/'+source_manifest['commit']
    context = configure(inputs, home, target)
    dependency_notices = files['DEPENDENCY-NOTICES.txt'][0]
    if dependency_notices != context['verified']['notices']:
        raise ValueError('Published complete dependency notices differ')
    runtime_notice = strict_json(files['notices/RUNTIME-NOTICES.json'][0])
    if runtime_notice['toolchain'] != '1.95.0' or runtime_notice['file'] != 'rust-1.95.0-COPYRIGHT-library.html':
        raise ValueError('Unexpected Rust runtime notices')
    runtime_notices = files['notices/'+runtime_notice['file']][0]
    if len(runtime_notices) != runtime_notice['bytes'] or digest(runtime_notices) != runtime_notice['sha256']:
        raise ValueError('Complete Rust runtime notices differ')
    rows = [json.loads(line) for line in files[LEDGER][0].decode('utf-8').splitlines() if line.strip()]
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
    output.mkdir(parents=True, exist_ok=False)
    runtime = output/runtime_relative
    runtime.mkdir(parents=True)
    path = runtime / "baseline.json"
    baseline_raw = json.dumps(baseline, ensure_ascii=False, separators=(",", ":"))
    path.write_bytes(baseline_raw.encode('utf-8'))
    run("cargo", "run", "--locked", "--quiet", "-p", "taxlane-scenario", "--bin", "validate-baseline", "--", str(path))
    run("cargo", "build", "--locked", "--release", "--target", "wasm32-unknown-unknown", "-p", "taxlane-scenario", "--lib", "--features", "wasm")
    metadata = json.loads(subprocess.check_output(["cargo", "metadata", "--no-deps", "--format-version", "1"], cwd=ROOT))
    wasm = Path(metadata["target_directory"]) / "wasm32-unknown-unknown/release/taxlane_scenario.wasm"
    run("wasm-bindgen", str(wasm), "--target", "web", "--out-dir", str(runtime / "pkg"))
    for name in ("style.css", "app.js", "worker.js", "lane-evidence.js"):
        (runtime/name).write_bytes(files['web/'+name][0])
    run('cargo', 'build', '--locked', '--release', '-p', 'taxlane-scenario', '--example', 'scenario-control')
    native = target/'release/examples'/('scenario-control.exe' if os.name == 'nt' else 'scenario-control')
    scenarios = [dict(lane_change_bps={}, receipts_change_bps=0, transit_share_bps=0, transit_area=0),
                 dict(lane_change_bps={'health':-1000}, receipts_change_bps=0, transit_share_bps=0, transit_area=0),
                 dict(lane_change_bps={'transportation':1000}, receipts_change_bps=0, transit_share_bps=1000, transit_area=2)]
    controls = [dict(baseline=baseline_raw, scenario=json.dumps(s, separators=(',', ':'))) for s in scenarios]
    native_results = subprocess.check_output([str(native)], cwd=ROOT,
        input=b''.join(encoded(control) for control in controls)).splitlines()
    if len(native_results) != len(controls):
        raise ValueError('Incomplete native comparison examples')
    native_frames = []
    for control, result in zip(controls, native_results):
        result = strict_json(result)
        if set(result) != {'Ok'} or not isinstance(result['Ok'], str):
            raise ValueError('Original native example rejected')
        native_frames.append(encoded(dict(inputs=control, native=result)))
    (output/'native-examples.ndjson').write_bytes(b''.join(native_frames))
    source_files = [dict(path=p.name, bytes=p.stat().st_size, sha256=digest(p.read_bytes()))
                    for p in sorted(source_output.iterdir()) if p.is_file()]
    release_tag = 'source-'+source_manifest['commit']
    source_base = 'https://github.com/giodl73-repo/TAXLANE/releases/download/'+release_tag+'/'
    source_records = {row['path']:row for row in source_files}
    substitutions = {'@@RUNTIME@@':runtime_relative, '@@COMMIT@@':source_manifest['commit'],
        '@@TREE@@':source_manifest['tree'], '@@SOURCE_BASE@@':source_base,
        '@@SOURCE_SHA@@':source_records['source.tar.xz']['sha256'],
        '@@SOURCE_BYTES@@':str(source_records['source.tar.xz']['bytes']),
        '@@SOURCE_FILES@@':str(source_receipt['source_files']),
        '@@SOURCE_FILE_BYTES@@':str(source_receipt['source_bytes']),
        '@@RECEIVER_SHA@@':source_records['receiver.zip']['sha256'],
        '@@RECEIVER_BYTES@@':str(source_records['receiver.zip']['bytes'])}
    for name in ('index.html', 'build.html'):
        template = files['web/'+name][0].decode('utf-8')
        for old, value in substitutions.items():
            template = template.replace(old, value)
        if '@@' in template:
            raise ValueError('Unresolved delivery template')
        (output/name).write_bytes(template.encode('utf-8'))
    (output/'LICENSE').write_bytes(files['LICENSE'][0])
    (output/'COMPONENT-NOTICES.txt').write_bytes(b'\n\n'.join(files['notices/'+name+'-COMPONENTS-LICENSE.txt'][0]
        for name in ['LATTICE', 'WITNESS', 'TERRAIN']))
    (output/'DEPENDENCY-NOTICES.txt').write_bytes(dependency_notices)
    (output/'RUST-RUNTIME-NOTICES.html').write_bytes(runtime_notices)
    def capture(*args):
        return subprocess.check_output(args, cwd=ROOT, text=True, encoding="utf-8").strip()
    provenance = {
        "commit": source_manifest['commit'],
        "tree": source_manifest['tree'],
        "dirty": False,
        "rust": capture("rustc", "--version"),
        "bindings": version,
        "source": "https://github.com/giodl73-repo/TAXLANE",
    }
    (output / "provenance.json").write_text(json.dumps(provenance, indent=2) + "\n", encoding="utf-8")
    (output / ".nojekyll").touch()
    policy = finish(context)
    if snapshot(ROOT) != (files, source_manifest):
        raise ValueError('Complete source changed during compilation')
    published = [dict(path=p.relative_to(output).as_posix(), bytes=p.stat().st_size, sha256=digest(p.read_bytes()))
                 for p in sorted(output.rglob('*')) if p.is_file()]
    build_manifest = dict(schema='taxlane.pages-build.v1', commit=source_manifest['commit'], tree=source_manifest['tree'],
        runtime=runtime_relative, source_archive_sha256=source_records['source.tar.xz']['sha256'],
        source_artifacts=source_receipt, source_release=dict(tag=release_tag, base_url=source_base, files=source_files),
        files=published, original_inputs=policy, toolchain='1.95.0', wasm_bindgen='0.2.127',
        manifest_scope='Every other runtime regular file; manifest excludes itself. Complete source artifacts measured separately.',
        runtime_budget_bytes=5_000_000)
    (output/'BUILD-MANIFEST.json').write_bytes(encoded(build_manifest))
    size = sum(path.stat().st_size for path in output.rglob("*") if path.is_file())
    if size >= 5_000_000:
        raise ValueError("Budget explorer exceeds its 5 MB delivery budget")
    print(f"Pages explorer: {size:,} bytes at {output}")


if __name__ == "__main__":
    main()
