# Furcule

Assumption-first reasoning graph. Rust engine plus TypeScript viewer. Open-core, MCP-first, zero LLM cost in v1: the user's own agent supplies reasoning through MCP; this code is deterministic.

## Commands

```bash
cargo test --workspace
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --all --check
cargo run -p furcule -- schema graph          # JSON Schema the viewer types are generated from

cd viewer && pnpm install
pnpm build && pnpm lint && pnpm format:check
```

## Pre-push checklist

1. `cargo fmt --all --check`
2. `cargo clippy --all-targets --all-features -- -D warnings`
3. `cargo test --workspace`
4. `cd viewer && pnpm build && pnpm lint && pnpm format:check`

## Layout

- `crates/furcule-core`: schema (`schema.rs`), errors, and later standing/propagation, branch overlay + diff, checks, exports. No I/O, no LLM.
- `crates/furcule-mcp`: MCP tools, prompts, resources over stdio and streamable HTTP.
- `crates/furcule-server`: Axum REST + SSE, serves the embedded viewer.
- `crates/furcule`: the single binary with subcommands.
- `viewer/`: Vite + React + React Flow. Types are generated from `furcule schema`, never hand-written.
- `fixtures/`: one directory per real use case, each with `input.md` and `expected.json`.
- `plugin/`: Claude Code plugin (`.claude-plugin/plugin.json`, `.mcp.json`, `skills/`).
- `docs/`: `landscape.md` (Phase 0), `naming.md` (decided: Furcule), `architecture.md`, `schema.md`, `tech-stack.md`, `comparison.md`, `faq.md`. Also `docs/BRAND.md` and `docs/DESIGN.md`: maintainer-only, git-ignored, present on the maintainer's machine; read them when they exist before touching copy or the viewer's look.

## Conventions

- Edition 2024, `unsafe_code = "forbid"`, `missing_docs = "warn"`, clippy pedantic on. CI denies warnings.
- Library crates return typed errors (`thiserror`); the binary uses `anyhow`.
- Logging through `tracing`; never `println!` outside the CLI's user-facing output. Logs go to stderr so stdout stays clean for MCP over stdio.
- Storage is a directory per graph: `graph.json` plus `branches/<name>.json` overlays. Keep it git-diffable.
- Every node and edge carries `provenance` (extracted | inferred | ambiguous) and nodes carry `status` (verified | unverified | negated). Do not add fields that lose that audit trail.
- Commits: imperative mood with a prefix (`feat:`, `fix:`, `docs:`, `chore:`, `refactor:`).
- Non-trivial features go through the `/orchestrate` flow: research, plan with acceptance criteria, implement, verify, then the maintainer tests locally before anything is pushed.

## Release process

1. Bump `version` in `Cargo.toml` (`[workspace.package]`), `viewer/package.json`, `contrib/npm/package.json`, `contrib/aur/PKGBUILD`, `contrib/homebrew/furcule.rb`.
2. `cargo update -w` and `cd viewer && pnpm install` so lockfiles match.
3. Run the pre-push checklist.
4. Commit `chore: release vX.Y.Z`, tag `vX.Y.Z`, push the tag. `release.yml` builds five targets and publishes the GitHub release with checksums.
5. Publish crates in dependency order: `furcule-core`, `furcule-mcp`, `furcule-server`, `furcule`. Then `cd contrib/npm && npm publish`. Update the Homebrew formula sha256 values and the AUR package.

## Observability and config

- Logs: `tracing` to stderr, level from `RUST_LOG` (default `info`). Stdout is reserved for command output and the MCP stdio transport.
- `furcule serve` exposes `GET /health` returning 200 with version and uptime; CI deploys and the systemd unit rely on it.
- Config precedence: CLI flag > environment (`FURCULE_GRAPHS`, `FURCULE_PORT`, `FURCULE_BIND`, `RUST_LOG`) > `~/.config/furcule/config.toml` > defaults. Zero config must work.
