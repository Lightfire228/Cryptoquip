#!/bin/bash


pyinstaller -y \
    --add-data "./word_exploded;word_exploded" \
    --add-data "./config.default.json;."       \
main.py

cd dist
cp -r main Cryptoquip/

7z a -r Cryptoquip.zip Cryptoquip/

rm -r Cryptoquip/