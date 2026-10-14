"""Bundle schemas/ into one draft-07 document, and a variant shaped for the code generators.

build/schemas.json  every file's root schema under `definitions`, keyed by its title. A titled object or enum nested
                    in a file is hoisted to its own definition, so every language names it the same way, when it is
                    a property, an array's `items`, or a branch of `anyOf`, `oneOf` or `allOf`; a titled schema
                    anywhere else stays inline. A title belongs to one schema: two different nested schemas with
                    the same title, or a nested title equal to a file's root title, stop the bundle.
build/codegen.json  the same, shaped for the code generators: the variants of each discriminated union are written
                    inline (typify only emits a tagged enum for inline variants and silently falls back to an
                    untagged one otherwise), and objects are opened up. The schemas stay closed so a producer
                    cannot send a field nobody agreed on; the generated types must stay open so a consumer keeps
                    working when a producer adds an optional field first.

A `$ref` names a whole file by its relative path; a `#fragment` in a file is refused, since it would point into the
bundle instead of the file.
"""

import copy
import json
import pathlib
import sys

ROOT = pathlib.Path(__file__).resolve().parent.parent
DRAFT_07 = "http://json-schema.org/draft-07/schema#"


def hoistable(node: dict) -> bool:
    return "title" in node and ("properties" in node or "enum" in node)


class Hoister:
    """Lifts nested titled schemas of one file into `definitions`; `roots` maps every file's root title to its file."""

    def __init__(self, definitions: dict, roots: dict, origin: str):
        self.definitions = definitions
        self.roots = roots
        self.origin = origin

    def hoist(self, node):
        if isinstance(node, list):
            return [self.hoist(item) for item in node]
        if not isinstance(node, dict):
            return node
        node = {key: self.hoist(value) for key, value in node.items()}
        if isinstance(node.get("properties"), dict):
            node["properties"] = {name: self.lift(schema) for name, schema in node["properties"].items()}
        if isinstance(node.get("items"), dict):
            node["items"] = self.lift(node["items"])
        for key in ("anyOf", "oneOf", "allOf"):
            if isinstance(node.get(key), list):
                node[key] = [self.lift(branch) for branch in node[key]]
        return node

    def lift(self, schema):
        if not isinstance(schema, dict) or not hoistable(schema):
            return schema
        title = schema["title"]
        if title in self.roots:
            sys.exit(f"{self.origin}: nested title {title!r} is the root title of {self.roots[title]}; rename one")
        if title in self.definitions and self.definitions[title] != schema:
            sys.exit(f"{self.origin}: title {title!r} is used for two different schemas")
        self.definitions[title] = schema
        return {"$ref": f"#/definitions/{title}"}


def rewrite_refs(node, path: pathlib.Path, titles: dict):
    if isinstance(node, list):
        return [rewrite_refs(item, path, titles) for item in node]
    if not isinstance(node, dict):
        return node
    node = {key: rewrite_refs(value, path, titles) for key, value in node.items()}
    ref = node.get("$ref")
    if isinstance(ref, str):
        where = path.relative_to(ROOT)
        if "#" in ref:
            sys.exit(f"{where}: $ref {ref!r} has a #fragment; refer to a whole schema file by its relative path")
        target = (path.parent / ref).resolve()
        if target not in titles:
            sys.exit(f"{where}: $ref {ref!r} is not a schema file under schemas/")
        node["$ref"] = f"#/definitions/{titles[target]}"
    return node


def bundle() -> dict:
    files = sorted((ROOT / "schemas").rglob("*.json"))
    schemas = {path.resolve(): json.loads(path.read_text()) for path in files}
    titles = {path: schema["title"] for path, schema in schemas.items()}
    roots = {title: str(path.relative_to(ROOT)) for path, title in titles.items()}
    if len(roots) != len(titles):
        sys.exit("two schema files share a title")
    definitions: dict = {}
    for path, schema in schemas.items():
        schema = {key: value for key, value in schema.items() if key != "$schema"}
        schema = rewrite_refs(schema, path, titles)
        definitions[schema["title"]] = Hoister(definitions, roots, str(path.relative_to(ROOT))).hoist(schema)
    return {
        "$schema": DRAFT_07,
        "title": "GowayEventSchemas",
        "x-subjects": wire_names(schemas, "events"),
        "x-frames": {side: wire_names(schemas, f"frames/{side}") for side in ("client", "server")},
        "definitions": dict(sorted(definitions.items())),
    }


def wire_names(schemas: dict, folder: str) -> dict:
    """File name = NATS subject or frame type, so a producer can find its schema by what it sends."""
    base = (ROOT / "schemas" / folder).resolve()
    return {path.stem: schema["title"] for path, schema in sorted(schemas.items()) if path.parent == base}


def opened(node):
    if isinstance(node, list):
        return [opened(item) for item in node]
    if not isinstance(node, dict):
        return node
    return {key: opened(value) for key, value in node.items() if not (key == "additionalProperties" and value is False)}


def for_codegen(document: dict) -> dict:
    document = opened(copy.deepcopy(document))
    definitions = document["definitions"]
    for schema in definitions.values():
        if "oneOf" in schema and "discriminator" in schema:
            schema["oneOf"] = [
                {key: value for key, value in definitions[branch["$ref"].rsplit("/", 1)[-1]].items() if key != "title"}
                if "$ref" in branch
                else branch
                for branch in schema["oneOf"]
            ]
    return document


def main() -> None:
    out = ROOT / "build"
    out.mkdir(exist_ok=True)
    document = bundle()
    (out / "schemas.json").write_text(json.dumps(document, indent=2, ensure_ascii=False) + "\n")
    (out / "codegen.json").write_text(json.dumps(for_codegen(document), indent=2, ensure_ascii=False) + "\n")
    print(f"bundled {len(document['definitions'])} definitions")


if __name__ == "__main__":
    main()
