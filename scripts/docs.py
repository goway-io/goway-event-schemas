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
        if "#" in ref:
            sys.exit(f"$ref {ref!r} from {base.relative_to(ROOT)} points inside a file; refer to the whole file")
        path = (base / ref).resolve()
        if not path.is_file():
            sys.exit(f"$ref {ref!r} from {base.relative_to(ROOT)}: no such file")
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


def publishers(channel: dict) -> list[str]:
    """`x-publisher` is one service, or a list when several services publish the same subject."""
    publisher = channel.get("x-publisher") or []
    return [publisher] if isinstance(publisher, str) else list(publisher)


def listed(names: list[str]) -> str:
    quoted = [f"`{name}`" for name in names]
    return quoted[0] if len(quoted) == 1 else f"{', '.join(quoted[:-1])} and {quoted[-1]}"


def who(channel: dict) -> str:
    """The publisher and consumers live in x- fields that viewers do not show, so they are written into the text."""
    by = publishers(channel)
    if not by:
        return ""
    consumers = channel.get("x-consumers") or []
    heard = listed(consumers) if consumers else "no service yet"
    return f"Published by {listed(by)}; consumed by {heard}."


def operation_title(by: list[str]) -> str:
    return f"{by[0]} publishes" if len(by) == 1 else f"{len(by)} services publish"


def build(name: str) -> dict:
    source = ROOT / "asyncapi" / f"{name}.yaml"
    document = inline(yaml.safe_load(source.read_text()), source.parent)
    channels = document.get("channels", {})
    for channel in channels.values():
        line = who(channel)
        if line:
            channel["description"] = f"{channel.get('description', '').rstrip()}\n\n{line}".strip()
    for operation in document.get("operations", {}).values():
        by = publishers(channels.get(operation["channel"]["$ref"].rsplit("/", 1)[-1], {}))
        if by and "title" not in operation:
            operation["title"] = operation_title(by)
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
