#!/bin/bash


pyinstaller -y \
    --add-data "./config.default.json;." \
    --add-data "./version;."             \
main.py

cd dist/main

7z a -r ../Cryptoquip.zip .

cp ../Cryptoquip.zip /c/Temp/