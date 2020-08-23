from io import BytesIO

from PIL import Image
from bs4 import BeautifulSoup

import requests

url        = 'https://www.cecildaily.com/diversions/cryptoquip/'
parser     = 'html.parser'
user_agent = 'Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:79.0) Gecko/20100101 Firefox/79.0'

def main(args):
    base_page   = get_base_page()
    image_cards = filter_images(base_page)
    image_urls  = extract_image_urls(image_cards)
    image       = get_image(image_urls, 0)

    process_image(image)

    log_img(image)


def parse_args():
    return None

#region get image

def get_base_page():
    text = req(url).text

    root = BeautifulSoup(text, parser)

    return root

def filter_images(page_root):

    content = page_root.find('section', id='main-page-container')

    card_grids = content.find_all('div', class_='card-grid')

    cards = [
        c 
        for g in card_grids
        for c in g.find_all('div', class_='card-container')
    ]

    return cards

def extract_image_urls(cards):

    img_tags  = [c.find('img') for c in cards]
    src_attrs = [i.attrs['data-srcset'] for i in img_tags]

    ##
    # the data that comes back is of the form:
    # https://url/file.jpg?resize=200%2C186 200w,https://url/file.jpg?resize=300%2C279 300w, ...
    # so, while this is horrendous, splitting on '?' is extremely effective

    urls = [s.split('?')[0] for s in src_attrs]

    return urls

def get_image(urls, index):
    url = urls[index]

    r = req(url)

    i = Image.open(BytesIO(r.content))

    return i

#endregion

#region processing

def process_image(image):
    pass

#endregion

#region utils

def req(url):
    headers = {
        'User-Agent': user_agent
    }

    r = requests.get(url, headers=headers)

    r.raise_for_status()

    return r

def log_bs4(data):

    from collections.abc import Iterable
    from pathlib         import Path
    
    out = BeautifulSoup('<html><body></body></html>', parser)

    if not isinstance(data, Iterable):
        data = [data]

    out.html.body.extend(data)

    Path('./out/test.html').write_text(out.prettify())

def log_img(image):
    from pathlib import Path

    file = Path('./out/test.png')

    image.save(str(file), 'PNG')



#endregion

if __name__ == "__main__":
    args = parse_args()
    main(args)