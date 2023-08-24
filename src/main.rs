use clap::Parser;
use hyprland::{
    event_listener::EventListenerMutable,
    shared::{HyprDataActive, HyprError},
};

mod cli;

use cli::CliArgs;
use cli::Query;

fn main() -> Result<(), HyprError> {
    let args = CliArgs::parse();

    match args.query {
        Query::CurrentWorkspace => {
            let workspace = hyprland::data::Workspace::get_active()?;
            println!("{}", workspace.id);
            if args.subscribe {
                let mut event_listener = EventListenerMutable::new();
                event_listener.add_workspace_change_handler(|a, _| println!("{a}"));
                event_listener.start_listener()?;
            }
        }
    }
    Ok(())
}
