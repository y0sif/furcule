# Furcule

Assumption-first reasoning graphs. Drop in any text, an idea, a paper, a case file, a show's plot so far, and Furcule turns it into a directed graph of facts, assumptions and conclusions with provenance on every node. Negate an assumption and watch which conclusions lose support. Branch to explore an alternative with an AI agent. Diff branches side by side. Hunt for contradictions and loose ends.

Open source, local-first, MCP-first: your own coding agent or chat app supplies the reasoning, Furcule is the referee.

> Pre-alpha. The schema and CLI skeleton exist; the engine, MCP server and viewer are being built. See `docs/landscape.md` for why this exists next to Mindwrinkles, Argdown and a plain chat box.

## Layout

```
crates/furcule-core     graph model, propagation, branch + diff, checks, exports
crates/furcule-mcp      MCP server: stdio and streamable HTTP
crates/furcule-server   Axum: REST, SSE, serves the viewer
crates/furcule          the one binary
viewer/                 Vite, React, React Flow board
fixtures/               six real-world test cases
plugin/                 Claude Code plugin and skill
docs/                   landscape, naming, architecture
```

## Development

```bash
cargo test --workspace
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --all --check
cargo run -p furcule -- schema graph

cd viewer && pnpm install && pnpm build
```

## License

MIT
