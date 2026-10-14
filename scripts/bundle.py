"""Bundle schemas/ into one draft-07 document, and a variant shaped for the code generators.

build/schemas.json  every file's root schema under `definitions`, keyed by its title; nested titled objects and
                    enums are hoisted to their own definition so every language names them the same way.
build/codegen.json  the same, shaped for the code generators: the variants of each discriminated union are written
                    inline (typify only emits a tagged enum for inline variants and silently falls back to an
                    untagged one otherwise), and objects are opened up. The schemas stay closed so a producer
                    cannot send a field nobody agreed on; the generated types must stay open so a consumer keeps
                    working when a producer adds an optional field first.
"""

import copy
import json
import pathlib
import sys

ROOT = pathlib.Path(__file__).resolve().parent.parent
DRAFT_07 = "http://json-schema.org/draft-07/schema#"


def hoistable(node: dict) -> bool:
    return "title" in node and ("properties" in node or "enum" in node)


def hoist(node, definitions: dict, origin: str):
    if isinstance(node, list):
        return [hoist(item, definitions, origin) for item in node]
    if not isinstance(node, dict):
        return node
    node = {key: hoist(value, definitions, origin) for key, value in node.items()}
    for key in ("properties",):
        if key in node:
            node[key] = {name: lift(schema, definitions, origin) for name, schema in node[key].items()}
    if "anyOf" in node:
        node["anyOf"] = [lift(branch, definitions, origin) for branch in node["anyOf"]]
    return node


def lift(schema, definitions: dict, origin: str):
    if not isinstance(schema, dict) or not hoistable(schema):
        return schema
    title = schema["title"]
    if title in definitions and definitions[title] != schema:
        sys.exit(f"{origin}: title {title!r} is used for two different schemas")
    definitions[title] = schema
    return {"$ref": f"#/definitions/{title}"}


def rewrite_refs(node, base: pathlib.Path, titles: dict):
    if isinstance(node, list):
        return [rewrite_refs(item, base, titles) for item in node]
    if not isinstance(node, dict):
        return node
    node = {key: rewrite_refs(value, base, titles) for key, value in node.items()}
    ref = node.get("$ref")
    if ref and not ref.startswith("#"):
        node["$ref"] = f"#/definitions/{titles[(base.parent / ref).resolve()]}"
    return node


def bundle() -> dict:
    files = sorted((ROOT / "schemas").rglob("*.json"))
    schemas = {path.resolve(): json.loads(path.read_text()) for path in files}
    titles = {path: schema["title"] for path, schema in schemas.items()}
    if len(set(titles.values())) != len(titles):
        sys.exit("two schema files share a title")
    definitions: dict = {}
    for path, schema in schemas.items():
        schema = {key: value for key, value in schema.items() if key != "$schema"}
        schema = rewrite_refs(schema, path, titles)
        definitions[schema["title"]] = hoist(schema, definitions, str(path.relative_to(ROOT)))
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
