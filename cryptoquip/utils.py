
from pathlib import Path

from bs4 import BeautifulSoup

def log_bs4(data, filename='test', wrap=False):

    file_ = Path('./out/') / f'{filename}.html' 

    out = BeautifulSoup('<html><body></body></html>', 'html.parser')

    if wrap:
        data = [data]

    out.html.body.extend(data)

    file_.write_text(out.prettify())


def log_img(image, filename='test'):

    file_ = Path('./out/') / f'{filename}.png'

    image.save(str(file_), 'PNG')

def log_pdf(pdf_binary, filename='test'):

    file_ = Path('./out/') / f'{filename}.pdf'

    file_.write_bytes(pdf_binary)