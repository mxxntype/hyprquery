use clap::builder::styling::{AnsiColor, Effects, Styles};
use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Query {
    /// The workspace that is currently focused.
    ActiveWorkspace,
    /// All workspaces as an array.
    Workspaces,
    /// The currently focused window.
    ActiveWindow,
    /// The current keyboard layout.
    KeyboardLayout,
}

/// Tiny tool for getting information from Hyprland's IPC.
#[derive(Parser, Debug)]
#[command(version, author, about)]
#[command(styles(Styles::styled()
    .usage(AnsiColor::Magenta.on_default()  | Effects::BOLD)
    .header(AnsiColor::Magenta.on_default() | Effects::BOLD)))]
pub struct Args {
    /// What information to query Hyprland for.
    #[arg(short, long, value_enum)]
    pub query: Query,
    /// Wait for corresponding events and re-query when they happen.
    #[arg(short, long)]
    pub subscribe: bool,
}
