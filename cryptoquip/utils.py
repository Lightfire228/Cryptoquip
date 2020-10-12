
from pathlib     import Path
from collections import namedtuple

from bs4 import BeautifulSoup

import os
import sys

Dim = namedtuple('Dim', ['x1', 'y1'])
Box = namedtuple('Box', ['x1', 'y1', 'x2', 'y2'])

APP_DIR = None

def get_version():
    version_file = Path('./version.txt')
    try:
        version_text = version_file.read_text().strip()

        return version_text
    except:
        return None

# https://stackoverflow.com/a/42615559/2716305
def _get_app_dir():

    if getattr(sys, 'frozen', False):
        return Path(sys._MEIPASS) # pylint: disable=no-member
    else:
        return Path(__file__).parent


APP_DIR = _get_app_dir()