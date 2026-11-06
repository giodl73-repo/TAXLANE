"""Publication state-machine and anonymous byte controls; never touches GitHub."""
import copy
import io
from pathlib import Path
import unittest
from urllib.request import Request

from input_custody import digest
from source_release import NAMES, REPO, PublicRedirect, publish_or_verify, stream_identity


def package():
    commit = 'a'*40
    tag = 'source-'+commit
    return dict(commit=commit, tree='b'*40, tag=tag, base='https://github.com/'+REPO+'/releases/download/'+tag+'/',
        files={name:dict(bytes=len(name), sha256=digest(name.encode())) for name in sorted(NAMES)},
        artifacts=Path('fixture-not-read'), body='Exact source/body binding\n', title='Exact source title', runtime_manifest_sha256='c'*64)


class FakeGitHub:
    def __init__(self, data):
        self.package = data;self.release = None;self.assets = {};self.tag = None
        self.events = [];self.default = data['commit'];self.default_reads = 0
        self.advance_before_publish = False;self.corrupt_public = False;self.wrong_public_body = False
        self.missing_digest = False;self.draft_reads = 0;self.public_reads = []

    def api(self, route, anonymous=False, missing=False, method='GET', payload=None):
        p = self.package
        if route == '':return dict(full_name=REPO, private=False, default_branch='main')
        if route == 'git/commits/'+p['commit']:return dict(sha=p['commit'], tree=dict(sha=p['tree']))
        if route == 'git/ref/heads/main':
            self.default_reads += 1
            sha = '0'*40 if self.advance_before_publish and self.default_reads>1 else self.default
            return dict(object=dict(sha=sha, type='commit'))
        if route == 'git/ref/tags/'+p['tag']:
            return dict(object=dict(sha=self.tag, type='commit')) if self.tag else None
        if route == 'releases/tags/'+p['tag']:
            value = copy.deepcopy(self.release)
            if anonymous and self.wrong_public_body:value['body'] += 'different'
            return value
        if route == 'releases/1/assets?per_page=100':return copy.deepcopy(list(self.assets.values()))
        if route == 'releases/1' and method == 'PATCH':
            assert payload == {'draft':False, 'make_latest':'false'}
            assert set(self.assets) == set(p['files'])
            self.release['draft'] = False;self.tag = p['commit'];self.events.append('publish')
            return copy.deepcopy(self.release)
        raise AssertionError('Unexpected API route '+route)

    def create(self, p):
        assert self.release is None
        self.release = dict(id=1, tag_name=p['tag'], target_commitish=p['commit'], name=p['title'],
                            body=p['body'], draft=True, prerelease=False, immutable=False)
        self.events.append('create')

    def upload(self, p, name):
        assert self.release['draft'] and name not in self.assets
        self.assets[name] = dict(id=len(self.assets)+1, name=name, state='uploaded', size=p['files'][name]['bytes'],
            digest=None if self.missing_digest else 'sha256:'+p['files'][name]['sha256'], browser_download_url=p['base']+name)
        self.events.append('upload:'+name)

    def draft_bytes(self, asset, expected):
        self.draft_reads += 1
        return expected.copy()

    def public_bytes(self, url, expected):
        assert self.release and not self.release['draft']
        self.public_reads.append(url)
        return dict(expected, sha256='0'*64) if self.corrupt_public else expected.copy()


class SourceRelease(unittest.TestCase):
    def setUp(self):
        self.package = package();self.client = FakeGitHub(self.package)

    def complete_draft(self):
        self.client.create(self.package)
        for name in self.package['files']:self.client.upload(self.package, name)
        self.client.events.clear()

    def test_draft_upload_verify_publish_anonymous_all_assets(self):
        result = publish_or_verify(self.package, self.client, publish=True)
        self.assertTrue(result['anonymous_complete_readback'])
        self.assertEqual(set(result['files']), NAMES)
        self.assertEqual(self.client.events[0], 'create')
        self.assertEqual(self.client.events[-1], 'publish')
        self.assertEqual(len(self.client.public_reads), len(NAMES))
        self.assertFalse(result['host_release_immutable'])

    def test_idempotent_published_and_historical_read_only(self):
        publish_or_verify(self.package, self.client, publish=True)
        self.client.events.clear()
        publish_or_verify(self.package, self.client, publish=True)
        self.assertEqual(self.client.events, [])
        self.client.default = 'd'*40
        publish_or_verify(self.package, self.client, publish=False)
        self.assertEqual(self.client.events, [])

    def test_read_only_never_creates_a_release(self):
        with self.assertRaises(ValueError):publish_or_verify(self.package, self.client)
        self.assertEqual(self.client.events, [])

    def test_resume_matching_partial_draft_without_clobber(self):
        self.client.create(self.package)
        first = next(iter(self.package['files']));self.client.upload(self.package, first)
        self.client.events.clear()
        publish_or_verify(self.package, self.client, publish=True)
        self.assertNotIn('create', self.client.events)
        self.assertNotIn('upload:'+first, self.client.events)
        self.assertEqual(self.client.events[-1], 'publish')

    def test_tag_conflict_refuses_movement_before_mutation(self):
        self.client.tag = 'd'*40
        with self.assertRaises(ValueError):publish_or_verify(self.package, self.client, publish=True)
        self.assertEqual(self.client.events, [])

    def test_body_conflict_refuses_overwrite(self):
        self.complete_draft();self.client.release['body'] += 'different'
        with self.assertRaises(ValueError):publish_or_verify(self.package, self.client, publish=True)
        self.assertEqual(self.client.events, [])

    def test_asset_conflicts_refuse_overwrite(self):
        for field, value in [('size',0), ('digest','sha256:'+'0'*64), ('state','starter'), ('browser_download_url','https://invalid.example/file')]:
            with self.subTest(field=field):
                self.setUp();self.complete_draft()
                self.client.assets[next(iter(self.package['files']))][field] = value
                with self.assertRaises(ValueError):publish_or_verify(self.package, self.client, publish=True)
                self.assertEqual(self.client.events, [])

    def test_unexpected_existing_asset_is_preserved_and_rejected(self):
        self.complete_draft();self.client.assets['unrelated'] = dict(name='unrelated')
        with self.assertRaises(ValueError):publish_or_verify(self.package, self.client, publish=True)
        self.assertEqual(self.client.events, [])

    def test_noncurrent_source_never_starts_publication(self):
        self.client.default = 'd'*40
        with self.assertRaises(ValueError):publish_or_verify(self.package, self.client, publish=True)
        self.assertEqual(self.client.events, [])

    def test_default_advance_leaves_verified_draft_unpublished(self):
        self.client.advance_before_publish = True
        with self.assertRaises(ValueError):publish_or_verify(self.package, self.client, publish=True)
        self.assertTrue(self.client.release['draft']);self.assertNotIn('publish', self.client.events)

    def test_missing_digest_requires_complete_draft_bytes(self):
        self.client.missing_digest = True
        publish_or_verify(self.package, self.client, publish=True)
        self.assertEqual(self.client.draft_reads, len(NAMES))

    def test_anonymous_binary_or_metadata_corruption_blocks_receipt(self):
        for field in ['corrupt_public', 'wrong_public_body']:
            with self.subTest(field=field):
                self.setUp();setattr(self.client, field, True)
                with self.assertRaises(ValueError):publish_or_verify(self.package, self.client, publish=True)

    def test_streaming_checks_complete_bytes_and_rejects_overflow(self):
        self.assertEqual(stream_identity(io.BytesIO(b'abc'),3), dict(bytes=3, sha256=digest(b'abc')))
        self.assertEqual(stream_identity(io.BytesIO(b'ab'),3)['bytes'],2)
        with self.assertRaises(ValueError):stream_identity(io.BytesIO(b'abcd'),3)

    def test_public_redirect_requires_https_and_no_credentials(self):
        handler = PublicRedirect();request = Request('https://github.com/example')
        for target in ['http://example.com/file','https://user:secret@example.com/file']:
            with self.assertRaises(ValueError):handler.redirect_request(request,None,302,'Found',{},target)
        request.add_header('Authorization','must-not-forward')
        with self.assertRaises(ValueError):handler.redirect_request(request,None,302,'Found',{},'https://example.com/file')


if __name__ == '__main__':
    unittest.main()
