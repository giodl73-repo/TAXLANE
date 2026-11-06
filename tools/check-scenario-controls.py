"""Run the original native/WASM adapter controls against a selected Pages build."""
import argparse
import json
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('site', type=Path)
parser.add_argument('destination', type=Path, help='New evidence directory')
args = parser.parse_args()
site, destination = args.site.resolve(), args.destination.resolve()
subprocess.run([sys.executable, str(ROOT/'tools/make-scenario-controls.py'),
                str(site/'baseline.json'), str(destination)], check=True)
commands = []


def run(command, stdout, stderr, source=None):
    with stdout.open('xb') as output, stderr.open('xb') as errors:
        if source is None:
            result = subprocess.run(command, cwd=ROOT, stdout=output, stderr=errors)
        else:
            with source.open('rb') as input_file:
                result = subprocess.run(command, cwd=ROOT, stdin=input_file, stdout=output, stderr=errors)
    commands.append(dict(command=command, returncode=result.returncode,
                         stdout=stdout.name, stderr=stderr.name))
    (destination/'commands.json').write_text(json.dumps(commands, indent=2)+'\n', encoding='utf8')
    result.check_returncode()


for name in ['controls', 'target-width-controls']:
    run(['cargo', 'run', '--quiet', '--locked', '--release', '-p', 'taxlane-scenario',
         '--example', 'scenario-control'], destination/(name+'-native.jsonl'),
        destination/(name+'-native.stderr'), destination/(name+'.jsonl'))
    run(['node', str(ROOT/'tools/wasm-scenario-controls.mjs'),
         str(site/'pkg/taxlane_scenario.js'), str(site/'pkg/taxlane_scenario_bg.wasm'),
         str(destination/(name+'.jsonl')), str(destination/(name+'-wasm.jsonl'))],
        destination/(name+'-wasm.stdout'), destination/(name+'-wasm.stderr'))
run([sys.executable, str(ROOT/'tools/verify-scenario-controls.py'),
     str(destination/'controls.jsonl'), str(destination/'controls-native.jsonl'),
     str(destination/'controls-wasm.jsonl'), '--receipt', str(destination/'receipt.json')],
    destination/'verification.stdout', destination/'verification.stderr')
# Keep original target-width rejection messages intact, outside the portable matrix.
for target in ['native', 'wasm']:
    rows = [json.loads(line) for line in (destination/('target-width-controls-'+target+'.jsonl')).read_text(encoding='utf8').splitlines()]
    if len(rows) != 2 or any(set(row) != {'Err'} or not row['Err'] for row in rows):
        raise ValueError('Oversized usize inputs must both reject: '+target)
print((destination/'receipt.json').read_text(encoding='utf8'))
