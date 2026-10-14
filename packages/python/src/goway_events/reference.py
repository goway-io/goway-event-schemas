"""The AsyncAPI documents in one file each, with real examples, for reference viewers such as Scalar on goway-edge."""

import json
from functools import cache
from importlib import resources

DOCUMENTS = ("events", "realtime")


@cache
def asyncapi(name: str) -> dict:
    if name not in DOCUMENTS:
        raise KeyError(f"no AsyncAPI document named {name!r}")
    return json.loads(resources.files("goway_events").joinpath("asyncapi", f"{name}.json").read_text())
