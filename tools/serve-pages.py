"""Serve exactly the selected local Pages tree for browser qualification."""
from functools import partial
from http.server import SimpleHTTPRequestHandler, ThreadingHTTPServer
import os
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
directory = Path(os.environ.get('TAXLANE_DIST', ROOT/'dist')).resolve()
port = int(os.environ.get('TAXLANE_TEST_PORT', '8766'))
if not directory.is_dir() or not 1024 <= port <= 65535:
    raise ValueError('Select an existing dist directory and a valid test port')
ThreadingHTTPServer(('127.0.0.1', port), partial(SimpleHTTPRequestHandler, directory=str(directory))).serve_forever()
