from io       import BytesIO
from pathlib  import Path

from PIL import Image, ImageDraw, ImageFont

STRETCH        = 250
PAD_PIXELS     = 100
TOP_BOX_HEIGHT = 60

FONT      = 'arial.ttf'
FONT_SIZE = 20

EXPORT_FORMAT = 'PNG'

def process_image(image_binary, image_context):
    
    image = Image.open(BytesIO(image_binary))

    if image_context.is_sunday:
        image = _stretch_image(image)
    
    image = _insert_padding(image)
    image = _insert_text(image, image_context.format_date())

    log_img(image)

    return _to_bytes(image)

def _stretch_image(image):

    width, height = image.size

    new_width = width + STRETCH
    new_size  = (new_width, height)

    resize = image.resize(new_size)

    return resize

def _insert_padding(image):
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

def _insert_text(image, text):

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

def _to_bytes(image):
    byte_handler = BytesIO()

    image.save(byte_handler, EXPORT_FORMAT)

    return byte_handler

def log_img(image):

    file = Path('./out/test.png')

    image.save(str(file), 'PNG')