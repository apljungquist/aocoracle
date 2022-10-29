#!/usr/bin/env python3
import functools
import hashlib
import itertools
import json
import logging
import os
import pathlib
import re
import time

import fire
import requests
from sprig import dictutils

logger = logging.getLogger(__name__)


class AnswerNotFoundError(Exception):
    ...


class Session:
    def __init__(
        self,
        session: str,
        cache_location: pathlib.Path = pathlib.Path(__file__).parents[1] / "data",
    ) -> None:
        self._cookie = session
        self._answers = {}
        self._cache_location = cache_location
        self._answers_location = cache_location / "answers.json"
        self._rate_limited_until = time.monotonic()

    def __enter__(self):
        with self._answers_location.open() as f:
            self._answers = dictutils.deflate(json.load(f))
        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        with self._answers_location.open("w") as f:
            json.dump(dictutils.inflate(self._answers), f, sort_keys=True, indent=2)

    @functools.cache
    def _get_text(self, url: str) -> str:
        delay = max(0.0, self._rate_limited_until - time.monotonic())
        logger.info("Downloading %s after %3.1f second delay", url, delay)
        time.sleep(delay)
        resp = requests.get(
            url,
            cookies={"session": self._cookie},
        )
        # Compromise between me being impatient and not wanting to hammer the server
        self._rate_limited_until = time.monotonic() + 10
        resp.raise_for_status()
        return resp.text

    @functools.cache
    def _input(self, year: int, day: int) -> str:
        return self._get_text(f"https://adventofcode.com/{year}/day/{day}/input")

    @functools.cache
    def _question(self, year: int, day: int) -> str:
        return self._get_text(f"https://adventofcode.com/{year}/day/{day}")

    @functools.cache
    def _user_fingerprint(self) -> str:
        return hashlib.sha1(self._input(2020, 2).encode("ascii")).hexdigest()[:10]

    def input(self, year: int, day: int) -> str:
        file_location = (
            self._cache_location
            / "inputs"
            / f"{year:04}"
            / f"{day:02}"
            / f"{self._user_fingerprint()}.txt"
        )
        if not file_location.exists():
            file_location.parent.mkdir(exist_ok=True, parents=True)
            file_location.write_text(self._input(year, day))
        return file_location.read_text()

    def question(self, year: int, day: int) -> str:
        file_location = (
            self._cache_location
            / "questions"
            / f"{year:04}"
            / f"{day:02}"
            / f"{self._user_fingerprint()}.html"
        )
        if not file_location.exists():
            file_location.parent.mkdir(exist_ok=True, parents=True)
            file_location.write_text(self._question(year, day))
        return file_location.read_text()

    def _answer(self, year: int, day: int, part: int) -> str:
        question = self.question(year, day)
        answers = {
            i: m[1]
            for i, m in enumerate(
                re.finditer(r"Your puzzle answer was <code>([^<]+)</code>", question), 1
            )
        }
        try:
            return answers[part]
        except KeyError as e:
            raise AnswerNotFoundError from e

    def answer(self, year: int, day: int, part: int) -> str:
        key = f"{year:04}/{day:02}/{part}/{self._user_fingerprint()}"
        if key not in self._answers:
            self._answers[key] = self._answer(year, day, part)
        return self._answers[key]


def main(session_cookie: str) -> None:
    """Scrape input-output pairs from the official website

    :param session_cookie: The value of the session cookie stored by the browser after
        authenticating to the the website.
    """
    with Session(session_cookie) as session:
        for year in itertools.count(2015):
            for day in range(1, 26):
                try:
                    session.input(year, day)
                except requests.HTTPError:
                    logger.info(
                        "Could not retrieve input for %s/%s/%s", year, day, part
                    )
                    return

                for part in [1, 2]:
                    try:
                        session.answer(year, day, part)
                    except AnswerNotFoundError:
                        logger.debug("No answer for found %s/%s/%s", year, day, part)


if __name__ == "__main__":
    logging.basicConfig(level=getattr(logging, os.environ.get("LEVEL", "WARNING")))
    fire.Fire(main)
