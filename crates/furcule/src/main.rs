//! The `furcule` command line.

use anyhow::{Result, bail};
use clap::{Parser, Subcommand};

/// Assumption-first reasoning graph.
#[derive(Parser)]
#[command(name = "furcule", version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Print the JSON Schema of a graph file (`graph`) or a branch file (`branch`).
    Schema {
        /// Which schema to print.
        #[arg(default_value = "graph")]
        which: String,
    },
    /// Extract facts, assumptions and conclusions from text into a graph directory.
    Extract,
    /// Add nodes or edges to a graph.
    Add,
    /// Negate an assumption and report which conclusions lose support.
    Negate,
    /// Create or list branches.
    Branch,
    /// Diff two branches, or a branch against its base.
    Diff,
    /// Run consistency and gap checks.
    Check,
    /// Export to JSON, Argdown, JSON Canvas or `GraphML`.
    Export,
    /// Serve the viewer and API locally.
    Serve,
    /// Run the MCP server over stdio.
    Mcp,
}

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .with_writer(std::io::stderr)
        .init();

    let cli = Cli::parse();
    match cli.command {
        Command::Schema { which } => {
            let schema = match which.as_str() {
                "graph" => furcule_core::schema::graph_schema(),
                "branch" => furcule_core::schema::branch_schema(),
                other => bail!("unknown schema `{other}`, expected `graph` or `branch`"),
            };
            println!("{}", serde_json::to_string_pretty(&schema)?);
            Ok(())
        }
        Command::Extract
        | Command::Add
        | Command::Negate
        | Command::Branch
        | Command::Diff
        | Command::Check
        | Command::Export
        | Command::Serve
        | Command::Mcp => bail!("not implemented yet, see docs/architecture.md for the plan"),
    }
}
