//! The on-disk and over-the-wire schema of a Furcule graph.
//!
//! A graph is a directed graph of [`Node`]s joined by [`Edge`]s. Every node
//! carries a [`Provenance`] tag (where it came from) and a [`Status`] (what we
//! currently believe about it). Inferences are held up by `supports` edges;
//! edges that share a `group` on the same target form a *linked* set (all of
//! them are needed), while ungrouped edges are *convergent* alternatives (any
//! one is enough). A [`Branch`] is an ordered list of [`BranchOp`]s applied on
//! top of a base graph, which keeps branches small and git-diffable.
//!
//! The JSON Schema for these types is emitted by `furcule schema` and is the
//! contract the TypeScript viewer is generated from.

use indexmap::IndexMap;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Schema version written into every graph file.
pub const SCHEMA_VERSION: u32 = 1;

/// Identifier of a node. Short, stable, human-readable slugs are preferred
/// (`f1`, `a3`, `safe-was-replaced`); the engine only requires uniqueness.
pub type NodeId = String;
/// Identifier of an edge.
pub type EdgeId = String;

/// What kind of statement a node is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum NodeKind {
    /// Something given or observed. Stands unless negated.
    Fact,
    /// Something taken as true without being established. The nodes this
    /// tool exists to surface. Stands unless negated.
    Assumption,
    /// A conclusion drawn from other nodes. Stands only while a support group
    /// stands.
    Inference,
    /// A person, place, object or organisation the statements are about.
    Entity,
    /// A known unknown: an open question or a loose end.
    Gap,
}

/// Where a node or edge came from. Borrowed from graphify's audit trail.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Provenance {
    /// Stated explicitly in the source text.
    Extracted,
    /// Not stated, but needed for the reasoning to hold; supplied by a reader
    /// or an agent.
    Inferred,
    /// Could not be placed with confidence. Needs a human look.
    Ambiguous,
}

/// What we currently hold about a node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Default)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    /// Checked against evidence and holds.
    Verified,
    /// Not yet checked. The default.
    #[default]
    Unverified,
    /// Held to be false. Negating an assumption is the core move of the tool.
    Negated,
}

/// Pointer back into the source material.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Default)]
pub struct Source {
    /// File the statement was taken from, relative to the graph directory.
    pub file: String,
    /// Optional character span `[start, end)` inside the file.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub span: Option<(usize, usize)>,
    /// Optional verbatim quote.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quote: Option<String>,
}

/// A statement in the graph.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct Node {
    /// Unique id within the graph.
    pub id: NodeId,
    /// Kind of statement.
    pub kind: NodeKind,
    /// The statement itself, one sentence where possible.
    pub text: String,
    /// Where it came from.
    pub provenance: Provenance,
    /// What we hold about it.
    #[serde(default)]
    pub status: Status,
    /// Pointer into the source material, when there is one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
    /// Free-form tags (`suspect`, `motive`, `experiment-3`).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
}

/// How two nodes relate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum EdgeKind {
    /// `from` is a reason for `to`. Drives standing.
    Supports,
    /// `from` and `to` cannot both stand. Drives contradiction checks.
    Contradicts,
    /// `to` presupposes `from` without being argued from it.
    DependsOn,
    /// Loose association, usually to or from an [`NodeKind::Entity`].
    RelatesTo,
}

/// A directed relation between two nodes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct Edge {
    /// Unique id within the graph.
    pub id: EdgeId,
    /// Source node.
    pub from: NodeId,
    /// Target node.
    pub to: NodeId,
    /// Relation kind.
    pub kind: EdgeKind,
    /// Where the relation came from.
    pub provenance: Provenance,
    /// For `supports` edges: edges sharing a group on the same target are a
    /// linked set (all needed). Ungrouped edges are convergent alternatives.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// Optional note explaining the relation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

/// A whole reasoning graph. This is the `graph.json` file.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct Graph {
    /// Schema version, see [`SCHEMA_VERSION`].
    pub version: u32,
    /// Human title of the graph.
    pub title: String,
    /// Nodes, keyed by id, in insertion order.
    #[serde(default)]
    pub nodes: IndexMap<NodeId, Node>,
    /// Edges, keyed by id, in insertion order.
    #[serde(default)]
    pub edges: IndexMap<EdgeId, Edge>,
}

impl Graph {
    /// Create an empty graph with the current schema version.
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            version: SCHEMA_VERSION,
            title: title.into(),
            nodes: IndexMap::new(),
            edges: IndexMap::new(),
        }
    }
}

/// One change applied on top of a base graph.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum BranchOp {
    /// Change what we hold about a node. Negating an assumption is this.
    SetStatus {
        /// Target node.
        node: NodeId,
        /// New status.
        status: Status,
    },
    /// Add a node (an alternative assumption, a new fact).
    AddNode {
        /// The node to add.
        node: Node,
    },
    /// Add an edge.
    AddEdge {
        /// The edge to add.
        edge: Edge,
    },
    /// Remove a node and every edge touching it.
    RemoveNode {
        /// Target node.
        node: NodeId,
    },
    /// Remove an edge.
    RemoveEdge {
        /// Target edge.
        edge: EdgeId,
    },
    /// Rewrite a node's text.
    EditText {
        /// Target node.
        node: NodeId,
        /// New text.
        text: String,
    },
}

/// A named overlay of changes on a base graph. This is `branches/<name>.json`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct Branch {
    /// Branch name, also the file stem.
    pub name: String,
    /// Why this branch exists, e.g. "what if the safe was never opened".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    /// Ordered changes applied on top of the base graph.
    #[serde(default)]
    pub ops: Vec<BranchOp>,
}

/// JSON Schema for the graph file, used to generate viewer types.
pub fn graph_schema() -> schemars::Schema {
    schemars::schema_for!(Graph)
}

/// JSON Schema for a branch file.
pub fn branch_schema() -> schemars::Schema {
    schemars::schema_for!(Branch)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Graph {
        let mut g = Graph::new("The safe");
        g.nodes.insert(
            "f1".into(),
            Node {
                id: "f1".into(),
                kind: NodeKind::Fact,
                text: "The recordings are gone.".into(),
                provenance: Provenance::Extracted,
                status: Status::Verified,
                source: None,
                tags: vec![],
            },
        );
        g.nodes.insert(
            "a1".into(),
            Node {
                id: "a1".into(),
                kind: NodeKind::Assumption,
                text: "The only way to the recordings is to open the safe.".into(),
                provenance: Provenance::Inferred,
                status: Status::Unverified,
                source: None,
                tags: vec![],
            },
        );
        g.nodes.insert(
            "i1".into(),
            Node {
                id: "i1".into(),
                kind: NodeKind::Inference,
                text: "Someone opened the safe.".into(),
                provenance: Provenance::Extracted,
                status: Status::Unverified,
                source: None,
                tags: vec![],
            },
        );
        g.edges.insert(
            "e1".into(),
            Edge {
                id: "e1".into(),
                from: "f1".into(),
                to: "i1".into(),
                kind: EdgeKind::Supports,
                provenance: Provenance::Extracted,
                group: Some("g1".into()),
                note: None,
            },
        );
        g.edges.insert(
            "e2".into(),
            Edge {
                id: "e2".into(),
                from: "a1".into(),
                to: "i1".into(),
                kind: EdgeKind::Supports,
                provenance: Provenance::Inferred,
                group: Some("g1".into()),
                note: None,
            },
        );
        g
    }

    #[test]
    fn graph_roundtrips_through_json() {
        let g = sample();
        let json = serde_json::to_string_pretty(&g).unwrap();
        let back: Graph = serde_json::from_str(&json).unwrap();
        assert_eq!(g, back);
        assert!(json.contains("\"kind\": \"assumption\""));
    }

    #[test]
    fn status_defaults_to_unverified() {
        let json = r#"{"id":"x","kind":"fact","text":"t","provenance":"extracted"}"#;
        let n: Node = serde_json::from_str(json).unwrap();
        assert_eq!(n.status, Status::Unverified);
        assert!(n.tags.is_empty());
    }

    #[test]
    fn branch_ops_are_tagged() {
        let b = Branch {
            name: "safe-replaced".into(),
            note: Some("what if the safe was swapped".into()),
            ops: vec![BranchOp::SetStatus {
                node: "a1".into(),
                status: Status::Negated,
            }],
        };
        let json = serde_json::to_string(&b).unwrap();
        assert!(json.contains(r#""op":"set_status""#));
        let back: Branch = serde_json::from_str(&json).unwrap();
        assert_eq!(b, back);
    }

    #[test]
    fn schemas_generate() {
        let s = serde_json::to_value(graph_schema()).unwrap();
        assert_eq!(s["title"], "Graph");
        let s = serde_json::to_value(branch_schema()).unwrap();
        assert_eq!(s["title"], "Branch");
    }
}
