from datetime import datetime, timedelta
from pathlib import Path

from bs4 import BeautifulSoup

import requests

URL        = 'https://www.cecildaily.com/diversions/cryptoquip/'
PARSER     = 'html.parser'
USER_AGENT = 'Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:79.0) Gecko/20100101 Firefox/79.0'

SUNDAY = 6

def get_image_contexts():
    html           = _download_page()
    image_cards    = _extract_image_cards(html)
    image_contexts = _to_image_contexts(image_cards)

    return image_contexts

def download_image_binary(image_context):

    url = image_context.url

    r = _req(url)

    return r.content

def _download_page():
    html = _req(URL).text

    return html

def _extract_image_cards(html):

    root = BeautifulSoup(html, PARSER)

    content = root.find('section', id='main-page-container')

    card_grids = content.find_all('div', class_='card-grid')

    cards = [
        c 
        for g in card_grids
        for c in g.find_all('div', class_='card-container')
    ]

    return cards

def _to_image_contexts(image_cards):
    
    def to_obj(i, card):
        url  = _extract_image_url(card)
        date = _extract_date(card)

        return ImageContext(i, url, date)

    return [
        to_obj(i, card)
        for i, card in enumerate(image_cards)
    ]


def _extract_image_url(card):

    img_tag  = card.find('img')
    src_attr = img_tag.attrs['data-srcset']

    ##
    # the data that comes back is of the form:
    # https://url/file.jpg?resize=200%2C186 200w,https://url/file.jpg?resize=300%2C279 300w, ...
    # so, while this is horrendous, splitting on '?' is extremely effective

    url = src_attr.split('?')[0]

    return url

def _extract_date(card):

    time = card.find('time')
    iso  = time.attrs['datetime']

    date = datetime.fromisoformat(iso)

    return date

def _req(url):
    headers = {
        'User-Agent': USER_AGENT
    }

    r = requests.get(url, headers=headers)

    r.raise_for_status()

    return r

class ImageContext():

    def __init__(self, ordinal, url, date):
        self.ordinal = ordinal
        self.url     = url
        self.date    = date
    
    @property
    def is_sunday(self):
        return self.date.weekday() == SUNDAY

    def format_date(self):
        day  = self.date.strftime('%A')
        date = self.date.strftime('%x')

        return f'{day} - {date}'