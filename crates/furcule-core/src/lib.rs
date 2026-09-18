//! Core of Furcule: the reasoning-graph data model and, in later phases, the
//! standing (propagation) engine, branch overlays with diff, and consistency
//! checks.
//!
//! This crate has no I/O and no LLM dependency. Everything that reasons is
//! deterministic here; whoever supplies intelligence (a coding agent over MCP,
//! a chat app over a remote connector, or a hosted agent) talks to this model.

pub mod error;
pub mod schema;

pub use error::Error;
pub use schema::{
    Branch, BranchOp, Edge, EdgeKind, Graph, Node, NodeKind, Provenance, Source, Status,
};

/// Result alias used across the crate.
pub type Result<T> = std::result::Result<T, Error>;
