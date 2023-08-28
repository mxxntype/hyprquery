use clap::Parser;
use hyprland::{
    data::Workspace,
    event_listener::EventListenerMutable,
    shared::{HyprData, HyprDataActive, HyprDataActiveOptional, HyprError},
};

mod cli;

use cli::CliArgs;
use cli::Query;

fn main() -> Result<(), HyprError> {
    let args = CliArgs::parse();

    match args.query {
        Query::ActiveWorkspace => {
            let workspace = hyprland::data::Workspace::get_active()?;
            println!("{}", workspace.id);
            if args.subscribe {
                let mut event_listener = EventListenerMutable::new();
                event_listener.add_workspace_change_handler(|a, _| println!("{a}"));
                event_listener.start_listener()?
            }
        }
        Query::Workspaces => {
            let handler = || {
                let workspaces = hyprland::data::Workspaces::get().unwrap();
                let mut workspaces = workspaces.collect::<Vec<Workspace>>();
                workspaces.sort_unstable_by_key(|k| k.id);
                let j = serde_json::to_string(&workspaces).unwrap();
                println!("{}", j);
            };

            handler();
            if args.subscribe {
                // TODO: Refactor this horrible shit
                let mut event_listener = EventListenerMutable::new();
                event_listener.add_workspace_added_handler(move |_, _| handler());
                event_listener.add_workspace_destroy_handler(move |_, _| handler());
                event_listener.add_workspace_moved_handler(move |_, _| handler());
                event_listener.add_window_open_handler(move |_, _| handler());
                event_listener.add_window_close_handler(move |_, _| handler());
                event_listener.add_window_moved_handler(move |_, _| handler());
                event_listener.add_window_moved_handler(move |_, _| handler());
                event_listener.start_listener()?
            }
        }
        Query::ActiveWindow => {
            let handler = || {
                let active_window = hyprland::data::Client::get_active().unwrap();
                if let Some(window) = active_window {
                    let active_window_json = serde_json::to_string(&window).unwrap();
                    println!("{}", active_window_json);
                }
            };

            handler();
            let mut event_listener = EventListenerMutable::new();
            event_listener.add_active_window_change_handler(move |_, _| handler());
            event_listener.start_listener()?
        }
    }
    Ok(())
}
