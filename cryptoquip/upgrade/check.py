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

BASE_URL = 'https://api.github.com/repos/Lightfire228/Cryptoquip/releases'
HEADERS  = {
    'Accept': 'application/vnd.github.v3+json'
}

ZIP_FILE_TYPE = 'application/x-zip-compressed'

def get_latest_version():

    url = f'{BASE_URL}/latest'

    try:
        r = requests.get(url, headers=HEADERS)
        r.raise_for_status()
    except:
        return None

    data = r.json()

    return UpgradeContext(data)

def get_local_version():

    data = {
        'tag_name': 'v2.2.0',
        'local':    True,
        'assets':   [
            {
                'content_type': ZIP_FILE_TYPE,
                'url':          'C:\\Temp\\Cryptoquip.zip'
            }
        ],
    }

    return UpgradeContext(data)

class UpgradeContext():

    def __init__(self, data):
        self.v_latest  = data['tag_name']
        self.v_current = utils.get_version()

        self.is_local = data.get('local', False)

        self.data = data

    @property
    def is_upgradable(self):

        if self.v_current or self.v_latest is None:
            return False
        
        # return version.parse(self.v_latest) > version.parse(self.v_current)
        return True

    @property
    def asset_url(self):
        
        assets     = self.data['assets']
        zip_assets = [a for a in assets if a['content_type'] == ZIP_FILE_TYPE]

        if len(zip_assets) == 0:
            return None

        zip_url = zip_assets[0]['url']

        return zip_url
    
    @property
    def asset_headers(self):
        return {
            'Accept': 'application/octet-stream'
        }
    
    @property
    def upgrade_dir(self):
        
        app_dir  = utils.APP_DIR
        v_latest = self.v_latest

        upgrade_dir = app_dir.parent / f'{app_dir.name}_{v_latest}'

        return upgrade_dir
    