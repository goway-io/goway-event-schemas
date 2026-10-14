"""Every example of the schema repo, read through the generated models and the schema check."""

import json
import pathlib

import pytest
from pydantic import ValidationError

import goway_events
from goway_events import check, check_event, problems

EXAMPLES = pathlib.Path(__file__).resolve().parents[3] / "examples"
VALID = sorted((EXAMPLES / "valid").rglob("*.json"))
INVALID = sorted((EXAMPLES / "invalid").rglob("*.json"))


def model_for(path: pathlib.Path):
    return getattr(goway_events, path.parent.name)


@pytest.mark.parametrize("path", VALID, ids=lambda p: f"{p.parent.name}/{p.stem}")
def test_a_valid_example_reads_and_what_python_writes_back_still_matches_the_schema(path):
    payload = json.loads(path.read_text())
    model = model_for(path).model_validate(payload)
    written = model.model_dump(mode="json", exclude_unset=True)

    assert problems(path.parent.name, payload) == []
    assert problems(path.parent.name, written) == []
    assert model_for(path).model_validate(written) == model


@pytest.mark.parametrize("path", INVALID, ids=lambda p: f"{p.parent.name}/{p.stem}")
def test_an_invalid_example_is_refused_by_the_model_and_the_schema(path):
    payload = json.loads(path.read_text())

    with pytest.raises(ValidationError):
        model_for(path).model_validate(payload)
    assert problems(path.parent.name, payload)


def test_a_consumer_ignores_a_field_it_does_not_know_yet_while_a_producer_is_refused():
    payload = json.loads((EXAMPLES / "valid" / "BookingCancelled" / "basic.json").read_text())
    newer = {**payload, "refund": {"amount": 1000, "currency": "UZS"}}

    assert goway_events.BookingCancelled.model_validate(newer).booking_id
    with pytest.raises(ValueError, match="refund"):
        check_event("booking.cancelled", newer)


def test_a_frame_union_picks_the_frame_by_type():
    payload = json.loads((EXAMPLES / "valid" / "ServerFrame" / "offer_result.won.json").read_text())
    frame = goway_events.ServerFrame.model_validate(payload).root

    assert isinstance(frame, goway_events.OfferResultFrame) and frame.data.outcome is goway_events.OfferOutcome.WON


def test_a_subject_without_a_schema_is_named():
    with pytest.raises(KeyError, match="add it to goway-event-schemas"):
        check_event("booking.teleported", {})
    with pytest.raises(KeyError):
        check("Teleport", {})
