# Tech stack

Every dependency, and why it beat the alternatives.

## Rust engine

| Crate | Why |
|---|---|
| `serde`, `serde_json` | The graph is JSON on disk and over the wire. `preserve_order` keeps files diffable. |
| `schemars` | One source of truth: the Rust types emit the JSON Schema the viewer types are generated from. |
| `indexmap` | Node and edge maps that keep insertion order, so files and diffs are stable. |
| `petgraph` | Topological passes and cycle detection for standing, instead of hand-rolling them. |
| `thiserror` | Typed errors in library crates so callers can match on them. |
| `anyhow` | Context-chained errors in the binary only. |
| `clap` | Derive-based CLI with env overrides and shell completions. |
| `tracing`, `tracing-subscriber` | Structured logs to stderr, level via `RUST_LOG`. Stdout stays clean for MCP over stdio. |
| `rmcp` | The official Rust MCP SDK. Passes the conformance suite, supports stdio and streamable HTTP. |
| `axum`, `tower-http` | The local server. Same ecosystem as `rmcp`'s HTTP transport, so one runtime. |
| `tokio` | Async runtime for the server and MCP transports. |
| `rust-embed` | The built viewer is compiled into the binary, so `furcule serve` needs nothing else installed. |
| `uuid` (v7), `jiff` | Ids and timestamps where the engine needs them; slugs are preferred for node ids. |
| `camino`, `directories` | UTF-8 paths and XDG config locations. |

Deferred: `z3` for the SMT layer in v0.3, behind an `smt` feature so default builds stay light.

Not chosen: an embedded database. JSON files are git-diffable and that is a feature.

## Viewer

| Package | Why |
|---|---|
| Vite | Fast builds, produces a static bundle the binary embeds. |
| React 19 | React Flow is React-first. |
| `@xyflow/react` (React Flow) | Node-based editor primitives, custom node components, selection, the board metaphor. Cytoscape is stronger for analysis, Sigma for million-node WebGL; neither matters here, hundreds of nodes on an editable board does. |
| `@dagrejs/dagre` | Top-to-bottom layered layout for a directed graph with a root. ELK stays an option if layouts get complex. |
| TypeScript, ESLint, Prettier | Strict types, zero warnings in CI. |
| pnpm | Lockfile committed, fast, standard in CI. |

## Distribution

Single binary. Published to crates.io, wrapped for npm with platform binaries the way Biome does so `npx furcule mcp` works in MCP configs, plus AUR and a Homebrew tap.
