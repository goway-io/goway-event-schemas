# goway-event-schemas

What GoWay's services and apps send each other when it is not a REST call: the events on NATS JetStream and the
frames on the realtime WebSocket. Written once in AsyncAPI 3.1 and JSON Schema (draft-07); typed models for
Python, Rust, TypeScript and Dart are generated from here, so every side reads the same shape.

REST stays in each service's OpenAPI, generated from its code.

## Layout

| Path | What |
|---|---|
| `asyncapi/events.yaml` | NATS subjects: who publishes, who consumes, which payload |
| `asyncapi/realtime.yaml` | The app socket: frames apps send and frames the server sends |
| `schemas/events/<subject>.json` | One payload per NATS subject; the file name is the subject |
| `schemas/frames/{client,server}/<type>.json` | One frame per file; the file name is the frame `type` |
| `examples/{valid,invalid}/<Title>/*.json` | Payloads every check and every language is tested against |
| `packages/python` | `goway-events`: pydantic models plus `check_event` / `check_frame` for producers' tests |
| `packages/rust` | `goway-events` crate (serde) |
| `packages/ts` | `@goway/events` types |
| `packages/dart` | `goway_events`: the socket frames only, for the apps |

## Rules

- A payload only grows within a version. Never rename, retype or remove a field; never add an enum value a
  consumer may not know. A breaking change is a new subject ending in `.v2`, published beside the old one until
  every consumer has moved.
- Schemas are closed (`additionalProperties: false`): a producer cannot send a field nobody agreed on. Generated
  types are open: a consumer ignores a field it does not know yet.
- draft-07 only, no `if`/`then`/`else`, `$ref` to relative files. A nested object or enum gets a `title`, which
  becomes its type name in every language.
- A required field that may be null is written `"type": ["string", "null"]`; a nullable enum is
  `anyOf: [titled enum, null]`.

## Adding an event or a frame

1. Write `schemas/events/<subject>.json` (or a frame under `schemas/frames/...` and add it to the union file).
2. Add the message to `asyncapi/events.yaml` (with `x-publisher`, `x-consumers`) or `asyncapi/realtime.yaml`.
3. Add at least one valid and one invalid example.
4. `make check generate test` and commit the generated code with the schema.
5. In the producing service, check what it publishes in its tests: `goway_events.check_event(subject, payload)`.

## Commands

`make tools` once, then `make check` (validate, lint, examples), `make generate` (all languages), `make test`,
`make docs` (HTML pages in `build/docs`), `make drift` (fails when committed generated code is stale).
