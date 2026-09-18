//! MCP server for Furcule.
//!
//! Exposes the graph engine as tools (`graph.create`, `nodes.add`,
//! `node.negate`, `graph.standing`, `branch.create`, `branch.diff`,
//! `graph.check`, `graph.export`), prompts (`extract`, `reductio`,
//! `alternatives`) and resources (`graph://<id>`), over stdio for local coding
//! agents and over streamable HTTP for remote connectors.
//!
//! Not implemented yet: see `docs/architecture.md`.

pub use furcule_core as core;
