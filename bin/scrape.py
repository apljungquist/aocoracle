#!/usr/bin/env python3
import datetime
import functools
import hashlib
import itertools
import json
import logging
import pathlib
import random
import re
import time
from typing import Any, Optional

import env_logger
import fire
import more_itertools
import requests
from typing_extensions import Self

logger = logging.getLogger(__name__)


class AnswerNotFoundError(Exception):
    ...


def _hexdigest(text: str) -> str:
    return hashlib.sha256(text.encode()).hexdigest()[:16]


class Session:
    def __init__(
        self,
        session: str,
        cache_location: pathlib.Path = pathlib.Path(__file__).parents[1] / "cache",
        data_location: pathlib.Path = pathlib.Path(__file__).parents[1] / "data",
    ) -> None:
        self._cookie = session
        self._cache_location = cache_location
        self._data_location = data_location
        self._rate_limited_until = time.monotonic()
        self.rng = random.Random(0)

    @property
    def index_location(self) -> pathlib.Path:
        return self._cache_location / f"{self.user_id()}.json"

    def __enter__(self) -> Self:
        return self

    def __exit__(self, exc_type: Any, exc_val: Any, exc_tb: Any) -> None:
        pass

    def _download(self, path: str, user_id: Optional[int], suffix: str) -> pathlib.Path:
        if user_id is None:
            preliminary: Optional[pathlib.Path] = None
        else:
            stem = path.replace("-", "--").replace("/", "-")
            preliminary = (
                self._cache_location / "user" / str(user_id) / stem
            ).with_suffix(suffix)

        if preliminary is not None and preliminary.exists():
            logger.debug("Using cache for %s", preliminary)
            return preliminary

        delay = max(0.0, self._rate_limited_until - time.monotonic())
        logger.info("Downloading %s after %3.1f second delay", path, delay)
        time.sleep(delay)
        resp = requests.get(
            "https://adventofcode.com/" + path,
            cookies={"session": self._cookie},
        )
        # Compromise between me being impatient and not wanting to hammer the server
        self._rate_limited_until = time.monotonic() + self.rng.gauss(10, 2)
        resp.raise_for_status()
        content = resp.text

        if preliminary is None:
            stem = _hexdigest(content)
            final = (self._cache_location / "other" / stem).with_suffix(suffix)
        else:
            final = preliminary

        logger.debug("Populating cache for %s", final)
        final.parent.mkdir(exist_ok=True, parents=True)
        final.write_text(content)
        return final

    def input_path(self, year: int, day: int, stem: str) -> pathlib.Path:
        return (
            self._data_location / f"{year:04}" / f"{day:02}" / "inputs" / f"{stem}.txt"
        )

    def input(self, year: int, day: int) -> str:
        cache_path = self._download(f"{year}/day/{day}/input", self.user_id(), ".txt")
        content = cache_path.read_text()
        stem = _hexdigest(content)
        data_path = self.input_path(year, day, stem)
        if data_path.exists():
            logger.debug("Reusing input %s", data_path)
        else:
            logger.debug("Creating input %s", data_path)
            data_path.parent.mkdir(parents=True, exist_ok=True)
            data_path.write_text(content)
        return data_path.read_text()

    @staticmethod
    def parsed_answer(question: str, part: int) -> str:
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

    def answer_path(self, year: int, day: int, part: int, stem: str) -> pathlib.Path:
        return (
            self._data_location
            / f"{year:04}"
            / f"{day:02}"
            / "answers"
            / f"{part:1}"
            / f"{stem}.txt"
        )

    def answer(self, year: int, day: int, part: int) -> str:
        cache_path = self._download(f"{year}/day/{day}", self.user_id(), ".html")
        content = cache_path.read_text()
        stem = _hexdigest(self.input(year, day))
        data_path = self.answer_path(year, day, part, stem)
        if data_path.exists():
            logger.debug("Reusing answer %s", data_path)
        else:
            logger.debug("Creating answer %s", data_path)
            data_path.parent.mkdir(parents=True, exist_ok=True)
            data_path.write_text(self.parsed_answer(content, part))
        return content

    @staticmethod
    def parsed_user_id(page: str) -> int:
        return int(
            more_itertools.one(re.finditer(r"\(anonymous user #(\d+)\)", page))[1]
        )

    @functools.cache
    def user_id(self) -> int:
        cache_path = self._download("settings", None, ".html")
        content = cache_path.read_text()
        return self.parsed_user_id(content)

    def create_answer(self, year: int, day: int, part: int, stem: str) -> None:
        path = self.answer_path(year, day, part, stem)
        if path.exists():
            logger.warning("Answer already exists %s", path)
        else:
            path.parent.mkdir(parents=True, exist_ok=True)
            path.touch()
            logger.debug("Creating empty answer %s", path)

    def create_input(self, year: int, day: int, stem: str, content: str = "") -> None:
        path = self.input_path(year, day, stem)
        path.parent.mkdir(parents=True, exist_ok=True)
        if path.exists():
            logger.warning("Input already exists %s", path)
        else:
            logger.debug("Creating empty input %s", path)
            path.write_text(content)


def _scrape_answers(session: Session) -> None:
    logger.info(f"Scraping {session.user_id()} for answers")
    for year in itertools.count(2015):
        for day in range(1, 26):
            for part in [1, 2]:
                try:
                    session.answer(year, day, part)
                except AnswerNotFoundError:
                    logger.debug("No answer for found %s/%s/%s", year, day, part)
                except requests.HTTPError:
                    logger.debug(
                        "Could not retrieve question for %s/%s/%s", year, day, part
                    )
                    return


def _scrape_inputs(session: Session) -> None:
    logger.info(f"Scraping {session.user_id()} for inputs")
    for year in itertools.count(2015):
        for day in range(1, 26):
            try:
                session.input(year, day)
            except requests.HTTPError:
                logger.info("Could not retrieve input for %s/%s", year, day)
                return


def _scrape_today(session: Session) -> None:
    now = datetime.datetime.now()
    year, day = now.year, now.day

    session.create_input(year, day, "EXAMPLE")
    session.create_input(year, day, "INPUT", session.input(year, day))
    for part in [1, 2]:
        session.create_answer(year, day, part, "EXAMPLE")
        session.create_answer(year, day, part, "INPUT")


SAVED_COOKIES_PATH = pathlib.Path(__file__).with_suffix(".sessions.json")


def _read_cookies() -> dict[int, str]:
    with SAVED_COOKIES_PATH.open() as f:
        return {int(k): v for k, v in json.load(f).items()}


def _write_cookies(cookies: dict[int, str]) -> None:
    with SAVED_COOKIES_PATH.open("w") as f:
        json.dump(cookies, f, indent=4, sort_keys=True)


class CLI:
    @staticmethod
    def add_session(session_cookie: str) -> None:
        try:
            cookies = _read_cookies()
        except FileNotFoundError:
            cookies = {}
        with Session(session_cookie) as session:
            cookies[session.user_id()] = session_cookie
        _write_cookies(cookies)

    @staticmethod
    def scrape_answers() -> None:
        for fingerprint, cookie in _read_cookies().items():
            with Session(session=cookie) as session:
                _scrape_answers(session)

    @staticmethod
    def scrape_inputs() -> None:
        for fingerprint, cookie in _read_cookies().items():
            with Session(session=cookie) as session:
                _scrape_inputs(session)

    @staticmethod
    def today() -> None:
        for fingerprint, cookie in _read_cookies().items():
            with Session(session=cookie) as session:
                _scrape_today(session)


if __name__ == "__main__":
    # logging.basicConfig(level=getattr(logging, os.environ.get("LEVEL", "WARNING")))
    env_logger.configure()
    fire.Fire(CLI)
