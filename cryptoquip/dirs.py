
from pathlib import Path

from . import utils

APP_DIR     = utils.get_app_dir()
INSTALL_DIR = APP_DIR.parent if utils.INSTALLED else Path('.')