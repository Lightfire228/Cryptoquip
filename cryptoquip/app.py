import os

from .app import export
from .app import export_bitmap
from .app import extract
from .app import menu
from .app import image
from .app import request

from .upgrade import check, upgrade

def run():

    upgrade_context = check.get_local_version()
    upgrade.upgrade(upgrade_context)

    # image_contexts   = request.get_image_contexts()
    # selected_context = menu   .choose_image(image_contexts)

    # pdf_binary   = request      .download_pdf_binary(selected_context)
    # image_binary = extract      .extract_image   (pdf_binary,   selected_context)
    # image_       = image        .process_image   (image_binary, selected_context)
    # file_        = export_bitmap.export_bitmap   (image_,       selected_context)

    # os.system(f'start {file_.name}')