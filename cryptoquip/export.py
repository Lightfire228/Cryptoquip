from pathlib  import Path

import os
import tempfile
import zipfile

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
