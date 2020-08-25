from io       import BytesIO
from datetime import datetime, timedelta
from pathlib  import Path
from types    import SimpleNamespace
from math     import ceil

import argparse
import os
import shutil
import sys
import tempfile
import zipfile

from PIL import Image, ImageDraw, ImageFont
from bs4 import BeautifulSoup

import requests

URL        = 'https://www.cecildaily.com/diversions/cryptoquip/'
PARSER     = 'parser.html'
USER_AGENT = 'Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:79.0) Gecko/20100101 Firefox/79.0'

DAY_STR_LEN = max([ 
    len(datetime.weekday(i))
    for i in range(7)
])

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

def get_image(urls, args):
    url = urls[args.days_ago]

    r = req(url)

    return r.content

def extract_date_text(cards, args):

    card = cards[args.days_ago]

    time = card.find('time')
    iso  = time.attrs['datetime']

    date = datetime.fromisoformat(iso)

    day    = date.strftime('%A')
    pretty = date.strftime('%x')


    text = f'{day} - {pretty}'

    return text

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

