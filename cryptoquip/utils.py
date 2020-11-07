
from pathlib     import Path
from collections import namedtuple

from bs4 import BeautifulSoup

import argparse
import sys

Dim = namedtuple('Dim', ['x1', 'y1'])
Box = namedtuple('Box', ['x1', 'y1', 'x2', 'y2'])

# https://stackoverflow.com/a/42615559/2716305
INSTALLED = getattr(sys, 'frozen', False)

def get_version():
    version_file = Path('./version.txt')
    try:
        version_text = version_file.read_text().strip()

        return version_text
    except:
        return None

def get_app_dir():

    if INSTALLED:
        return Path(sys._MEIPASS) # pylint: disable=no-member
    else:
        return Path(__file__).parent

def log(*args, **kwargs):
    print(*args,  **kwargs)

def _parse_args():
    parser = argparse.ArgumentParser()
    parser.add_argument('-s', '--skip-menu', action='store_true')
    parser.add_argument('-d', '--day', type=int, default=0)
    parser.add_argument('-w', '--wait-for-pid', type=int, default=0)

    return parser.parse_args()

args = _parse_args()