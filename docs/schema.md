# Graph schema

The source of truth is `crates/furcule-core/src/schema.rs`. `furcule schema graph` and `furcule schema branch` print the JSON Schema, and the viewer's types are generated from it.

## Node

| Field | Type | Notes |
|---|---|---|
| `id` | string | unique in the graph; short slugs preferred (`f1`, `a3`, `safe-was-replaced`) |
| `kind` | `fact` \| `assumption` \| `inference` \| `entity` \| `gap` | see below |
| `text` | string | one sentence where possible |
| `provenance` | `extracted` \| `inferred` \| `ambiguous` | where it came from |
| `status` | `verified` \| `unverified` \| `negated` | default `unverified` |
| `source` | `{ file, span?, quote? }` | optional pointer into `sources/` |
| `tags` | string[] | free-form |

Kinds:

- **fact**: given or observed. Stands unless negated.
- **assumption**: taken as true without being established. The reason the tool exists. Stands unless negated.
- **inference**: drawn from other nodes. Stands only while a support group stands.
- **entity**: a person, place, object or organisation the statements are about.
- **gap**: a known unknown, an open question, a loose end.

## Edge

| Field | Type | Notes |
|---|---|---|
| `id` | string | unique |
| `from`, `to` | node ids | direction matters |
| `kind` | `supports` \| `contradicts` \| `depends_on` \| `relates_to` | |
| `provenance` | as for nodes | |
| `group` | string, optional | `supports` only: same group on the same target = linked set |
| `note` | string, optional | |

## Graph file

```json
{
  "version": 1,
  "title": "The safe",
  "nodes": {
    "f1": { "id": "f1", "kind": "fact", "text": "The recordings are gone.", "provenance": "extracted", "status": "verified" },
    "a1": { "id": "a1", "kind": "assumption", "text": "The only way to the recordings is to open the safe.", "provenance": "inferred" },
    "i1": { "id": "i1", "kind": "inference", "text": "Someone opened the safe.", "provenance": "extracted" }
  },
  "edges": {
    "e1": { "id": "e1", "from": "f1", "to": "i1", "kind": "supports", "provenance": "extracted", "group": "g1" },
    "e2": { "id": "e2", "from": "a1", "to": "i1", "kind": "supports", "provenance": "inferred", "group": "g1" }
  }
}
```

`nodes` and `edges` are maps keyed by id and keep insertion order.

## Branch file

```json
{
  "name": "safe-replaced",
  "note": "what if the safe was never opened",
  "ops": [
    { "op": "set_status", "node": "a1", "status": "negated" },
    { "op": "add_node", "node": { "id": "a2", "kind": "assumption", "text": "The whole safe was swapped for an empty replica.", "provenance": "inferred" } },
    { "op": "add_node", "node": { "id": "i2", "kind": "inference", "text": "Whoever built the safe could build its twin.", "provenance": "inferred" } },
    { "op": "add_edge", "edge": { "id": "e3", "from": "f1", "to": "i2", "kind": "supports", "provenance": "inferred", "group": "g2" } },
    { "op": "add_edge", "edge": { "id": "e4", "from": "a2", "to": "i2", "kind": "supports", "provenance": "inferred", "group": "g2" } }
  ]
}
```

Ops are tagged with `op` and applied in order. Available: `set_status`, `add_node`, `add_edge`, `remove_node` (also removes touching edges), `remove_edge`, `edit_text`.

## Standing, in one paragraph

Facts and assumptions stand unless negated. An inference groups its incoming `supports` edges by `group` (ungrouped edges are singleton groups) and stands if any group has all its sources standing. No supports at all means unsupported, which is reported by `check`, not treated as a fall. Entities and gaps neither stand nor fall.

## Versioning

`version` is the schema version, currently 1. Breaking changes bump it and the loader migrates older files forward.
