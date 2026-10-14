"""Checks a payload against its published schema, the way a producer's tests should before an event ships.

The schemas are closed: an unknown field is an error here. The generated models are open and ignore unknown
fields, so a consumer keeps working when a producer adds an optional field before the consumer has upgraded.
"""

import json
from functools import cache
from importlib import resources
from typing import Any

from jsonschema import Draft7Validator, FormatChecker


@cache
def document() -> dict:
    return json.loads(resources.files("goway_events").joinpath("schemas.json").read_text())


def problems(title: str, payload: Any) -> list[str]:
    definitions = document()["definitions"]
    if title not in definitions:
        raise KeyError(f"no schema titled {title!r}")
    validator = Draft7Validator({**document(), "$ref": f"#/definitions/{title}"}, format_checker=FormatChecker())
    return [f"{'/'.join(map(str, e.absolute_path)) or '(root)'}: {e.message}" for e in validator.iter_errors(payload)]


def check(title: str, payload: Any) -> None:
    found = problems(title, payload)
    if found:
        raise ValueError(f"{title} does not match its schema: " + "; ".join(found))


def check_event(subject: str, payload: Any) -> None:
    """For a producer: does what we are about to publish on this subject match the published schema?"""
    subjects = document()["x-subjects"]
    if subject not in subjects:
        raise KeyError(f"no schema for subject {subject!r}; add it to goway-event-schemas first")
    check(subjects[subject], payload)


def check_frame(side: str, payload: Any) -> None:
    """`side` is `client` for frames an app sends, `server` for frames goway-realtime sends."""
    check({"client": "ClientFrame", "server": "ServerFrame"}[side], payload)
