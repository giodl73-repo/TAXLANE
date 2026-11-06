"""Package the complete committed source and a separately trusted receiver.

No source release is published by this command. Research/data grants remain
source-specific; a source snapshot does not admit fiscal or research claims.
"""
import argparse
import io
import json
import os
from pathlib import Path
import zipfile

from input_custody import destination, digest, overlaps, plain_path
from source_custody import archive, encoded, snapshot, verify

ROOT = Path(__file__).resolve().parents[1]


def receiver_bundle(files):
    names = ['tools/receive-source.py', 'tools/source_custody.py', 'tools/input_custody.py',
             'LICENSE', 'notices/LATTICE-COMPONENTS-LICENSE.txt',
             'notices/WITNESS-COMPONENTS-LICENSE.txt', 'notices/TERRAIN-COMPONENTS-LICENSE.txt']
    entries = {name: files[name][0] for name in names}
    entries['README.txt'] = (
        'TAXLANE complete-source receiver: Python 3.11+ standard library only.\n'
        'Inspect/trust the receiver before running it. Obtain the expected archive\n'
        'SHA256 through a trusted channel. Verification executes no downloaded source.\n'
        'python -I -S -B tools/receive-source.py --archive /downloads/source.tar.xz '
        '--sha256 EXPECTED --output /new/TAXLANE --protect /compiled-target\n'
        'Use a NEW output outside helper/download/compiled-target directories.\n'
        'Full-source bounds: 256,000,000 packed bytes; 512,000,000 decoded tar bytes;\n'
        '6,000 original files. Verification holds bounded decoded source in memory;\n'
        'allow several GB of available RAM plus disk space for source and builds.\n'
        'Original split LICENSE and complete helper MIT grants are included.\n'
        'Third-party/source-specific data terms remain in their original source files.\n'
        'Hash/Git identity proves byte identity, not authorship or fiscal admission.\n'
        'Python, Rust1.95.0, wasm-bindgen0.2.127, browser/npm and research-model\n'
        'tools are external prerequisites, not installed by this receiver.\n').encode()
    output = io.BytesIO()
    # This small trusted-helper bundle must reproduce across the supported
    # Python/platform versions without depending on their deflate encoders.
    with zipfile.ZipFile(output, 'w', compression=zipfile.ZIP_STORED) as bundle:
        for name, value in sorted(entries.items()):
            item = zipfile.ZipInfo(name, (1980, 1, 1, 0, 0, 0))
            item.create_system = 3
            item.external_attr = 0o100644 << 16
            item.compress_type = zipfile.ZIP_STORED
            bundle.writestr(item, value)
    return output.getvalue()


def package(source, output):
    output = destination(output)
    source = plain_path(source)
    if overlaps(output, source) and not (source/'dist' in output.parents):
        raise ValueError('Source artifact output must be external or under source/dist')
    if os.environ.get('CARGO_TARGET_DIR') and overlaps(output, plain_path(os.environ['CARGO_TARGET_DIR'])):
        raise ValueError('Source artifacts overlap compiled target')
    files, manifest = snapshot(source)
    raw = archive(files, manifest)
    checked_files, checked_manifest = verify(raw, digest(raw))
    if checked_files != files or checked_manifest != manifest:
        raise ValueError('Full source readback differs')
    receiver = receiver_bundle(files)
    entries = {'source.tar.xz': raw, 'receiver.zip': receiver,
               'SOURCE-MANIFEST.json': encoded(manifest), 'LICENSE': files['LICENSE'][0]}
    for name in ['LATTICE', 'WITNESS', 'TERRAIN']:
        path = 'notices/'+name+'-COMPONENTS-LICENSE.txt'
        entries[name+'-COMPONENTS-LICENSE.txt'] = files[path][0]
    receipt = dict(schema='taxlane.source-artifacts.v1', commit=manifest['commit'], tree=manifest['tree'],
        source_files=len(files), source_bytes=sum(len(data) for data, mode in files.values()),
        files=[dict(path=name, bytes=len(data), sha256=digest(data)) for name, data in sorted(entries.items())],
        scope='Complete committed source, original locked Rust capsule and grants; no fiscal admission or host immutability claim')
    output.mkdir(parents=True, exist_ok=False)
    for name, data in entries.items():
        with (output/name).open('xb') as stream:
            stream.write(data)
    (output/'SOURCE-ARTIFACTS.json').write_bytes(encoded(receipt))
    return receipt


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--source', type=Path, default=ROOT)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    print(json.dumps(package(args.source, args.output), indent=2))


if __name__ == '__main__':
    main()
