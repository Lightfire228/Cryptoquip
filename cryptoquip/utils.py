
from pathlib     import Path
from collections import namedtuple

from bs4 import BeautifulSoup

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
