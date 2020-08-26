
from pathlib import Path

from bs4 import BeautifulSoup

def log_bs4(data):

    from collections.abc import Iterable
    
    out = BeautifulSoup('<html><body></body></html>', 'parser.html')

    if not isinstance(data, Iterable):
        data = [data]

    out.html.body.extend(data)

    Path('./out/test.html').write_text(out.prettify())


def log_img(image):

    file = Path('./out/test.png')

    image.save(str(file), 'PNG')