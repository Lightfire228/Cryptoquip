import shutil
import zipfile

import requests

from .. import dirs
from .. import utils

MAIN_LINK_FILE = dirs.INSTALL_DIR / 'main.exe'

def update(update_context):

    zip_data = update_context.download()

    _extract_zip  (update_context, zip_data)
    _set_link_file(update_context)


def delete_old_version(old_dir):

    if not dirs.INSTALL_DIR.parents in old_dir.parents:
        raise Exception(f'Trying to remove old app dir "{old_dir}" which is not a subdir of "{dirs.INSTALL_DIR}"!')

    shutil.rmtree(old_dir)

def _extract_zip(update_context, zip_data):

    update_dir = update_context.update_dir

    update_dir.mkdir(exist_ok=True)


    utils.log('Extracting zip')
    with zipfile.ZipFile(zip_data) as zip:
        zip.extractall(update_dir)

def _set_link_file(update_context):
    utils.log('symlinking', MAIN_LINK_FILE)
    utils.log('symlinking to', update_context.update_dir / 'main.exe')

    MAIN_LINK_FILE.unlink(missing_ok=True)
    MAIN_LINK_FILE.symlink_to(update_context.update_dir / 'main.exe')