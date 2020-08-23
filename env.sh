#!/bin/bash

# https://stackoverflow.com/a/28776166/2716305
(return 0 2>/dev/null) && sourced=1 || sourced=0

if [[ sourced -eq 0 ]]; then
    echo "Source this file from shell, don't run it"
    exit 1
fi

echo "Sourcing python venv"
source ./.venv/Scripts/activate