import setuptools
from pathlib import Path

requirements = Path('./requirements.txt').read_text()

setuptools.setup(
    name='cryptoquip-lightfire228',
    version='1.0.0',
    packages=setuptools.find_packages(),
    include_package_data=True,
    python_requires='>=3.8',
    install_requires=requirements.split('\n')

)