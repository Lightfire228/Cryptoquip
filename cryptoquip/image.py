from io import BytesIO

from PIL import Image, ImageDraw, ImageFont

from . import utils

Dim = utils.Dim
Box = utils.Box

STRETCH = 0

PAD_PIXELS    =   75
TOP_BOX_Y     =   80
BORDER_PIXELS =    1
FOOTER_Y_SUN  = -191

ANS_BOTTOM_Y        = -104
ANS_HEADER_DIM      = Dim(101, 57)
ANS_PIXEL_THRESHOLD = 225
CLUE_PADDING        = 20

DATE_BOX = Box(0, 0, 175, 65)

WHITE = '#FFFFFF'

FONT      = 'arial.ttf'
FONT_SIZE = 40

EXPORT_FORMAT = 'PNG'



def process_image(image_binary, image_context):
    
    image = Image.open(BytesIO(image_binary))

    if image_context.is_sunday:
        image = _stretch_image(image)
        image = _crop_footer(image)
    else:
        image = _crop_answer(image)

    image = _hide_date(image)
    image = _insert_header_padding(image)
    image = _insert_text(image, image_context.format_date())

    # utils_dev.log_img(image, image_context.day_str)

    return image

def _stretch_image(image):

    width, height = image.size

    new_width = width + STRETCH
    new_size  = (new_width, height)

    resize = image.resize(new_size)

    return resize

def _crop_footer(image):
    width, height = image.size

    footer_top_y = (FOOTER_Y_SUN + height) % height

    top_box = Box(0, 0, width, footer_top_y)

    return image.crop(top_box)

def _hide_date(image):

    draw = ImageDraw.Draw(image)

    draw.rectangle(DATE_BOX, fill=WHITE)

    return image

def _insert_header_padding(image):
    width, height = image.size

    top_box              = Box(0, 0, width, TOP_BOX_Y)
    bottom_corner        = Dim(0, TOP_BOX_Y)
    bottom_box           = Box(0, TOP_BOX_Y, width, height)
    target_bottom_corner = Dim(0, TOP_BOX_Y + PAD_PIXELS)

    target_height = height + PAD_PIXELS
    target_dim    = Dim(width, target_height)

    target = Image.new(image.mode, target_dim, WHITE)

    top_region    = image.crop(top_box)
    bottom_region = image.crop(bottom_box)
    
    target.paste(top_region,    top_box)
    target.paste(bottom_region, target_bottom_corner)

    return target

def _crop_answer(image):
    width, height = image.size

    ans_bottom_y = (ANS_BOTTOM_Y + height) % height
    ans_bottom_h = height - ans_bottom_y

    ans_top_y = _scan_ans_header(image)

    if ans_top_y == None:
        print('Unable to crop nswer programmatically')
        return image

    target_h   = height - (ans_bottom_y - ans_top_y)
    target_dim = Dim(width, target_h + CLUE_PADDING)

    top_box    = Box(0,             0, width, ans_top_y)
    bottom_box = Box(0, ans_bottom_y, width, height)

    target_top    = top_box
    target_bottom = Box(
        0,     target_top.y2 + CLUE_PADDING,
        width, target_top.y2 + CLUE_PADDING + ans_bottom_h
    )

    target = Image.new(image.mode, target_dim, WHITE)

    top_region    = image.crop(top_box)
    bottom_region = image.crop(bottom_box)
    
    target.paste(top_region,    target_top)
    target.paste(bottom_region, target_bottom)

    return target


def _scan_ans_header(image):
    width, height = image.size

    ans_header_w, ans_header_h = ANS_HEADER_DIM

    ans_scan_h   = ANS_BOTTOM_Y + ans_header_h
    ans_bottom_h = (ANS_BOTTOM_Y + height) % height
    ans_top_h    = ans_bottom_h - ans_header_h 

    found_white = False
    for dy in reversed(range(ans_top_h)):
        ans_box = _dim_to_box(ANS_HEADER_DIM, (0, dy))

        ans_candidate = image.crop(ans_box)

        is_white = (
            p > ANS_PIXEL_THRESHOLD
            for p in _iter_pixels(ans_candidate)
        )

        # look for first large junk of white on left side of screen
        if not found_white and all(is_white):
            found_white = True
        
        # look for first line of black pixels after that
        elif found_white and not all(is_white):
            # height of answer found
            return dy +1

    return None

def _iter_pixels(image):

    w, h = image.size

    for dy in range(h):
        for dx in range(w):
            yield image.getpixel((dx, dy))

def _dim_to_box(dim, pos):
    x, y = pos
    w, h = dim

    return Box(x, y, x + w, y + h)

def _box_to_dim(box):

    x, y, dx, dy = box

    return Dim(dx - x, dy - y)


def _insert_text(image, text):

    draw = ImageDraw.Draw(image)

    font = ImageFont.truetype(FONT, FONT_SIZE)

    text_box     = font.getmask(text).getbbox()
    text_center  = text_box[2]   // 2
    image_center = image.size[0] // 2

    offset_width  = image_center - text_center
    offset_height = TOP_BOX_Y + 10
    offset_dim    = Dim(offset_width, offset_height)
    
    draw.text(offset_dim, text, font=font, fill='black')

    return image

# don't think this is needed, since image is on paper
def _add_border(image, color=WHITE):

    width, height = image.size
    border        = BORDER_PIXELS * 2

    dim = Dim(width + border, height + border)

    target = Image.new(image.mode, dim, color)

    region = image.crop(Box(0, 0, width, height))
    target.paste(region, Dim(BORDER_PIXELS, BORDER_PIXELS))

    return target
