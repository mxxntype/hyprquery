use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Query {
    /// The workspace that is currently focused
    ActiveWorkspace,
    /// All workspaces as an array
    Workspaces,
    /// The currently focused window
    ActiveWindow,
    /// The current keyboard layout
    KeyboardLayout,
}

#[derive(Parser, Debug)]
// #[commmand(author, version, about, long_about = None)]
pub struct CliArgs {
    /// What information to query Hyprland for
    #[arg(short, long, value_enum)]
    pub query: Query,

    /// Wait for corresponding events and re-query when they happen
    #[arg(short, long)]
    pub subscribe: bool,
}
