
from pathlib     import Path
from collections import namedtuple

from bs4 import BeautifulSoup

Dim = namedtuple('Dim', ['x1', 'y1'])
Box = namedtuple('Box', ['x1', 'y1', 'x2', 'y2'])

def get_version():
    version_file = Path('./version.txt')
    try:
        version_text = version_file.read_text().strip()

        return version_text
    except:
        return None