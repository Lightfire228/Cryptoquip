from io import BytesIO

import itertools
import time

from PIL import Image, ImageDraw, ImageFont, ImageOps

from .. import utils
from .. import utils_dev

Dim = utils.Dim
Box = utils.Box

WHITE = '#FFFFFF'

ANSWER_LEFT_THRESHOLD = 10

HEADER_PADDING = 75
CLUE_PADDING   = 50

TEXT_FONT      = 'arial.ttf'
TEXT_FONT_SIZE = 40
TEXT_PADDING   = 10

EXPORT_FORMAT = 'PNG'


def process_image(image_binary, image_context):
    
    image = Image.open(BytesIO(image_binary))

    utils_dev.log_img(image, 'test')

    boxes = _rectangulate(image)
    image = _hide_date(image, boxes, image_context)

    # After cropping, `boxes` is no longer accurate; however,
    # things are done bottom to top, so they don't need to be
    if image_context.is_sunday:
        image = _crop_footer(image, boxes)
    else:
        image = _crop_answer(image, boxes)

    image = _insert_header_padding(image, boxes)
    image = _insert_text(image, boxes, image_context.format_date())

    return image

def _rectangulate(image):

    width, _ = image.size

    rows = []

    line_ver = _merge_to_line(image)

    for y1, y2 in _find_black_line_segments(line_ver):

        row = []

        box_hor  = image.crop((0, y1, width, y2))
        line_hor = _merge_to_line(box_hor, vertical=False)

        for x1, x2 in _find_black_line_segments(line_hor):
            row.append(Box(x1, y1, x2, y2))
        
        rows.append(row)

    return rows

def _crop_footer(image, boxes):
    width, _ = image.size

    # second from last row to get rid of whitespace between footer and clue
    y_footer = boxes[-2][0].y2

    not_footer_box = Box(0, 0, width, y_footer) 

    return image.crop(not_footer_box)

def _hide_date(image, boxes, image_context):

    width, _ = image.size
    draw     = ImageDraw.Draw(image)
    row      = boxes[0]

    # I could filter by counting the number of boxes expected by date
    # but that could be subject to both date formatting and kerning
    w_quarter  = width // 4
    date_boxes = [ box for box in row if box.x1 < w_quarter ]

    for box in date_boxes:
        draw.rectangle(box, fill=WHITE)

    return image

def _insert_header_padding(image, boxes):
    width, height = image.size

    header_row = boxes[0]

    header_y2        = header_row[0].y2
    target_header_y2 = header_y2 + HEADER_PADDING

    target_height = height + HEADER_PADDING
    target        = Image.new(image.mode, (width, target_height), color=WHITE)

    header_box      = Box(0,                0, width, header_y2)
    rest_box        = Box(0,        header_y2, width, height)
    target_rest_box = Box(0, target_header_y2, width, target_height)

    header_crop = image.crop(header_box)
    rest_crop   = image.crop(rest_box)

    target.paste(header_crop, header_box)
    target.paste(rest_crop,   target_rest_box)

    return target

##                                                                    
#  Heights offset so as to crop out one and only one extra space      
#                                                                     
#  ############################################### _                  
#  #                  CRYPTOQUIP                 #  |                 
#  #  AA AAAAA AAAAAAAAA AAAAAAAAAAAAAAA AAAAAAA #  |                 
#  #  AAAA AAAAAAA AAAAAAAA AAAAAAAAAA AAAAAAAAA #  | puzzle_box      
#  #  AAA AAAAAAAAAAAAAAA AA AAAAAAAAAAA AAAAAAA #  |                 
#  #                                             #  |                 
#  #                                             # _| _               
#  #  Yesterday's Cryptoquip: YY YYYYYYYY YYYYY  #     |              
#  #  YYY YYYYYYYYYYY YYYYYYY YYYYYYYYY YYYYYYY  #     | answer_box   
#  #                                             #     |              
#  #                                             # _  _|              
#  #       Today's Crypto Clue: A equals Y       # _| clue_box        
#  ###############################################                    
#                                                                     
def _crop_answer(image, boxes):
    width, height = image.size

    # remove clue row
    rows = boxes[:-1]

    # reverse and find the last (now first) row that doesn't start on the left edge
    answer_row = [ r for r in rows[::-1] if r[0].x1 > ANSWER_LEFT_THRESHOLD ][0]
    clue_row   = boxes[-1]

    answer_y1 = answer_row[0].y1
    clue_y1   = clue_row  [0].y1

    # relative to image box
    puzzle_box = Box(0,         0, width, answer_y1)
    answer_box = Box(0, answer_y1, width, clue_y1)
    clue_box   = Box(0,   clue_y1, width, height)

    target_height      = height - (answer_box.y2 - answer_box.y1) + CLUE_PADDING
    target_clue_height = puzzle_box.y2 + CLUE_PADDING

    # relative to target box
    target_box        = Box(0,                  0, width, target_height)
    target_clue_box   = Box(0, target_clue_height, width, target_box.y2)
    target_puzzle_box = puzzle_box

    target = Image.new(image.mode, target_box.to_dim(), color=WHITE)

    puzzle_crop = image.crop(puzzle_box)
    clue_crop   = image.crop(clue_box)

    target.paste(puzzle_crop, target_puzzle_box)
    target.paste(clue_crop,   target_clue_box)

    return target

######################################################
# Utils

## 
# Abuses pillow to merge all black pixels into a single line by "folding" 
# the image onto itself repeatedly.  
def _merge_to_line(image, vertical=True):

    target = image

    if not vertical:
        target = target.transpose(Image.ROTATE_90)

    w, h = target.size

    while (w > 1):
        w_half = w // 2

        box1 = Box(0,      0, w_half, h)
        box2 = Box(w_half, 0, w,      h)

        half   = target.crop(box2)
        target = target.crop(box1)

        # uses the image itself as a mask, to transfer only black pixels
        # but must invert the mask because only white pixels are transferred
        mask = _invert_image(half)

        target.paste(half, mask=mask)

        w = w_half

    if not vertical:
        target = target.transpose(Image.ROTATE_270)

    return target

# no such built-in function for mode 1 images 
def _invert_image(image):
    invert = image.convert('L')
    invert = ImageOps.invert(invert)
    return invert.convert('1')

def _find_black_line_segments(line):

    target = line

    width, height = target.size

    if not (width != 1 or height != 1):
        raise Exception('Image must be a line (width or height must be 1)')

    if width != 1:
        target        = target.transpose(Image.ROTATE_270)
        width, height = target.size

    y1 = None
    was_white = target.getpixel((0, 0))

    if not was_white:
        y1 = 0

    for y in range(1, height):
        is_white = target.getpixel((0, y)) != 0

        if is_white ^ was_white:

            if was_white:
                y1 = y
            else:
                yield (y1, y)
                y1 = None

            was_white = is_white
    
    if y1 != None:
        yield (y1, height)

    

def _insert_text(image, boxes, text):

    width, _ = image.size

    draw = ImageDraw.Draw(image)
    font = ImageFont.truetype(TEXT_FONT, TEXT_FONT_SIZE)

    header_y2 = boxes[0][0].y2

    text_box     = font.getmask(text).getbbox()
    text_center  = text_box[2] // 2
    image_center = width       // 2

    offset_width  = image_center - text_center
    offset_height = header_y2    + TEXT_PADDING
    offset_dim    = Dim(offset_width, offset_height)
    
    draw.text(offset_dim, text, font=font, fill='black')

    return image
