#!/bin/bash


pyinstaller -y \
    --add-data "./word_exploded;word_exploded" \
    --add-data "./config.default.json;."       \
    --add-data "./version;."                   \
main.py

cd dist/main

7z a -r ../Cryptoquip.zip .