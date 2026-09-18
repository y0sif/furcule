# Architecture

## The split

Furcule is two halves with a hard line between them.

**The engine** is deterministic Rust. It stores graphs, computes standing, applies branch overlays, diffs them, runs checks, renders the board and speaks MCP. It never calls a language model.

**The driver** is whatever supplies reasoning: a coding agent over local MCP (Claude Code, OpenCode), a chat app over a remote MCP connector (Claude.ai, ChatGPT), or later a hosted agent. The driver reads text and proposes nodes; the engine keeps score.

This split is the business model as much as the design. The user's existing subscription pays for the thinking, so v1 ships at zero LLM cost, and the same engine serves a hosted tier later without change.

## Crates

| Crate | Responsibility |
|---|---|
| `furcule-core` | Schema (`schema.rs`), standing engine, branch overlay and diff, checks, exports. No I/O. |
| `furcule-mcp` | MCP tools, prompts and resources. Stdio transport for local agents, streamable HTTP for remote connectors. Built on the official `rmcp` SDK. |
| `furcule-server` | Axum HTTP: REST over a graph directory, SSE for live updates, the embedded viewer. |
| `furcule` | The binary. Subcommands wrap the crates above. |

## Storage

One directory per graph, plain files, git-friendly:

```
idea/
├── graph.json              the base graph
├── branches/
│   └── safe-replaced.json  ordered ops on top of the base
└── sources/
    └── idea.md             the text nodes point back into
```

A branch is a list of ops, not a copy, so `git diff` on a branch file reads as a list of decisions. SQLite only appears in the hosted tier.

## Standing

Standing is the one computation everything else leans on.

- A `fact` or `assumption` stands unless its status is `negated`.
- An `inference` collects its incoming `supports` edges into groups: edges sharing a `group` value form a linked set, each ungrouped edge is its own set. The inference stands if any set has every source standing. With no supports edges it is `unsupported`, which is a check finding, not a fall.
- `entity` and `gap` nodes do not stand or fall.
- `depends_on` does not carry standing but is walked by the cycle check and shown in the viewer.

Propagation is a topological pass over `supports` edges. Cycles through `supports` are rejected at load with a check finding, so the pass is linear.

`negate <node>` sets the status, recomputes, and reports every inference whose standing changed, in dependency order, which is also the order the viewer animates.

## Branches and diff

`Branch { name, note, ops }`. Ops: `set_status`, `add_node`, `add_edge`, `remove_node`, `remove_edge`, `edit_text`. Applying a branch folds the ops over a clone of the base and recomputes standing.

`diff a b` compares two materialised graphs and reports three things: structural changes (nodes and edges added, removed, edited), status changes, and standing changes (inferences that stand in one and fall in the other). The viewer renders the same three lists as overlay or split.

## Checks

| Check | Finding |
|---|---|
| contradiction | both ends of a `contradicts` edge stand |
| unsupported | an inference with no `supports` edge |
| orphan | an entity with no edge at all |
| open gap | a gap node not marked resolved |
| cycle | a cycle through `supports` or `depends_on` |
| ambiguous | count of nodes and edges with `ambiguous` provenance, for the human to clear |

## MCP surface, v1

Tools: `graph.create`, `graph.load`, `nodes.add`, `edges.add`, `node.set_status`, `node.negate`, `graph.standing`, `branch.create`, `branch.apply`, `branch.diff`, `graph.check`, `graph.export`, `graph.url`.

Prompts: `extract` (text to nodes and edges with provenance, following the twelve-step reconstruction in Pirozelli et al. 2026 including implicit-premise recovery), `reductio` (attempt to derive a contradiction from a chosen conclusion using only standing nodes), `alternatives` (given a negated assumption, propose replacements consistent with the standing facts).

Resources: `graph://<dir>` and `graph://<dir>/branches/<name>`.

## Viewer

Vite, React, React Flow. Types come from `furcule schema` through JSON Schema, generated in CI, never hand-written. Layout is dagre, top to bottom, root at the top, because reasoning in this tool is directional. The board reads the graph over REST and listens on SSE, so an agent editing through MCP shows up live.

## Decisions

- **No LLM in the binary.** Subscription users far outnumber API-key users, and the engine is the moat.
- **JSON files, not a database.** Git diff on a branch file is a feature. A database arrives with the hosted tier.
- **One binary.** `furcule mcp` in an MCP config is the whole install story.
- **Rust engine, TypeScript viewer.** Single static binary, `z3` available for the SMT layer later, and React Flow is the right tool for an editable board of hundreds of nodes.
- **Formal layer in stages.** v1 is propagation plus the reductio prompt, v2 adds SMT for graphs that encode cleanly, Lean autoformalisation is out of scope.

## Roadmap

| Version | Lands |
|---|---|
| v0.1 | engine (standing, branch, diff, checks), stdio MCP, extraction prompt, viewer with overlay and split diff, six fixtures passing, cargo and npx install |
| v0.2 | streamable HTTP MCP for remote connectors, exports, AUR and Homebrew, config file |
| v0.3 | SMT validity, hosted tier with cloud graphs and share links |
