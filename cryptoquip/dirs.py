
from pathlib import Path

import sys

# https://stackoverflow.com/a/42615559/2716305
INSTALLED = getattr(sys, 'frozen', False)

# pylint: disable=no-member
APP_DIR     = Path(sys._MEIPASS) if INSTALLED else Path(__file__).parent.parent
INSTALL_DIR = APP_DIR.parent     if INSTALLED else APP_DIR