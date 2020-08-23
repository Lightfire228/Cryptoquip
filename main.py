from io       import BytesIO
from datetime import datetime
from pathlib  import Path

import os
import shutil
import tempfile
import zipfile

from PIL import Image, ImageDraw, ImageFont
from bs4 import BeautifulSoup

import requests

URL        = 'https://www.cecildaily.com/diversions/cryptoquip/'
PARSER     = 'html.parser'
USER_AGENT = 'Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:79.0) Gecko/20100101 Firefox/79.0'

PAD_PIXELS     = 100
TOP_BOX_HEIGHT = 60

FONT      = 'arial.ttf'
FONT_SIZE = 20

STRETCH = 200

def main(args):

    day = 0

    base_page   = get_base_page()
    image_cards = filter_images(base_page)
    image_urls  = extract_image_urls(image_cards)
    date_text   = extract_date_text(image_cards, day)
    image       = get_image(image_urls, day)

    image = insert_padding(image)
    image = insert_text(image, date_text)

    compile_word_doc(image)

def parse_args():
    return None

#region get image

def get_base_page():
    text = req(URL).text

    root = BeautifulSoup(text, PARSER)

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

def stretch_image(image):

    width, height = image.size

    new_width = width + STRETCH
    new_size  = (new_width, height)

    resize = image.resize(new_size)

    return resize

def extract_date_text(cards, index):

    card = cards[index]

    time = card.find('time')
    iso  = time.attrs['datetime']

    date = datetime.fromisoformat(iso)

    day    = date.strftime('%A')
    pretty = date.strftime('%x')


    text = f'{day} - {pretty}'

    return text

#endregion

#region processing

def insert_padding(image):
    width, height = image.size

    top_box           = (0, 0, width, TOP_BOX_HEIGHT)
    bottom_corner     = (0, TOP_BOX_HEIGHT)
    bottom_box        = (0, TOP_BOX_HEIGHT, width, height)
    new_bottom_corner = (0, TOP_BOX_HEIGHT + PAD_PIXELS)

    new_height = height + PAD_PIXELS
    dim        = (width, new_height)
    white      = '#FFFFFF'

    target = Image.new(image.mode, dim, white)

    top_region    = image.crop(top_box)
    bottom_region = image.crop(bottom_box)
    
    target.paste(top_region,    top_box)
    target.paste(bottom_region, new_bottom_corner)

    return target

def insert_text(image, text):

    draw = ImageDraw.Draw(image)

    font = ImageFont.truetype(FONT, FONT_SIZE)

    text_box     = font.getmask(text).getbbox()
    text_center  = text_box[2]   // 2
    image_center = image.size[0] // 2

    offset_width  = image_center - text_center
    offset_height = TOP_BOX_HEIGHT + 10
    offset_box    = (offset_width, offset_height)
    
    draw.text(offset_box, text, font=font, fill='black')

    return image

#endregion

#region word

def compile_word_doc(image):

    tmp = tempfile.gettempdir()
    doc = tempfile.NamedTemporaryFile(delete=False, suffix='.docx') 

    image_file    = Path(tmp) / 'image.png'
    word_template = Path('./word_exploded')

    files = [f for f in word_template.glob('**/*') if f.is_file()]
    image.save(image_file, image.format)

    with zipfile.ZipFile(doc.name, 'w', zipfile.ZIP_DEFLATED, strict_timestamps=False) as zip:

        for f in files:
            name = Path('/'.join(f.parts[1:]))
            zip.write(str(f), str(name))

        zip.write(image_file, 'word/media/image1.png')


    doc.close()
    os.system(f'start {doc.name}')


#endregion

#region utils

def req(url):
    headers = {
        'User-Agent': USER_AGENT
    }

    r = requests.get(url, headers=headers)

    r.raise_for_status()

    return r

def log_bs4(data):

    from collections.abc import Iterable
    
    out = BeautifulSoup('<html><body></body></html>', PARSER)

    if not isinstance(data, Iterable):
        data = [data]

    out.html.body.extend(data)

    Path('./out/test.html').write_text(out.prettify())

def log_img(image):

    file = Path('./out/test.png')

    image.save(str(file), 'PNG')



#endregion

if __name__ == "__main__":
    args = parse_args()
    main(args)