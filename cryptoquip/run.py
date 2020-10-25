import os

from .app import export
from .app import export
from .app import extract
from .app import menu
from .app import image
from .app import request

from .upgrade import check, upgrade

def run():

    _check_update()

    # image_contexts   = request.get_image_contexts()
    # selected_context = menu   .choose_image(image_contexts)

    # pdf_binary   = request.download_pdf_binary(selected_context)
    # image_binary = extract.extract_image      (pdf_binary,   selected_context)
    # image_       = image  .process_image      (image_binary, selected_context)
    # file_        = export .export_bitmap      (image_,       selected_context)

    # os.system(f'start {file_.name}')

def _check_update():
    upgrade_context = check.check()
    if upgrade_context and upgrade_context.is_upgradable:
        #TODO: ask for upgrade
        upgrade.upgrade(upgrade_context)