"""Verify complete source/runtime custody, publish without clobber, read back anonymously."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import tempfile
from urllib.error import HTTPError
from urllib.request import Request, HTTPRedirectHandler, build_opener
from urllib.parse import urlsplit

from input_custody import destination, digest, overlaps, path_set, plain_path, strict_json, verify as verify_inputs
from source_artifacts import receiver_bundle
from source_custody import encoded, verify as verify_source

REPO = 'giodl73-repo/TAXLANE'
API = 'https://api.github.com/repos/'+REPO
NAMES = {'source.tar.xz', 'receiver.zip', 'SOURCE-MANIFEST.json', 'SOURCE-ARTIFACTS.json',
         'LICENSE', 'LATTICE-COMPONENTS-LICENSE.txt', 'WITNESS-COMPONENTS-LICENSE.txt', 'TERRAIN-COMPONENTS-LICENSE.txt'}


def require(condition, message):
    if not condition:
        raise ValueError(message)


def inventory(root):
    result = {}
    for path in root.rglob('*'):
        plain_path(path)
        if path.is_file():
            with path.open('rb') as stream:
                result[path.relative_to(root).as_posix()] = dict(bytes=path.stat().st_size,
                    sha256=hashlib.file_digest(stream, 'sha256').hexdigest())
        else:
            require(path.is_dir(), 'Unexpected artifact filesystem node')
    path_set(list(result))
    return result


def local_package(site, artifacts, expected_commit):
    require(re.fullmatch('[0-9a-f]{40}', expected_commit), 'Expected a full source commit')
    site, artifacts = plain_path(site), plain_path(artifacts)
    manifest_raw = (site/'BUILD-MANIFEST.json').read_bytes()
    manifest = strict_json(manifest_raw)
    require(manifest['schema'] == 'taxlane.pages-build.v1' and manifest['commit'] == expected_commit,
            'Runtime/source commit differs')
    runtime = inventory(site)
    expected = {f['path']:dict(bytes=f['bytes'], sha256=f['sha256']) for f in manifest['files']}
    require(len(expected) == len(manifest['files']) and expected == {k:v for k,v in runtime.items() if k != 'BUILD-MANIFEST.json'},
            'Complete runtime file inventory differs')
    require(sum(f['bytes'] for f in runtime.values()) < 5_000_000, 'Runtime delivery budget exceeded')
    actual = inventory(artifacts)
    expected = {f['path']:dict(bytes=f['bytes'], sha256=f['sha256']) for f in manifest['source_release']['files']}
    require(set(actual) == NAMES and actual == expected and len(expected) == len(manifest['source_release']['files']),
            'Complete source artifact set or hashes differ')
    tag = 'source-'+expected_commit
    base = 'https://github.com/'+REPO+'/releases/download/'+tag+'/'
    require(manifest['source_release']['tag'] == tag and manifest['source_release']['base_url'] == base,
            'Unexpected source publication route')
    require(actual['source.tar.xz']['sha256'] == manifest['source_archive_sha256'], 'Source archive hash differs')
    files, source = verify_source((artifacts/'source.tar.xz').read_bytes(), manifest['source_archive_sha256'])
    require(source['commit'] == expected_commit and source['tree'] == manifest['tree'], 'Complete source Git identity differs')
    require(manifest['runtime'] == 'runtime/'+expected_commit, 'Runtime asset path is not source-bound')
    provenance = strict_json((site/'provenance.json').read_bytes())
    require(provenance['commit'] == expected_commit and provenance['tree'] == source['tree']
            and provenance['dirty'] is False and provenance['bindings'] == 'wasm-bindgen 0.2.127'
            and provenance['rust'].startswith('rustc 1.95.0 '), 'Runtime provenance differs')
    require((site/'LICENSE').read_bytes() == files['LICENSE'][0]
            and (site/'DEPENDENCY-NOTICES.txt').read_bytes() == files['DEPENDENCY-NOTICES.txt'][0],
            'Complete runtime source/dependency grants differ')
    helper_grants = b'\n\n'.join(files['notices/'+name+'-COMPONENTS-LICENSE.txt'][0] for name in ['LATTICE','WITNESS','TERRAIN'])
    require((site/'COMPONENT-NOTICES.txt').read_bytes() == helper_grants, 'Complete runtime helper grants differ')
    runtime_notice = strict_json(files['notices/RUNTIME-NOTICES.json'][0])
    require(runtime_notice['toolchain'] == '1.95.0' and runtime_notice['file'] == 'rust-1.95.0-COPYRIGHT-library.html',
            'Unexpected Rust runtime notice identity')
    notice = files['notices/'+runtime_notice['file']][0]
    require(len(notice) == runtime_notice['bytes'] and digest(notice) == runtime_notice['sha256']
            and (site/'RUST-RUNTIME-NOTICES.html').read_bytes() == notice, 'Complete runtime standard-library grants differ')
    require((artifacts/'SOURCE-MANIFEST.json').read_bytes() == encoded(source), 'External complete source manifest differs')
    receipt = strict_json((artifacts/'SOURCE-ARTIFACTS.json').read_bytes())
    require(receipt == manifest['source_artifacts'] and receipt['commit'] == expected_commit and receipt['tree'] == source['tree']
            and receipt['source_files'] == len(files) and receipt['source_bytes'] == sum(len(v) for v,m in files.values()),
            'Source artifact receipt identity/counts differ')
    expected_inner = {f['path']:dict(bytes=f['bytes'], sha256=f['sha256']) for f in receipt['files']}
    require(len(expected_inner) == len(receipt['files']) and expected_inner == {k:v for k,v in actual.items() if k != 'SOURCE-ARTIFACTS.json'},
            'Source artifact receipt inventory differs')
    require((artifacts/'LICENSE').read_bytes() == files['LICENSE'][0]
            and (artifacts/'receiver.zip').read_bytes() == receiver_bundle(files), 'Original source/receiver/grants differ')
    for name in ['LATTICE', 'WITNESS', 'TERRAIN']:
        grant = name+'-COMPONENTS-LICENSE.txt'
        require((artifacts/grant).read_bytes() == files['notices/'+grant][0], 'Original helper grant differs')
    capsule_sha = files['cargo-inputs/inputs.sha256'][0].decode().split()[0]
    inputs = verify_inputs(files['cargo-inputs/inputs.zip'][0], capsule_sha, files['Cargo.lock'][0])
    require(inputs['notices'] == files['DEPENDENCY-NOTICES.txt'][0], 'Original dependency notices differ')
    body = ('Complete TAXLANE source for '+expected_commit+'\n\n'
            'Git tree: '+source['tree']+'\n'
            'Runtime BUILD-MANIFEST SHA256: '+digest(manifest_raw)+'\n'
            'Source artifacts (bytes/SHA256):\n'+encoded(actual).decode()+'\n'
            'Original split software/content and source-specific grants are preserved.\n'
            'Source availability does not admit fiscal/research claims.\n'
            'Content/hash identity and publisher no-overwrite checks are not a claim of host-enforced immutability.\n')
    return dict(commit=expected_commit, tree=source['tree'], tag=tag, base=base, files=actual,
        artifacts=artifacts, body=body, title='TAXLANE complete source '+expected_commit,
        runtime_manifest_sha256=digest(manifest_raw))


class NoRedirect(HTTPRedirectHandler):
    def redirect_request(self, request, *args, **kwargs):
        return None


class PublicRedirect(HTTPRedirectHandler):
    def redirect_request(self, request, fp, code, message, headers, new_url):
        target = urlsplit(new_url)
        require(target.scheme == 'https' and not target.username and not target.password,
                'Anonymous asset redirect must remain HTTPS without credentials')
        require(not any(k.lower() in ('authorization', 'cookie') for k,v in request.header_items()),
                'Anonymous asset request contains credentials')
        return super().redirect_request(request, fp, code, message, headers, new_url)


class GitHub:
    def api(self, route, *, anonymous=False, missing=False, method='GET', payload=None):
        headers = {'Accept':'application/vnd.github+json', 'X-GitHub-Api-Version':'2026-03-10', 'User-Agent':'TAXLANE-source-custody'}
        token = os.environ.get('GH_TOKEN')
        if not anonymous and token:
            headers['Authorization'] = 'Bearer '+token
        if method != 'GET':
            require(not anonymous and bool(token), 'Set the intended account GH_TOKEN explicitly')
        data = encoded(payload) if payload is not None else None
        if data is not None:
            headers['Content-Type'] = 'application/json'
        request = Request(API+('/'+route if route else ''), data=data, headers=headers, method=method)
        try:
            with build_opener(NoRedirect()).open(request, timeout=60) as response:
                raw = response.read(4_000_001)
                require(len(raw) <= 4_000_000, 'API JSON byte limit')
                return strict_json(raw)
        except HTTPError as error:
            if missing and error.code == 404:
                return None
            raise RuntimeError('GitHub '+method+' '+route+' returned HTTP '+str(error.code)) from None

    def command(self, *args):
        environment = os.environ.copy()
        environment.pop('GH_DEBUG', None)
        subprocess.run(['gh', *args, '--repo', REPO], env=environment, check=True,
                       stdout=subprocess.DEVNULL)

    def create(self, package):
        with tempfile.TemporaryDirectory(prefix='taxlane-release-') as temp:
            body = Path(temp)/'notes.md'
            body.write_bytes(package['body'].encode())
            self.command('release', 'create', package['tag'], '--draft', '--latest=false',
                '--target', package['commit'], '--title', package['title'], '--notes-file', str(body))

    def upload(self, package, name):
        self.command('release', 'upload', package['tag'], str(package['artifacts']/name))

    def draft_bytes(self, asset, expected):
        # gh follows authenticated asset API redirects without exposing binary
        # data or credentials to logs. Never capture a 180 MB body as text.
        environment = os.environ.copy();environment.pop('GH_DEBUG', None)
        with tempfile.TemporaryFile() as errors:
            with subprocess.Popen(['gh', 'api', 'repos/'+REPO+'/releases/assets/'+str(asset['id']),
                '-H', 'Accept: application/octet-stream'], env=environment, stdout=subprocess.PIPE, stderr=errors) as process:
                try:
                    result = stream_identity(process.stdout, expected['bytes'])
                    require(process.wait() == 0, 'Draft asset binary readback failed')
                    return result
                except BaseException:
                    process.kill();process.wait();raise

    def public_bytes(self, url, expected):
        # No auth headers, cookies, gh configuration or downloaded code involved.
        request = Request(url, headers={'User-Agent':'TAXLANE-anonymous-source-readback'})
        with build_opener(PublicRedirect()).open(request, timeout=60) as response:
            return stream_identity(response, expected['bytes'])


def stream_identity(stream, limit):
    total = 0;sha = hashlib.sha256()
    while True:
        block = stream.read(min(65536, limit-total+1))
        if not block:
            return dict(bytes=total, sha256=sha.hexdigest())
        total += len(block)
        require(total <= limit, 'Asset byte limit exceeded')
        sha.update(block)


def tag_commit(client, tag, anonymous=False):
    ref = client.api('git/ref/tags/'+tag, anonymous=anonymous, missing=True)
    if ref is None:
        return None
    obj = ref['object']
    for depth in range(8):
        require(re.fullmatch('[0-9a-f]{40}', obj['sha']), 'Unexpected tag object SHA')
        if obj['type'] == 'commit':
            return obj['sha']
        require(obj['type'] == 'tag', 'Source tag must resolve to a commit')
        obj = client.api('git/tags/'+obj['sha'], anonymous=anonymous)['object']
    raise ValueError('Source tag nesting exceeds limit')


def check_release(release, package):
    require(release['tag_name'] == package['tag'] and release['target_commitish'] == package['commit']
            and release['name'] == package['title'] and release['body'] == package['body']
            and not release['prerelease'], 'Existing release identity/body differs; no overwrite')


def assets_for(client, release, package, anonymous=False):
    assets = client.api('releases/'+str(release['id'])+'/assets?per_page=100', anonymous=anonymous)
    result = {asset['name']:asset for asset in assets}
    require(len(result) == len(assets) and set(result) <= set(package['files']), 'Unexpected/duplicate existing release assets')
    return result


def check_asset(client, asset, expected, package, name, anonymous=False):
    require(asset['state'] == 'uploaded' and asset['size'] == expected['bytes']
            and asset['browser_download_url'] == package['base']+name, 'Existing asset size/state/route differs; no overwrite')
    if asset.get('digest'):
        require(asset['digest'] == 'sha256:'+expected['sha256'], 'Existing asset digest differs; no overwrite')
    else:
        actual = client.public_bytes(package['base']+name, expected) if anonymous else client.draft_bytes(asset, expected)
        require(actual == expected, 'Existing asset complete bytes differ; no overwrite')
        return actual


def publish_or_verify(package, client, publish=False):
    repo = client.api('')
    require(repo['full_name'] == REPO and not repo['private'] and repo['default_branch'] == 'main', 'Unexpected repository/publication boundary')
    commit = client.api('git/commits/'+package['commit'])
    require(commit['sha'] == package['commit'] and commit['tree']['sha'] == package['tree'], 'Hosted source commit/tree differs')

    def current_default():
        require(client.api('git/ref/heads/main')['object']['sha'] == package['commit'], 'Only the current default source may publish')

    if publish:
        current_default()
    target = tag_commit(client, package['tag'])
    require(target in (None, package['commit']), 'Existing source tag points elsewhere; no movement')
    release = client.api('releases/tags/'+package['tag'], missing=True)
    if release is None:
        require(publish, 'Source release is not published')
        client.create(package)
        release = client.api('releases/tags/'+package['tag'])
    check_release(release, package)
    assets = assets_for(client, release, package)
    for name, expected in package['files'].items():
        if name not in assets:
            require(publish and release['draft'], 'Missing asset on published/read-only release; no overwrite')
            client.upload(package, name)
            assets = assets_for(client, release, package)
        check_asset(client, assets[name], expected, package, name, anonymous=not release['draft'])
    require(set(assets) == set(package['files']), 'Incomplete source asset set')
    if release['draft']:
        require(publish, 'Source release is still a draft')
        current_default()
        require(tag_commit(client, package['tag']) in (None, package['commit']), 'Source tag changed before publication')
        release = client.api('releases/'+str(release['id']), method='PATCH', payload={'draft':False, 'make_latest':'false'})
        check_release(release, package)
    require(not release['draft'] and tag_commit(client, package['tag']) == package['commit'], 'Published source/tag identity differs')
    public = client.api('releases/tags/'+package['tag'], anonymous=True)
    check_release(public, package)
    require(not public['draft'] and tag_commit(client, package['tag'], anonymous=True) == package['commit'], 'Anonymous release/tag identity differs')
    public_assets = assets_for(client, public, package, anonymous=True)
    require(set(public_assets) == set(package['files']), 'Anonymous source asset set differs')
    readback = {}
    for name, expected in package['files'].items():
        verified = check_asset(client, public_assets[name], expected, package, name, anonymous=True)
        actual = verified or client.public_bytes(package['base']+name, expected)
        require(actual == expected, 'Anonymous complete asset readback differs: '+name)
        readback[name] = dict(url=package['base']+name, **actual)
    return dict(schema='taxlane.source-publication.v1', commit=package['commit'], tree=package['tree'],
        tag=package['tag'], release_id=release['id'], runtime_manifest_sha256=package['runtime_manifest_sha256'],
        anonymous_complete_readback=True, files=readback, host_release_immutable=public.get('immutable'),
        scope='Verified complete source/runtime identity, no tag movement/asset clobber, and anonymous complete asset bytes. Hash identity is not authorship or fiscal admission.')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--site', type=Path, required=True)
    parser.add_argument('--artifacts', type=Path, required=True)
    parser.add_argument('--expected-commit', required=True)
    parser.add_argument('--receipt', type=Path, required=True)
    mode = parser.add_mutually_exclusive_group(required=True)
    mode.add_argument('--publish', action='store_true')
    mode.add_argument('--verify-published', action='store_true')
    args = parser.parse_args()
    args.receipt = destination(args.receipt)
    require(not any(overlaps(args.receipt, plain_path(p)) for p in [args.site, args.artifacts]),
            'Publication receipt overlaps source/runtime artifacts')
    if args.publish:
        require(bool(os.environ.get('GH_TOKEN')), 'Set the intended account GH_TOKEN explicitly')
        require(os.environ.get('GITHUB_REPOSITORY', REPO) == REPO and
                os.environ.get('GITHUB_REF', 'refs/heads/main') == 'refs/heads/main', 'Only main may publish source')
    package = local_package(args.site, args.artifacts, args.expected_commit)
    result = publish_or_verify(package, GitHub(), publish=args.publish)
    args.receipt.parent.mkdir(parents=True, exist_ok=True)
    with args.receipt.open('xb') as stream:
        stream.write(encoded(result))
    print(json.dumps(result, indent=2))
