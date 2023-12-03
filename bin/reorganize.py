#!/usr/bin/env python3
import hashlib
import json
import logging
import os
import pathlib

import fire
from sprig import dictutils

logger = logging.getLogger(__name__)

PROJECT_ROOT = pathlib.Path(__file__).parents[1]


def is_manual_example(name: str) -> bool:
    return any(
        [
            name.startswith("example"),
            name == "easy",
        ]
    )


def main() -> None:
    old_data_path = PROJECT_ROOT / "data.bak"
    new_data_path = PROJECT_ROOT / "data"

    old_inputs_path = old_data_path / "inputs"
    old_answers_path = old_data_path / "answers.json"

    new_inputs_path = new_data_path / old_inputs_path.name
    new_answers_path = new_data_path / old_answers_path.name
    old_data_path.mkdir()
    new_inputs_path.rename(old_inputs_path)
    new_answers_path.rename(old_answers_path)

    answers = dictutils.deflate(json.loads((old_answers_path).read_text()))
    inputs = {
        path.relative_to(old_inputs_path).with_suffix("").__str__(): path.read_text()
        for path in old_inputs_path.glob("*/*/*.txt")
    }

    for k, v in answers.items():
        y, d, p, old_name = k.split("/")
        if is_manual_example(old_name):
            new_name = old_name.upper()
        else:
            new_name = hashlib.sha256(
                inputs[f"{y:04}/{d:02}/{old_name}"].encode()
            ).hexdigest()[:16]

        new_path = new_data_path / f"{y:04}/{d:02}/answers/{p:01}/{new_name}.txt"
        if new_path.exists():
            logger.warning("Duplicate answer: %s", new_path)
        new_path.parent.mkdir(parents=True, exist_ok=True)
        new_path.write_text(v)

    for k, v in sorted(inputs.items()):
        y, d, old_name = k.split("/")
        if is_manual_example(old_name):
            new_name = old_name.upper()
        else:
            new_name = hashlib.sha256(v.encode()).hexdigest()[:16]

        new_path = new_data_path / f"{y:04}/{d:02}/inputs/{new_name}.txt"
        if new_path.exists():
            logger.warning("Duplicate input: %s", new_path)
        new_path.parent.mkdir(parents=True, exist_ok=True)
        new_path.write_text(v)


if __name__ == "__main__":
    logging.basicConfig(level=getattr(logging, os.environ.get("LEVEL", "WARNING")))
    fire.Fire(main)
