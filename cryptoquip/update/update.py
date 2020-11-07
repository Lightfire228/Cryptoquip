from cryptoquip.dirs import APP_DIR
import os
import shutil
import subprocess
from typing import Mapping
import zipfile

from .. import dirs
from .. import utils

args = utils.args

MAIN_LINK_FILE   = dirs.INSTALL_DIR / 'main.exe'
DELETE_FLAG_FILE = '_delete'

def update(update_context):

    zip_data = update_context.download()

    _extract_zip  (update_context, zip_data)
    _set_link_file(update_context)
    _mark_to_delete()


def delete_old_versions():

    if not utils.INSTALLED:
        return

    if args.wait_for_pid:
        utils.log('Waiting for old app to exit..')

        try:
            os.waitpid(args.wait_for_pid, 0)
        except ChildProcessError:
            pass

        utils.log('Done\n')

    old_versions = [
        f 
        for f in dirs.INSTALL_DIR.iterdir() 
        if 
            f.is_dir()
            and (f / DELETE_FLAG_FILE).exists() 
            and not f == dirs.APP_DIR
    ]

    if len(old_versions):
        utils.log('Removing old app versions')

    for old in old_versions:
        shutil.rmtree(old)

def launch_new_app(update_context):

    pid = os.getpid()

    file = update_context.update_dir / MAIN_LINK_FILE.name
    
    os.system(f'start "" "{file}" -w {pid}')

def _extract_zip(update_context, zip_data):

    update_dir = update_context.update_dir

    update_dir.mkdir(exist_ok=True)


    utils.log('Extracting zip')
    with zipfile.ZipFile(zip_data) as zip:
        zip.extractall(update_dir)

def _set_link_file(update_context):
    MAIN_LINK_FILE.unlink(missing_ok=True)
    MAIN_LINK_FILE.symlink_to(update_context.update_dir / MAIN_LINK_FILE.name)

def _mark_to_delete():
    delete_file = dirs.APP_DIR / DELETE_FLAG_FILE

    delete_file.touch(exist_ok=True)
