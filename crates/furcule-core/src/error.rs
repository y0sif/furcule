//! Typed errors for the core model.

use thiserror::Error;

/// Errors raised by the core graph model.
#[derive(Debug, Error)]
pub enum Error {
    /// A node id was referenced that does not exist in the graph.
    #[error("unknown node `{0}`")]
    UnknownNode(String),
    /// An edge id was referenced that does not exist in the graph.
    #[error("unknown edge `{0}`")]
    UnknownEdge(String),
    /// A node or edge id was added twice.
    #[error("duplicate id `{0}`")]
    DuplicateId(String),
    /// The graph file could not be parsed.
    #[error("invalid graph: {0}")]
    Invalid(String),
    /// JSON (de)serialisation failed.
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}
