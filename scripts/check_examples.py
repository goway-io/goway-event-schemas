# /// script
# requires-python = ">=3.13"
# dependencies = [
#     "jsonschema[format]>=4.26.0",
# ]
# ///
"""Every example under examples/valid/<Title> must pass that schema and every one under examples/invalid must fail.

An invalid example breaks exactly one rule, so it keeps proving that rule: one with two defects still fails when either
rule is lost. A frame union reports a wrong frame as one `oneOf` error, so there the count says less. Every event needs
a valid and an invalid example, and every frame type a valid example of its union.

No AsyncAPI tool checks message examples in a v3 document, so this is the check.
"""

import json
import pathlib
import sys

from jsonschema import Draft7Validator, FormatChecker

ROOT = pathlib.Path(__file__).resolve().parent.parent


def main() -> int:
    document = json.loads((ROOT / "build" / "schemas.json").read_text())
    wrong = checked = 0
    for expected in ("valid", "invalid"):
        for path in sorted((ROOT / "examples" / expected).rglob("*.json")):
            title = path.parent.name
            if title not in document["definitions"]:
                print(f"no schema titled {title} for {path.relative_to(ROOT)}")
                wrong += 1
                continue
            validator = Draft7Validator({**document, "$ref": f"#/definitions/{title}"}, format_checker=FormatChecker())
            errors = list(validator.iter_errors(json.loads(path.read_text())))
            checked += 1
            if bool(errors) == (expected == "valid"):
                wrong += 1
                why = errors[0].message[:120] if errors else "passed but should not"
                print(f"{expected:7} {path.relative_to(ROOT)}: {why}")
            elif len(errors) > 1:
                wrong += 1
                why = "; ".join(error.message[:60] for error in errors)
                print(f"{expected:7} {path.relative_to(ROOT)}: breaks {len(errors)} rules, keep it to one: {why}")
    for gap in uncovered():
        print(gap)
        wrong += 1
    print(f"{checked - wrong}/{checked} examples behave as expected")
    return 1 if wrong else 0


def uncovered() -> list[str]:
    """Every event needs a valid and an invalid example, and every frame type a valid example of its union."""
    valid = ROOT / "examples" / "valid"
    gaps = []
    for path in sorted((ROOT / "schemas" / "events").glob("*.json")):
        title = json.loads(path.read_text())["title"]
        for expected in ("valid", "invalid"):
            if not any((ROOT / "examples" / expected / title).glob("*.json")):
                gaps.append(f"no {expected} example for {title}")
    for side, union in (("client", "ClientFrame"), ("server", "ServerFrame")):
        shown = {json.loads(path.read_text())["type"] for path in (valid / union).glob("*.json")}
        for path in sorted((ROOT / "schemas" / "frames" / side).glob("*.json")):
            frame = json.loads(path.read_text())["properties"]["type"]["const"]
            if frame not in shown:
                gaps.append(f"no valid {union} example with type {frame}")
    return gaps


if __name__ == "__main__":
    sys.exit(main())
