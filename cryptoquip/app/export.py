import tempfile

from io          import BytesIO
from collections import namedtuple

from PIL import Image, ImageDraw, ImageFont


from .. import utils

Dim = utils.Dim
Box = utils.Box

WHITE = '#FFFFFF'

EXPORT_FORMAT = 'PNG'

# inches
LETTER_DIM_IN = Dim(8.5, 11)
MARGIN_IN    = 1
HEIGHT_IN     = 3
WIDTH_SUN_IN  = LETTER_DIM_IN.x1 - MARGIN_IN *2

def export_bitmap(image, image_context):

    target          = _convert_to_letter_size(image, image_context)
    tmp_file_handle = _export_to_tmp(target)

    return tmp_file_handle

def _export_to_tmp(image):
    tmp = tempfile.gettempdir()
    png = tempfile.NamedTemporaryFile(delete=False, suffix='.png') 

    image.save(png, 'PNG')

    png.close()
    return png

def _convert_to_letter_size(image, image_context):
    px_scale_factor = _letter_to_pixel_scale_factor(image, image_context.is_sunday)
    target_dim_px   = _scale_dim(LETTER_DIM_IN, px_scale_factor)

    target = Image.new(image.mode, target_dim_px, WHITE)

    target = _paste_corners(target, image, px_scale_factor)

    return target


def _letter_to_pixel_scale_factor(image, is_sunday):

    width, height   = image.size
    pixels_per_inch = None

    return (
        width  / WIDTH_SUN_IN if is_sunday else
        height / HEIGHT_IN
    )


def _scale_dim(dim, scale):
    return Dim(
        int(dim[0] * scale),
        int(dim[1] * scale)
    )

def _paste_corners(target, image, px_scale_factor):
    target_w, target_h = target.size
    image_w,  image_h  = image .size

    margin_px = int(MARGIN_IN * px_scale_factor)

    top_box = Box(
        margin_px,           margin_px, 
        margin_px + image_w, margin_px + image_h
    )
    bottom_box = Box(
        margin_px,           target_h - margin_px - image_h,
        margin_px + image_w, target_h - margin_px
    )

    target.paste(image, top_box)
    target.paste(image, bottom_box)

    return target

