
from . import export_word
from . import menu
from . import process_image
from . import request

def run():
    image_contexts    = request.get_image_contexts()
    # selected_contexts = menu.
    menu.choose_image(image_contexts)