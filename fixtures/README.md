# Fixtures

One directory per real use case. Each holds:

- `README.md`: where the text comes from and which hidden assumption it is built around.
- `input.md`: the source text an extraction run reads.
- `expected.json`: what a correct graph must contain, and what must happen when the key assumption is negated.

## expected.json

Matching is by keywords, case-insensitive: a node satisfies an expectation when its `text` contains every keyword in the list. This keeps the check deterministic across agents that phrase things differently.

```json
{
  "case": "blacklist-safe",
  "must_extract": [
    { "kind": "assumption", "provenance": "inferred", "keywords": ["open", "safe"], "why": "never stated, needed for the conclusion" },
    { "kind": "inference", "keywords": ["opened", "safe"] }
  ],
  "negation": {
    "target": { "kind": "assumption", "keywords": ["open", "safe"] },
    "must_fall": [["opened", "safe"]],
    "must_stand": [["recordings", "gone"]]
  },
  "gaps": [ { "keywords": ["built", "safe"] } ],
  "contradictions": [ [["inspector", "permit"], ["county", "no record"]] ]
}
```

- `must_extract`: nodes that have to exist, with kind and optionally provenance.
- `negation`: negate the first node matching `target`; every `must_fall` inference must lose standing and every `must_stand` node must keep it.
- `gaps`: gap nodes a good extraction surfaces. Optional.
- `contradictions`: pairs of nodes that must be joined by a `contradicts` edge. Optional.

The harness in `furcule-core` tests (added with the engine) loads `actual/graph.json` produced by an extraction run and checks it against this file. Extraction itself is done by an agent, so the run is recorded, not executed in CI.
