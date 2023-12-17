#!/usr/bin/env python3
from __future__ import annotations

import copy
import datetime
import json
import logging
import os
from collections import defaultdict
from math import inf, isinf
from operator import itemgetter
from pathlib import Path
from typing import Iterator, Literal, Optional, TypedDict

import bar_chart_race as bcr
import fire
import pandas as pd

logger = logging.getLogger(__name__)

PROJECT_ROOT = Path(__file__).parents[1]


class Leaderboard(TypedDict):
    owner_id: int
    event: str
    members: dict[str, Member]


class Member(TypedDict):
    id: int
    global_score: int
    completion_day_level: dict[str, dict[str, dict[str, int]]]
    name: str
    stars: int
    last_star_ts: int
    local_score: int


def _member_at(member: Member, ts: int) -> Member:
    """Return member as it was at timestamp ts

    Note that the following attributes are not updated and will be incorrect:
    * global_score
    * local_score
    """

    member = copy.deepcopy(member)
    completion_day_level = member["completion_day_level"]
    for day in list(completion_day_level):
        for part in list(completion_day_level[day]):
            get_star_ts = completion_day_level[day][part]["get_star_ts"]
            if get_star_ts > ts:
                del completion_day_level[day][part]

        if completion_day_level[day] == {}:
            del completion_day_level[day]

    timestamps = [
        part["get_star_ts"]
        for day in member["completion_day_level"].values()
        for part in day.values()
    ]
    member["stars"] = len(timestamps)
    member["last_star_ts"] = max(timestamps, default=0)

    return member


def _leaderboard_at(final: Leaderboard, ts: int) -> Leaderboard:
    """Return leaderboard as it was at timestamp ts

    Note that global_score is not updated and will be incorrect.
    """
    snapshot = copy.copy(final)
    snapshot["members"] = {
        member_id: _member_at(member, ts)
        for member_id, member in final["members"].items()
    }
    _set_local_scores(snapshot)
    return snapshot


def _set_local_scores(leaderboard: Leaderboard) -> None:
    aggregate: defaultdict[int, int] = defaultdict(int)
    for day in range(1, 26):
        for part in ("1", "2"):
            timestamps = {
                member["id"]: member["completion_day_level"]
                .get(str(day), {})
                .get(part, {})
                .get("get_star_ts", inf)
                for member in leaderboard["members"].values()
            }
            marginal = {
                member_id: (0 if isinf(ts) else score)
                for score, (member_id, ts) in enumerate(
                    sorted(timestamps.items(), key=itemgetter(1), reverse=True), start=1
                )
            }
            for member_id, score in marginal.items():
                aggregate[member_id] += score
    for member in leaderboard["members"].values():
        member["local_score"] = aggregate[member["id"]]


def _alias(member_id: int, name: Optional[str], variant: int) -> str:
    if name is None:
        return str(member_id)

    parts = []
    for part in name.split():
        variant += 1
        length = min(len(part), variant)
        variant -= length

        parts.append(part[:length].title())
        if length < len(part):
            parts.append(".")
        else:
            parts.append(" ")

    return "".join(parts)


def _unique_aliases(leaderboard: Leaderboard) -> dict[int, str]:
    names = {member["id"]: member["name"] for member in leaderboard["members"].values()}
    aliases = {}
    used = set()
    for member_id, name in names.items():
        for i in range(10):
            alias = _alias(member_id, name, i)
            if alias in used:
                continue
            used.add(alias)
            aliases[member_id] = alias
            break
        else:
            raise RuntimeError(
                f"Could not find unique alias for '{name}' ({member_id})"
            )
    return aliases


def _daily_scores(
    final: Leaderboard, ordering: Literal["stars", "local_score"]
) -> Iterator[dict[str, int]]:
    """Yield the score of each member at the end of each day"""
    year = int(final["event"])
    last_day = max(
        int(day)
        for member in final["members"].values()
        for day in member["completion_day_level"].keys()
    )
    for day in range(1, last_day + 1):
        cutoff = int(
            datetime.datetime(
                year, 12, day + 1, hour=5, tzinfo=datetime.timezone.utc
            ).timestamp()
        )
        snapshot = _leaderboard_at(final, cutoff)
        for member in snapshot["members"].values():
            yield {
                "day": day,
                "member_id": member["id"],
                "score": member[ordering],
            }


def bar_chart_race(
    src: Path | str,
    dst: Optional[Path | str] = None,
    max_bars: Optional[int] = None,
    ordering: Literal["local_score", "stars"] = "local_score",
) -> None:
    """Animate leaderboard as it changes day by day"""
    src = Path(src)
    if dst is None:
        dst = src.with_suffix(".gif")
        writer = "pillow"  # imagemagick is not working on my machine
    else:
        dst = Path(dst)
        writer = None

    if ordering not in ("stars", "local_score"):
        raise ValueError(
            f"ordering must be one of 'stars' or 'local_score', not '{ordering}'"
        )

    final = json.loads(src.read_text())
    aliases = _unique_aliases(final)

    df = (
        pd.DataFrame.from_records(_daily_scores(final, ordering))
        .pivot(index="day", columns="member_id", values="score")
        .rename(columns=aliases)
    )

    if max_bars is not None:
        max_bars = min(max_bars, len(df.columns))

    period_length = 2000
    bcr.bar_chart_race(
        df=df,
        filename=str(dst),
        n_bars=max_bars,
        steps_per_period=period_length // 1000 * 40,
        period_length=period_length,
        period_fmt="{x:2.0f}",
        period_label={"x": 0.99, "y": 0.03, "ha": "right", "va": "center"},
        figsize=(6, 6),
        title=ordering.replace("_", " ").title(),
        writer=writer,
    )


if __name__ == "__main__":
    logging.basicConfig(level=getattr(logging, os.environ.get("LEVEL", "WARNING")))
    fire.Fire({f.__name__: f for f in [bar_chart_race]})
