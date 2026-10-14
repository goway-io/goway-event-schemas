#!/usr/bin/env bash
# Writes every language's types from build/. Run through `make generate`, which bundles first.
set -euo pipefail
cd "$(dirname "$0")/.."
BIN=node_modules/.bin
CODEGEN=build/codegen.json

cp build/schemas.json packages/python/src/goway_events/schemas.json
uv run --quiet scripts/docs.py >/dev/null
(cd packages/python && uv run --quiet datamodel-codegen --input ../../build/schemas.json --input-file-type jsonschema \
  --output src/goway_events/models.py --output-model-type pydantic_v2.BaseModel --target-python-version 3.14 \
  --use-title-as-name --use-annotated --field-constraints --use-standard-collections --use-union-operator \
  --use-double-quotes --disable-timestamp --use-schema-description --use-field-description --use-subclass-enum \
  --capitalise-enum-members --extra-fields ignore --formatters ruff-format ruff-check)

PATH="$PWD/.tools/bin:$PATH" cargo typify --no-builder "$CODEGEN" -o packages/rust/src/generated.rs
(cd packages/rust && cargo fmt)

"$BIN/json2ts" -i build/schemas.json -o packages/ts/src/index.ts --additionalProperties false --unreachableDefinitions \
  --bannerComment "/* Generated from goway-event-schemas by make generate; change the schemas, never this file. */"

frames=$(python3 -c "import json; d=json.load(open('build/schemas.json'))['x-frames']; \
print(' '.join(f'$CODEGEN#/definitions/{t}' for side in ('client', 'server') for t in d[side].values()))")
# shellcheck disable=SC2086
"$BIN/quicktype" -s schema --lang dart --use-json-annotation --part-name models -o build/models.dart $frames >/dev/null
{ printf '// ignore_for_file: constant_identifier_names\n'; cat build/models.dart; } > packages/dart/lib/src/models.dart
(cd packages/dart && dart run build_runner build --delete-conflicting-outputs >/dev/null)
echo "generated python, rust, typescript and dart"
