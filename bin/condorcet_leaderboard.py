#!/usr/bin/env python3
import json
import logging
import os
import pathlib
from pprint import pprint
from typing import Iterator

import condorcet
import fire

logger = logging.getLogger(__name__)

PROJECT_ROOT = pathlib.Path(__file__).parents[1]


def _names(jsn: dict) -> dict[str, str]:
    return {
        member_id: member_data["name"]
        for member_id, member_data in jsn["members"].items()
    }


def _candidates(jsn: dict) -> list[str]:
    return sorted(
        jsn["members"],
        key=lambda member_id: jsn["members"][member_id]["local_score"],
        reverse=True,
    )


def _votes(jsn: dict) -> Iterator[dict[str, int]]:
    for day in range(1, 26):
        day = str(day)
        for part in ("1", "2"):
            timestamps = {}
            for member_id, member_data in jsn["members"].items():
                try:
                    timestamps[member_id] = member_data["completion_day_level"][day][
                        part
                    ]["get_star_ts"]
                except KeyError:
                    pass
            ordered = sorted(timestamps.keys(), key=timestamps.__getitem__)
            vote = {member_id: i for i, member_id in enumerate(ordered)}
            for member_id in jsn["members"]:
                vote.setdefault(member_id, len(ordered))
            yield vote


def main(src: pathlib.Path | str) -> None:
    src = pathlib.Path(src)
    jsn = json.loads(src.read_text())
    evaluator = condorcet.CondorcetEvaluator(
        candidates=_candidates(jsn), votes=list(_votes(jsn))
    )
    names = _names(jsn)
    winners, rest_of_table = evaluator.get_n_winners(100)
    print("winners")
    for winner in winners:
        print(f"{names[winner]} ({winner})")
    print("")
    if len(rest_of_table) < 10:
        print("rest_of_table")
        pprint(
            {
                names[k]: {
                    "losses": [names[v] for v in vs["losses"]],
                    "wins": [names[v] for v in vs["wins"]],
                }
                for k, vs in rest_of_table.items()
            }
        )


if __name__ == "__main__":
    logging.basicConfig(level=getattr(logging, os.environ.get("LEVEL", "WARNING")))
    fire.Fire(main)
