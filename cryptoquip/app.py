import os

from . import export
from . import export_bitmap
from . import extract
from . import menu
from . import image
from . import request

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