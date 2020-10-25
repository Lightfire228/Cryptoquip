from io       import BytesIO
from pathlib  import Path

import os
import tempfile
import zipfile

def compile_word_doc(image, image_context):

    tmp = tempfile.gettempdir()
    doc = tempfile.NamedTemporaryFile(delete=False, suffix='.docx') 

    template_folder = _get_word_template_folder(image_context)
    files           = [f for f in template_folder.glob('**/*') if f.is_file()]

    with _open_zip(doc) as zip, _open_image_handle(image) as img:

        for f in files:
            (src, dest) = _get_zip_src_dest(f, template_folder)
            
            zip.write(src, dest)
        
        zip.writestr('word/media/image1.png', img.getvalue())
    
    doc.close()
    return doc


def launch_word(word_file):
    os.system(f'start {word_file.name}')

def _get_word_template_folder(image_context):
    folder = 'sunday' if image_context.is_sunday else 'default'

    path = Path('./word_exploded') / folder

    return path

def _open_zip(file_):
    return zipfile.ZipFile(file_.name, 'w', zipfile.ZIP_DEFLATED, strict_timestamps=False)

def _open_image_handle(image):
    file_handle = BytesIO()

    image.save(file_handle, 'PNG')

    return file_handle

def _get_zip_src_dest(file_, parent_folder):
    src  = str(file_)
    dest = str(file_.relative_to(parent_folder))

    return (src, dest)
