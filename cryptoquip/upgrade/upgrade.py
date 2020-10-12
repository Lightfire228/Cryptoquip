from io      import BytesIO
from pathlib import Path

import shutil
import subprocess
import sys
import zipfile

import requests

from .. import utils

MAIN_LINK_FILE = utils.APP_DIR.parent / 'main.lnk'

def upgrade(upgrade_context):

    zip_data = _download_upgrade(upgrade_context)

    _extract_zip  (upgrade_context, zip_data)
    _set_link_file(upgrade_context)


def _download_upgrade(upgrade_context):

    if upgrade_context.is_local:
        return _download_local_upgrade(upgrade_context)

    url     = upgrade_context.asset_url
    headers = upgrade_context.asset_headers

    try:
        print('Downloading zip file:', url)
        r = requests.get(url, headers=headers)
        r.raise_for_status()

        zip_data = BytesIO()
        zip_data.write(r.content)

        return zip_data

    except:
        raise Exception('Unable to find the zip file.  Contact your local tech support.')

def delete_old_version(old_dir):
    shutil.rmtree(old_dir)

def _download_local_upgrade(upgrade_context):

    zip_file = Path(upgrade_context.asset_url)

    zip_data = BytesIO()
    zip_data.write(zip_file.read_bytes())

    return zip_data


def _extract_zip(upgrade_context, zip_data):

    upgrade_dir = upgrade_context.upgrade_dir

    upgrade_dir.mkdir(exist_ok=True)


    print('Extracting zip')
    with zipfile.ZipFile(zip_data) as zip:
        zip.extractall(upgrade_dir)

def _set_link_file(upgrade_context):
    print('symlinking', MAIN_LINK_FILE)
    print('symlinking to', upgrade_context.upgrade_dir / 'main.exe')

    MAIN_LINK_FILE.unlink(missing_ok=True)
    MAIN_LINK_FILE.symlink_to(upgrade_context.upgrade_dir / 'main.exe')