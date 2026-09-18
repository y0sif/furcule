//! Local HTTP server for Furcule.
//!
//! Serves the embedded viewer, a REST API over the graph directory, and an
//! SSE stream so the board updates live while an agent edits the graph.
//!
//! Not implemented yet: see `docs/architecture.md`.

pub use furcule_core as core;
