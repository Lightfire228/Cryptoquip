
from pathlib     import Path
from collections import namedtuple

from bs4 import BeautifulSoup

import argparse

from . import dirs

Dim = namedtuple('Dim', ['x1', 'y1'])
Box = namedtuple('Box', ['x1', 'y1', 'x2', 'y2'])

def get_version():
    version_file = dirs.APP_DIR / 'version'

    try:
        version_text = version_file.read_text().strip()

        return version_text
    except:
        return None


def log(*args, **kwargs):
    print(*args,  **kwargs)

def _parse_args():
    parser = argparse.ArgumentParser()
    parser.add_argument('-s', '--skip-menu', action='store_true')
    parser.add_argument('-d', '--day', type=int, default=0)
    parser.add_argument('-w', '--wait-for-pid', type=int, default=0)

    return parser.parse_args()

args = _parse_args()