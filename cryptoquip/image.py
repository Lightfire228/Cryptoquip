from io import BytesIO

from PIL import Image, ImageDraw, ImageFont

STRETCH = 400

PAD_PIXELS     = 75
TOP_BOX_HEIGHT = 75
BORDER_PIXELS  = 100

DATE_BOX = (0, 0, 150, 60)

WHITE = '#FFFFFF'

FONT      = 'arial.ttf'
FONT_SIZE = 40

EXPORT_FORMAT = 'PNG'


def process_image(image_binary, image_context):
    
    image = Image.open(BytesIO(image_binary))

    if image_context.is_sunday:
        image = _stretch_image(image)
    
    image = _hide_date(image)
    image = _insert_header_padding(image)
    image = _insert_text(image, image_context.format_date())
    # image = _add_border(image)

    return image

def _stretch_image(image):

    width, height = image.size

    new_width = width + STRETCH
    new_size  = (new_width, height)

    resize = image.resize(new_size)

    return resize

def _hide_date(image):

    draw = ImageDraw.Draw(image)

    draw.rectangle(DATE_BOX, fill=WHITE)

    return image

def _insert_header_padding(image):
    width, height = image.size

    top_box              = (0, 0, width, TOP_BOX_HEIGHT)
    bottom_corner        = (0, TOP_BOX_HEIGHT)
    bottom_box           = (0, TOP_BOX_HEIGHT, width, height)
    target_bottom_corner = (0, TOP_BOX_HEIGHT + PAD_PIXELS)

    target_height = height + PAD_PIXELS
    target_dim    = (width, target_height)

    target = Image.new(image.mode, target_dim, WHITE)

    top_region    = image.crop(top_box)
    bottom_region = image.crop(bottom_box)
    
    target.paste(top_region,    top_box)
    target.paste(bottom_region, target_bottom_corner)

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

# don't think this is needed, since image is on paper
def _add_border(image):

    width, height = image.size
    border        = BORDER_PIXELS * 2

    dim = (width + border, height + border)

    target = Image.new(image.mode, dim, WHITE)

    region = image.crop((0, 0, width, height))
    target.paste(region, (BORDER_PIXELS, BORDER_PIXELS))

    return target
