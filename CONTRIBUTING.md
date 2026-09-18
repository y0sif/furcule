# Contributing to Furcule

Thanks for looking. Issues, fixtures and pull requests are all welcome.

## Setup

```bash
git clone https://github.com/y0sif/furcule
cd furcule
cargo build --workspace
cd viewer && pnpm install
```

Rust stable with `rustfmt` and `clippy` (the `rust-toolchain.toml` installs them), Node 20 or newer, pnpm 10.

## Before you push

```bash
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --workspace
cd viewer && pnpm build && pnpm lint && pnpm format:check
```

CI runs exactly these. Warnings fail the build.

## Style

- Edition 2024. No `unsafe`. Public items are documented.
- Library crates return typed errors with `thiserror`; the binary uses `anyhow`.
- Log with `tracing`, never `println!` outside user-facing CLI output. Logs go to stderr so stdout stays clean for MCP over stdio.
- Every node and edge keeps its `provenance`; every node keeps its `status`. Do not add features that lose that audit trail.
- Viewer types are generated from `furcule schema`, never hand-written.

## Commits

Imperative mood with a prefix: `feat:`, `fix:`, `docs:`, `chore:`, `refactor:`, `polish:`. Summary line, blank line, detail if needed.

## Pull requests

1. Branch from `main`: `feat/short-description` or `fix/short-description`.
2. Keep one change per PR.
3. Fill in the PR template: what it does, how it was tested.
4. Add or update tests. Engine changes need a unit test; extraction changes need a fixture.

## Fixtures

`fixtures/<case>/` holds `input.md` and `expected.json`. A good fixture is a real text with at least one unstated assumption and one conclusion that should fall when it is negated. Add the case to the fixture README and say where the text came from.

## Reporting bugs

Use the bug template. Include the graph directory if you can share it, the exact command, and the output of `furcule --version`.
