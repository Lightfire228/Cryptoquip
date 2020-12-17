
from pathlib import Path

from bs4 import BeautifulSoup
from PIL import ImageDraw

from .dirs import INSTALLED

_folder = Path('./out')

_folder.mkdir(exist_ok=True, parents=True)

def if_dev(func):

    def wrapper(*args, **kwargs):

        if not INSTALLED:
            return func(*args, **kwargs)
        else:
            return None
    
    return wrapper

@if_dev
def log_bs4(data, filename='test', wrap=False):

    file_ = _folder / f'{filename}.html' 

    out = BeautifulSoup('<html><body></body></html>', 'html.parser')

    if wrap:
        data = [data]

    out.html.body.extend(data)

    file_.write_text(out.prettify())

@if_dev
def log_img(image, filename='test'):

    file_ = Path('./out/') / f'{filename}.png'

    image.save(str(file_), 'PNG')

@if_dev
def log_pdf(pdf_binary, filename='test'):

    file_ = _folder / f'{filename}.pdf'

    file_.write_bytes(pdf_binary)

@if_dev
def draw_boxes(image, boxes):

    draw = ImageDraw.Draw(image)

    for box in boxes:
        draw.rectangle(box)
    
    return image