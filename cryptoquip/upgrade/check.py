from datetime import datetime, timedelta
from pathlib  import Path
from io       import BytesIO

import os
import shutil
import subprocess
import sys
import zipfile

from packaging import version

import requests

from .. import utils
from .. import config

BASE_URL = 'https://api.github.com/repos/Lightfire228/Cryptoquip/releases'
HEADERS  = {
    'Accept': 'application/vnd.github.v3+json'
}

ZIP_FILE_TYPE = 'application/x-zip-compressed'

UPGRADE_CONF = config.config.update

def check():
    
    update_checking = UPGRADE_CONF.update_checking.resolve(True)
    use_local       = UPGRADE_CONF.use_local      .resolve(False)

    if not update_checking:
        print('Update checking disabled')
        return None
        
    elif use_local:
        return LocalUpgradeContext()

    else:
        return _get_latest_version()


def _get_latest_version():

    url = f'{BASE_URL}/latest'

    try:
        r = requests.get(url, headers=HEADERS)
        r.raise_for_status()
    except:
        return None

    data = r.json()

    return RemoteUpgradeContext(data)

class UpgradeContext():

    def __init__(self):
        self.v_latest  = None
        self.v_current = utils.get_version()

    @property
    def is_upgradable(self):

        if self.v_current or self.v_latest is None:
            return False
        
        return version.parse(self.v_latest) > version.parse(self.v_current)
    
    @property
    def upgrade_dir(self):
        
        app_dir  = utils.APP_DIR
        v_latest = self.v_latest

        upgrade_dir = app_dir.parent / f'{app_dir.name}_{v_latest}'

        return upgrade_dir

class RemoteUpgradeContext(UpgradeContext):

    def __init__(self, data):
        super().__init__()

        self.v_latest = data['tag_name']
        self.data     = data

    @property
    def asset_url(self):
        
        assets     = self.data['assets']
        zip_assets = [a for a in assets if a['content_type'] == ZIP_FILE_TYPE]

        if len(zip_assets) == 0:
            return None

        zip_url = zip_assets[0]['url']

        return zip_url
    
    def download(self):
        try:
            url     = self.asset_url
            headers = {
                'Accept': 'application/octet-stream'
            }
        
            print('Downloading zip file:', url)
            r = requests.get(url, headers=headers)
            r.raise_for_status()

            zip_data = BytesIO()
            zip_data.write(r.content)

            return zip_data

        except:
            raise Exception('Unable to download the zip file.  Contact your local tech support.')
    
class LocalUpgradeContext(UpgradeContext):

    def __init__(self):
        super().__init__()

        conf = UPGRADE_CONF.local
        
        self.v_latest = conf.version.resolve()
        self.file     = conf.file.resolve()

    def download(self):
        zip_file = Path(self.file)

        zip_data = BytesIO()
        zip_data.write(zip_file.read_bytes())

        return zip_data

    @property
    def is_upgradable(self):
        return True