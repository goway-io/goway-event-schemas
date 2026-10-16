"""Refuse a change that breaks a consumer of the contract, measured against the latest `v*` tag.

Events and server frames are read by consumers, so a change breaks when a consumer could no longer read what is sent:
a file or a property disappears, a required property turns optional, a property takes a type or an enum value it did
not before. Client frames run the other way, the server reads what an app sends, so there a change breaks when an app
built on the tag would be refused: a new required property, a property removed, a type or an enum value dropped. A new
optional property is never a break: the generated types are open (see bundle.py). A renamed `title` is, on both sides,
because it renames the generated type every consumer's code is written against.

A break can be meant. A commit since the tag says so the conventional way, `!` before the colon of its subject
(`feat!: ...`), and a new major tag follows it. CI needs the full history and tags for this; without a tag there is
nothing to compare, and the check passes.
"""

import json
import pathlib
import posixpath
import subprocess
import sys

ROOT = pathlib.Path(__file__).resolve().parent.parent
READ_BY_APPS = ("schemas/events", "schemas/frames/server")
READ_BY_SERVER = ("schemas/frames/client",)


def git(*args: str) -> str:
    return subprocess.run(["git", *args], cwd=ROOT, capture_output=True, text=True, check=True).stdout.strip()


def latest_tag() -> str | None:
    try:
        return git("describe", "--tags", "--abbrev=0", "--match", "v*")
    except subprocess.CalledProcessError:
        return None


def load(tag: str | None, path: str) -> dict:
    """One schema file as it was at `tag`, or as it is on disk when `tag` is None."""
    return json.loads((ROOT / path).read_text() if tag is None else git("show", f"{tag}:{path}"))


def names_at(tag: str | None, folder: str) -> set[str]:
    if tag is None:
        return {path.name for path in (ROOT / folder).glob("*.json")}
    return {name for name in git("ls-tree", "--name-only", f"{tag}:{folder}").splitlines() if name.endswith(".json")}


def resolve(tag: str | None, node: dict, path: str) -> tuple[dict, str]:
    """A `$ref` names a whole file relative to the one holding it; follow it to the schema it stands for."""
    while "$ref" in node:
        path = posixpath.normpath(posixpath.join(posixpath.dirname(path), node["$ref"]))
        node = load(tag, path)
    return node, path


class Shape:
    """One schema with its `anyOf` branches folded together: a nullable enum is the enum plus `null`."""

    def __init__(self, node: dict):
        branches = node.get("anyOf", [node])
        self.title = node.get("title") or next((b["title"] for b in branches if "title" in b), None)
        self.types: set[str] = set()
        self.values: set[str] | None = None
        self.properties: dict[str, dict] = {}
        self.required: set[str] = set()
        self.items: dict | None = None
        for branch in branches:
            kind = branch.get("type", "any")
            self.types |= set(kind) if isinstance(kind, list) else {kind}
            if "enum" in branch or "const" in branch:
                values = branch.get("enum", [branch.get("const")])
                self.values = (self.values or set()) | {json.dumps(value) for value in values}
            self.properties |= branch.get("properties", {})
            self.required |= set(branch.get("required", []))
            self.items = branch.get("items", self.items)


class Comparison:
    """`old` nodes come from the tag and `new` from the disk; each carries the file it sits in, for its `$ref`s."""

    def __init__(self, tag: str, read_by_apps: bool):
        self.tag = tag
        self.read_by_apps = read_by_apps
        self.problems: list[str] = []

    def walk(self, old_node: dict, old_file: str, new_node: dict, new_file: str, where: str) -> None:
        old_node, old_file = resolve(self.tag, old_node, old_file)
        new_node, new_file = resolve(None, new_node, new_file)
        old, new = Shape(old_node), Shape(new_node)
        if old.title and new.title and old.title != new.title:
            self.problems.append(f"{where}: type {old.title} renamed to {new.title}")
        if "any" not in old.types | new.types:
            if self.read_by_apps and new.types - old.types:
                self.problems.append(f"{where}: now also {sorted(new.types - old.types)}")
            if not self.read_by_apps and old.types - new.types:
                self.problems.append(f"{where}: no longer {sorted(old.types - new.types)}")
        if old.values is not None and new.values is not None:
            if self.read_by_apps and new.values - old.values:
                self.problems.append(f"{where}: new values {', '.join(sorted(new.values - old.values))}")
            if not self.read_by_apps and old.values - new.values:
                self.problems.append(f"{where}: values dropped {', '.join(sorted(old.values - new.values))}")
        for name in sorted(old.properties.keys() - new.properties.keys()):
            self.problems.append(f"{where}.{name}: removed")
        if self.read_by_apps:
            for name in sorted((old.required - new.required) & new.properties.keys()):
                self.problems.append(f"{where}.{name}: no longer always sent")
        else:
            for name in sorted(new.required - old.required):
                self.problems.append(f"{where}.{name}: newly required")
        for name in sorted(old.properties.keys() & new.properties.keys()):
            self.walk(old.properties[name], old_file, new.properties[name], new_file, f"{where}.{name}")
        if old.items is not None and new.items is not None:
            self.walk(old.items, old_file, new.items, new_file, f"{where}[]")


def breaks_since(tag: str) -> list[str]:
    problems: list[str] = []
    for folders, read_by_apps in ((READ_BY_APPS, True), (READ_BY_SERVER, False)):
        for folder in folders:
            now = names_at(None, folder)
            for name in sorted(names_at(tag, folder)):
                path = f"{folder}/{name}"
                if name not in now:
                    problems.append(f"{path}: removed")
                    continue
                comparison = Comparison(tag, read_by_apps)
                comparison.walk(load(tag, path), path, load(None, path), path, path)
                problems += comparison.problems
    return problems


def main() -> int:
    tag = latest_tag()
    if tag is None:
        print("no v* tag yet, nothing to compare against")
        return 0
    problems = breaks_since(tag)
    if not problems:
        print(f"no breaking change since {tag}")
        return 0
    print(f"breaking changes since {tag}:")
    print("\n".join(f"  {problem}" for problem in problems))
    marked = [s for s in git("log", f"{tag}..HEAD", "--format=%s").splitlines() if "!:" in s.split(" ", 1)[0]]
    if marked:
        print(f"allowed, a commit says so: {marked[0]}")
        print("tag a new major version once it is on main")
        return 0
    print("if this is meant, mark the commit `feat!: ...` and tag a new major version after it")
    return 1


if __name__ == "__main__":
    sys.exit(main())
