#!/usr/bin/env bash

PIP_CONSTRAINT="$(pwd)/constraints.txt"
export PIP_CONSTRAINT

if [[ ! -e venv ]] ; then
    echo "Creating venv"
    python -m venv --prompt $(basename $(pwd)) venv
    source venv/bin/activate
    PIP_CONSTRAINT=constraints.txt pip install pip setuptools
else
    echo "Reusing venv"
    source venv/bin/activate
fi

export PATH="$(pwd)/bin:${PATH}"
