"""Complete source packaging controls; fixture code is never executed."""
import io
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch
import zipfile

from input_custody import digest, source_tree
import source_custody as source
from source_artifacts import package, receiver_bundle


class SourceArtifacts(unittest.TestCase):
    def setUp(self):
        self.files = {name: (('Original complete '+name+'\n').encode(), 0o100644) for name in [
            'LICENSE', 'tools/receive-source.py', 'tools/source_custody.py', 'tools/input_custody.py',
            'notices/LATTICE-COMPONENTS-LICENSE.txt', 'notices/WITNESS-COMPONENTS-LICENSE.txt',
            'notices/TERRAIN-COMPONENTS-LICENSE.txt', 'data/raw/retained.dat']}
        self.commit = b'tree '+source_tree(self.files).encode()+b'\nauthor A <a@b> 0 +0000\ncommitter A <a@b> 0 +0000\n\nFixture\n'
        self.manifest = source.manifest_for(self.files, self.commit)

    def test_complete_source_and_all_artifacts_regenerate(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            restored = source.restore(self.files, self.manifest, root/'source')
            first, second = root/'first', root/'second'
            receipt = package(restored, first)
            self.assertEqual(receipt, package(restored, second))
            self.assertEqual({p.name:p.read_bytes() for p in first.iterdir()},
                             {p.name:p.read_bytes() for p in second.iterdir()})
            for row in receipt['files']:
                value = (first/row['path']).read_bytes()
                self.assertEqual((len(value), digest(value)), (row['bytes'], row['sha256']))
            raw = (first/'source.tar.xz').read_bytes()
            self.assertEqual(source.verify(raw, digest(raw)), (self.files, self.manifest))
            self.assertEqual(receipt['source_files'], len(self.files))

    def test_receiver_preserves_code_and_complete_original_grants(self):
        raw = receiver_bundle(self.files)
        self.assertEqual(raw, receiver_bundle(self.files))
        with zipfile.ZipFile(io.BytesIO(raw)) as bundle:
            for name in self.files:
                if name != 'data/raw/retained.dat':
                    self.assertEqual(bundle.read(name), self.files[name][0])
            for entry in bundle.infolist():
                self.assertEqual(entry.external_attr >> 16, 0o100644)
                self.assertEqual(entry.date_time, (1980, 1, 1, 0, 0, 0))

    def test_existing_and_source_destinations_preserved(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            restored = source.restore(self.files, self.manifest, root/'source')
            marker = root/'existing';marker.write_bytes(b'keep')
            for output in [marker, restored/'nested', root]:
                with self.assertRaises(ValueError):
                    package(restored, output)
            self.assertEqual(marker.read_bytes(), b'keep')
            self.assertFalse((restored/'nested').exists())

    def test_compiled_target_overlap_rejected_before_writes(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            restored = source.restore(self.files, self.manifest, root/'source')
            with patch.dict('os.environ', {'CARGO_TARGET_DIR':str(root/'target')}):
                with self.assertRaises(ValueError):
                    package(restored, root/'target'/'artifacts')
            self.assertFalse((root/'target').exists())

    def test_changed_received_source_is_not_packaged(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            restored = source.restore(self.files, self.manifest, root/'source')
            (restored/'data/raw/retained.dat').write_bytes(b'changed')
            with self.assertRaises(ValueError):
                package(restored, root/'output')
            self.assertFalse((root/'output').exists())


if __name__ == '__main__':
    unittest.main()
