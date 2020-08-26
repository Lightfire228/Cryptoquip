
from . import export_word
from . import menu
from . import process_image
from . import request

def run():
    image_contexts   = request.get_image_contexts()
    selected_context = menu.choose_image(image_contexts)

    image_binary = request.download_image_binary(selected_context)
    image_file   = process_image.process_image(image_binary, selected_context)


    image_file.close()