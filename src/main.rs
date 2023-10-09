use clap::Parser;
use color_eyre::Result;
use hyprland::{
    data::Workspace,
    event_listener::EventListenerMutable,
    shared::{Address, HyprData, HyprDataActive, HyprDataActiveOptional},
};

mod cli;

use cli::Args;
use cli::Query;

fn main() -> Result<()> {
    color_eyre::install()?;
    let args = Args::parse();

    match args.query {
        Query::ActiveWorkspace => {
            let workspace = hyprland::data::Workspace::get_active()?;
            println!("{}", workspace.id);
            if args.subscribe {
                let mut event_listener = EventListenerMutable::new();
                event_listener.add_workspace_change_handler(|ws, _| println!("{ws}"));
                event_listener.start_listener()?;
            }
        }
        Query::Workspaces => {
            let handler = || {
                let workspaces = hyprland::data::Workspaces::get().unwrap();
                let mut workspaces = workspaces.collect::<Vec<Workspace>>();
                for id in 1..=10 {
                    if !workspaces.iter().any(|ws| ws.id == id) {
                        let monitor = workspaces.get(0).unwrap().monitor.clone();
                        workspaces.push(Workspace {
                            id,
                            name: id.to_string(),
                            monitor,
                            windows: 0,
                            fullscreen: false,
                            last_window: Address::new(String::new()),
                            last_window_title: String::new(),
                        });
                    }
                }
                workspaces.sort_unstable_by_key(|ws| ws.id);
                let workspaces_json = serde_json::to_string(&workspaces).unwrap();
                println!("{workspaces_json}");
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
                event_listener.start_listener()?;
            }
        }
        Query::ActiveWindow => {
            let handler = || {
                if let Some(window) =
                    // BUG: always panicks for some reason
                    hyprland::data::Client::get_active()
                        .expect("could not get active window")
                {
                    let active_window_json = serde_json::to_string(&window).unwrap();
                    println!("{active_window_json}");
                }
            };

            handler();
            if args.subscribe {
                let mut event_listener = EventListenerMutable::new();
                event_listener.add_active_window_change_handler(move |_, _| handler());
                event_listener.start_listener()?;
            }
        }
        Query::KeyboardLayout => {
            let keyboards = hyprland::data::Devices::get()?.keyboards;
            if let Some(default_laptop_keyboard) = keyboards
                .iter()
                .find(|keyboard| keyboard.name == "at-translated-set-2-keyboard")
            {
                println!("{}", default_laptop_keyboard.active_keymap);
            }

            if args.subscribe {
                let mut event_listener = EventListenerMutable::new();
                event_listener.add_keyboard_layout_change_handler(|layout_event, _| {
                    if let Some(extracted_layout_name) =
                        layout_event.keyboard_name.split(',').nth(1)
                    {
                        println!("{extracted_layout_name}");
                    }
                });
                event_listener.start_listener()?;
            }
        }
    }
    Ok(())
}
