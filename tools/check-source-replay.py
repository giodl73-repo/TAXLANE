"""Receive full source without Git, then independently rebuild source and runtime.

Run only after trusting the source and builder. Raw commands/logs are retained.
"""
import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import sys

from input_custody import destination, digest, overlaps, plain_path, strict_json, zip_files
from source_custody import snapshot

ROOT = Path(__file__).resolve().parents[1]


def inventory(directory):
    result = {}
    for p in directory.rglob('*'):
        plain_path(p)
        if p.is_file():
            with p.open('rb') as stream:
                result[p.relative_to(directory).as_posix()] = dict(bytes=p.stat().st_size,
                    sha256=hashlib.file_digest(stream, 'sha256').hexdigest())
        elif not p.is_dir():
            raise ValueError('Unexpected artifact filesystem node')
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--site', type=Path, required=True)
    parser.add_argument('--source-artifacts', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--browser', action='store_true')
    args = parser.parse_args()
    site, artifacts, output = plain_path(args.site), plain_path(args.source_artifacts), destination(args.output)
    if any(overlaps(output, p) for p in (ROOT, site, artifacts)):
        raise ValueError('Replay output overlaps source or artifacts')
    original_files, original_source = snapshot(ROOT)
    manifest = strict_json((site/'BUILD-MANIFEST.json').read_bytes())
    before = inventory(site)
    expected = {f['path']:dict(bytes=f['bytes'], sha256=f['sha256']) for f in manifest['files']}
    without_manifest = {k:v for k,v in before.items() if k != 'BUILD-MANIFEST.json'}
    source_before = inventory(artifacts)
    source_expected = {f['path']:dict(bytes=f['bytes'], sha256=f['sha256']) for f in manifest['source_release']['files']}
    if (expected != without_manifest or source_before != source_expected
            or manifest['commit'] != original_source['commit'] or manifest['tree'] != original_source['tree']
            or sum(v['bytes'] for v in before.values()) >= 5_000_000
            or source_before['source.tar.xz']['sha256'] != manifest['source_archive_sha256']):
        raise ValueError('Original complete source/runtime artifact identity differs')
    output.mkdir(parents=True)
    helper = output/'trusted-receiver'
    helper.mkdir()
    for name, (data, mode) in zip_files((artifacts/'receiver.zip').read_bytes()).items():
        path = helper/name
        path.parent.mkdir(parents=True, exist_ok=True)
        with path.open('xb') as stream:
            stream.write(data)
    env = os.environ.copy()
    for name in ['RUSTFLAGS', 'CARGO_ENCODED_RUSTFLAGS', 'TAXLANE_SITE_URL']:
        env.pop(name, None)
    env.update(CARGO_INCREMENTAL='0', CARGO_BUILD_JOBS='2', CI='1')
    commands = []
    source = output/'source'
    inputs, home, target = [output/name for name in ('inputs', 'cargo-home', 'empty-target')]
    regenerated = output/'regenerated-source-artifacts'

    def run(name, command, cwd, environment=env):
        with (output/(name+'.stdout')).open('xb') as stdout, (output/(name+'.stderr')).open('xb') as stderr:
            result = subprocess.run([str(v) for v in command], cwd=cwd, env=environment, stdout=stdout, stderr=stderr)
        commands.append(dict(name=name, command=[str(v) for v in command], returncode=result.returncode))
        (output/'commands.json').write_text(json.dumps(commands, indent=2)+'\n', encoding='utf8')
        if result.returncode:
            raise RuntimeError(name+' failed; raw evidence retained at '+str(output))
        print(name+' passed', flush=True)

    run('receive-without-git-cargo-path', [sys.executable, '-I', '-S', '-B', helper/'tools/receive-source.py',
        '--archive', artifacts/'source.tar.xz', '--sha256', manifest['source_archive_sha256'],
        '--output', source, '--protect', inputs, '--protect', home, '--protect', target], helper, dict(env, PATH=''))
    if (source/'.git').exists() or any(p.exists() for p in (inputs, home, target)):
        raise ValueError('Replay must start outside Git with absent inputs/home/compiled target')
    if snapshot(source) != (original_files, original_source):
        raise ValueError('Restored complete source differs')
    run('independent-empty-target-build', [sys.executable, '-B', source/'tools/build-pages.py',
        '--inputs', inputs, '--cargo-home', home, '--target', target, '--source-output', regenerated], source)
    rebuilt = source/'dist/TAXLANE'
    after, source_after = inventory(rebuilt), inventory(regenerated)
    if before != after or source_before != source_after:
        (output/'artifact-differences.json').write_text(json.dumps(dict(runtime_before=before, runtime_after=after,
            source_before=source_before, source_after=source_after), indent=2)+'\n', encoding='utf8')
        raise ValueError('Complete source/runtime bytes or manifests differ after independent rebuild')
    # Configure using the received helper, hence the received source root and
    # original owned inputs. Every command gets fresh matching path remaps.
    bootstrap = ('import json,subprocess,sys;from pathlib import Path;'
        'from rust_build import configure,finish,ROOT;'
        'context=configure(*[Path(v)for v in sys.argv[1:4]]);'
        'subprocess.run(sys.argv[4:],cwd=ROOT,check=True);'
        'print(json.dumps(finish(context)))')

    def native(name, command):
        run(name, [sys.executable, '-B', '-c', bootstrap, inputs, home, target, *command], source/'tools')

    native('native-tests', ['cargo', 'test', '--locked', '-p', 'taxlane-core', '-p', 'taxlane-scenario'])
    counts = [int(n) for n in re.findall(r'test result: ok\. (\d+) passed', (output/'native-tests.stdout').read_text(encoding='utf8'))]
    if sum(counts) != 156:
        raise ValueError('Original 156-test native coverage differs')
    native('adapter-format', ['cargo', 'fmt', '-p', 'taxlane-scenario', '--check'])
    native('adapter-clippy', ['cargo', 'clippy', '--locked', '--no-deps', '-p', 'taxlane-scenario',
        '--all-targets', '--all-features', '--', '-D', 'warnings'])
    native('full-data-guardrail', ['cargo', 'run', '--locked', '-p', 'taxlane-tools', '--', 'income-tax-outlay', 'validate'])
    run('full-native-wasm', [sys.executable, '-B', source/'tools/check-scenario-controls.py', rebuilt,
        output/'scenario-controls', '--inputs', inputs, '--cargo-home', home, '--target', target], source)
    if args.browser:
        browser_env = dict(env, TAXLANE_DIST=str(source/'dist'), PLAYWRIGHT_JSON_OUTPUT_FILE=str(output/'browser.json'))
        run('browser', ['npm.cmd' if os.name == 'nt' else 'npm', 'test', '--', '--reporter=list,json'], ROOT, browser_env)
    expected_capsule = (source/'cargo-inputs/inputs.sha256').read_text().split()[0]
    run('owned-input-readback', [sys.executable, '-I', '-S', '-B', source/'tools/input_custody.py', '--sha256',
        expected_capsule, '--inputs', inputs, '--cargo-home', home, '--readback'], source)
    if snapshot(source) != (original_files, original_source) or snapshot(ROOT) != (original_files, original_source):
        raise ValueError('Original or received complete source changed during qualification')
    receipt = dict(schema='taxlane.complete-source-runtime-replay.v1', commit=original_source['commit'], tree=original_source['tree'],
        source_files=len(original_files), source_bytes=sum(len(data) for data, mode in original_files.values()),
        source_archive_sha256=manifest['source_archive_sha256'], source_received_without_git_cargo_path=True,
        inputs_home_target_initially_absent=True, complete_runtime_and_source_artifacts_byte_exact=True,
        runtime_files=before, source_artifacts=source_before, native_tests=sum(counts),
        portable_native_wasm=json.loads((output/'scenario-controls/receipt.json').read_bytes())['counts'],
        browser_run=args.browser, browser_stats=json.loads((output/'browser.json').read_bytes())['stats'] if args.browser else None,
        scope='Same installed platform/toolchain; complete source outside Git, new owned offline inputs/home and initially absent target. No cross-platform binary or default/published/live claim.', commands=commands)
    (output/'receipt.json').write_text(json.dumps(receipt, indent=2)+'\n', encoding='utf8')
    print(json.dumps({k:v for k,v in receipt.items() if k not in ('runtime_files','source_artifacts','commands')}), flush=True)


if __name__ == '__main__':
    main()
