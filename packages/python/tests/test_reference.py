"""The reference documents goway-edge serves: self-contained, and every message shows a real example."""

import json

import pytest

from goway_events import asyncapi, problems


@pytest.mark.parametrize("name", ["events", "realtime"])
def test_every_message_carries_an_example_that_matches_its_payload(name):
    document = asyncapi(name)
    assert "../schemas" not in json.dumps(document)
    for message in document["components"]["messages"].values():
        assert message["examples"]
        for example in message["examples"]:
            assert problems(message["payload"]["title"], example["payload"]) == []


def test_an_unknown_document_is_refused():
    with pytest.raises(KeyError):
        asyncapi("internal")
