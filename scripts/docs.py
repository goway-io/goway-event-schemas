# /// script
# requires-python = ">=3.13"
# dependencies = [
#     "pyyaml>=6.0.3",
# ]
# ///
"""Write each AsyncAPI document as one self-contained JSON file for API reference viewers (Scalar on goway-edge).

Payload files are inlined because a viewer fetches a single URL, and every message gets the valid examples from
examples/valid, so the reference shows real frames and events instead of placeholders made up from the types.
"""

import json
import pathlib
import sys

import yaml

ROOT = pathlib.Path(__file__).resolve().parent.parent
DOCS = ("events", "realtime")
OUT = ROOT / "packages" / "python" / "src" / "goway_events" / "asyncapi"


def inline(node, base: pathlib.Path):
    if isinstance(node, list):
        return [inline(item, base) for item in node]
    if not isinstance(node, dict):
        return node
    ref = node.get("$ref")
    if isinstance(ref, str) and not ref.startswith("#"):
        path = (base / ref).resolve()
        schema = {key: value for key, value in json.loads(path.read_text()).items() if key != "$schema"}
        return inline(schema, path.parent)
    return {key: inline(value, base) for key, value in node.items()}


def label(stem: str) -> str:
    return stem.replace("-", " ").replace("_", " ").replace(".", " ").capitalize()


def example(name: str, path: pathlib.Path) -> dict:
    return {"name": name, "payload": json.loads(path.read_text())}


def examples_for(message: dict) -> list[dict]:
    """Events keep their examples under the payload title; frames under the union, named by frame type."""
    title = message["payload"].get("title", "")
    valid = ROOT / "examples" / "valid"
    if (valid / title).is_dir():
        return [example(label(path.stem), path) for path in sorted((valid / title).glob("*.json"))]
    frame = message.get("name", "")
    files = [
        path
        for union in ("ClientFrame", "ServerFrame")
        for path in sorted((valid / union).glob("*.json"))
        if path.stem.split(".", 1)[0] == frame
    ]
    return [
        example(label(path.stem.split(".", 1)[1]) if "." in path.stem else message.get("title", label(frame)), path)
        for path in files
    ]


def who(channel: dict) -> str:
    """The publisher and consumers live in x- fields that viewers do not show, so they are written into the text."""
    publisher = channel.get("x-publisher")
    if not publisher:
        return ""
    consumers = channel.get("x-consumers") or []
    heard = ", ".join(f"`{name}`" for name in consumers) if consumers else "no service yet"
    return f"Published by `{publisher}`; consumed by {heard}."


def build(name: str) -> dict:
    source = ROOT / "asyncapi" / f"{name}.yaml"
    document = inline(yaml.safe_load(source.read_text()), source.parent)
    channels = document.get("channels", {})
    for channel in channels.values():
        line = who(channel)
        if line:
            channel["description"] = f"{channel.get('description', '').rstrip()}\n\n{line}".strip()
    for operation in document.get("operations", {}).values():
        publisher = channels.get(operation["channel"]["$ref"].rsplit("/", 1)[-1], {}).get("x-publisher")
        if publisher and "title" not in operation:
            operation["title"] = f"{publisher} publishes"
    missing = []
    for key, message in document.get("components", {}).get("messages", {}).items():
        examples = examples_for(message)
        if not examples:
            missing.append(key)
        message["examples"] = examples
    if missing:
        sys.exit(f"{name}: no valid example for {', '.join(missing)}")
    return document


def main() -> None:
    OUT.mkdir(parents=True, exist_ok=True)
    for name in DOCS:
        document = build(name)
        (OUT / f"{name}.json").write_text(json.dumps(document, indent=1, ensure_ascii=False) + "\n")
        print(f"{name}: {len(document['components']['messages'])} messages")


if __name__ == "__main__":
    main()
