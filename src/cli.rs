use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Query {
    /// The workspace that is currently focused
    CurrentWorkspace,
}

#[derive(Parser, Debug)]
// #[commmand(author, version, about, long_about = None)]
pub struct CliArgs {
    /// What information to query Hyprland for
    #[arg(short, long, value_enum)]
    pub query: Query,

    /// Whether to keep waiting for events instead of exiting after the initial query
    #[arg(short, long)]
    pub subscribe: bool,
}
