from pathlib import Path

import shutil
import zipfile


def export():
    folder = Path('./word_exploded')

    default = folder / 'default'
    sunday  = folder / 'sunday'

    shutil.rmtree(default)
    shutil.rmtree(sunday)

    default.mkdir()
    sunday .mkdir()

    _extract('default.docx', default)
    _extract('sunday.docx',  sunday)

    _delete_image(default)
    _delete_image(sunday)
    
def _extract(source, dest):

    with zipfile.ZipFile(source, 'r') as zip:
        zip.extractall(str(dest))

def _delete_image(dest):
    image = dest / 'word/media/image1.png'

    image.unlink()

if __name__ == "__main__":
    export()