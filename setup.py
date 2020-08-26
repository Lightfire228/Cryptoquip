import setuptools
from pathlib import Path

requirements = Path('./requirements.txt').read_text()
version      = Path('./version').read_text().strip()

setuptools.setup(
    name='cryptoquip-lightfire228',
    version=version,
    packages=setuptools.find_packages(),
    include_package_data=True,
    python_requires='>=3.8',
    install_requires=requirements.split('\n')

)