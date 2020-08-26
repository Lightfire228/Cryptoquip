
from . import export
from . import extract
from . import menu
from . import image
from . import request

from . import utils

def run():
    image_contexts   = request.get_image_contexts()
    selected_context = menu.choose_image(image_contexts)

    pdf_binary   = request.download_pdf_binary(selected_context)
    image_binary = extract.extract_image(pdf_binary, selected_context)
    image_       = image.process_image(image_binary, selected_context)
    # image_file   = image.process_image(image_binary, selected_context)

    utils.log_img(image_)

    # image_file.close()