import os

from .app import export
from .app import extract
from .app import menu
from .app import image
from .app import request

from .update import check, update

def run():

    update.delete_old_versions()
    update_context = check.check()

    image_contexts   = request.get_image_contexts()
    selected_context = menu   .choose_image(image_contexts, update_context)

    if selected_context is update_context:
        update.update        (update_context)
        update.launch_new_app(update_context)
    
    else:
        pdf_binary   = request.download_pdf_binary(selected_context)
        image_binary = extract.extract_image      (pdf_binary,   selected_context)
        image_       = image  .process_image      (image_binary, selected_context)
        file_        = export .export_bitmap      (image_,       selected_context)

        os.system(f'start {file_.name}')

