//! # Tiny tool for getting information from Hyprland's IPC

mod cli;

use crate::cli::{Args, Query};
use clap::Parser;
use color_eyre::eyre::Result;
use hyprland::{
    data::{Client, Workspace},
    event_listener::EventListenerMutable,
    shared::{Address, HyprData, HyprDataActive, HyprDataActiveOptional},
};

fn main() -> Result<()> {
    color_eyre::install()?;
    let args = Args::parse();

    match args.query {
        Query::ActiveWorkspace => handle_active_workspace(&args)?,
        Query::Workspaces => handle_workspaces(&args)?,
        Query::ActiveWindow => handle_active_window(&args)?,
        Query::KeyboardLayout => handle_keyboard_layout(&args)?,
    }

    Ok(())
}

fn handle_active_workspace(args: &Args) -> Result<()> {
    let workspace = hyprland::data::Workspace::get_active()?;
    println!("{}", workspace.id);
    if args.subscribe {
        let mut listener = EventListenerMutable::new();
        listener.add_workspace_change_handler(|ws, _| println!("{ws}"));
        listener.add_active_monitor_change_handler(|ws, _| println!("{}", ws.workspace));
        listener.start_listener()?;
    }
    Ok(())
}

fn handle_workspaces(args: &Args) -> Result<()> {
    let handler = || {
        let workspaces = hyprland::data::Workspaces::get().unwrap();
        let mut workspaces = workspaces.collect::<Vec<Workspace>>();
        for id in 1..=10 {
            if !workspaces.iter().any(|ws| ws.id == id) {
                let monitor = workspaces.first().unwrap().monitor.clone();
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
        // FIXME: Refactor this horrible shit.
        let mut listener = EventListenerMutable::new();
        listener.add_workspace_added_handler(move |_, _| handler());
        listener.add_workspace_destroy_handler(move |_, _| handler());
        listener.add_workspace_moved_handler(move |_, _| handler());
        listener.add_window_open_handler(move |_, _| handler());
        listener.add_window_close_handler(move |_, _| handler());
        listener.add_window_moved_handler(move |_, _| handler());
        listener.add_window_moved_handler(move |_, _| handler());
        listener.start_listener()?;
    }

    Ok(())
}

fn handle_active_window(args: &Args) -> Result<()> {
    let printer = || -> Result<()> {
        if let Some(window) = Client::get_active()? {
            let active_window_json = serde_json::to_string(&window)?;
            println!("{active_window_json}");
        }
        Ok(())
    };

    printer()?;
    if args.subscribe {
        let mut listener = EventListenerMutable::new();
        listener.add_active_window_change_handler(move |_, _| {
            let _ = printer();
        });
        listener.start_listener()?;
    }

    Ok(())
}

fn handle_keyboard_layout(args: &Args) -> Result<()> {
    let printer = |layout: &str| {
        println!(
            "{}",
            match layout {
                "English (US)" => "English",
                _ => layout,
            }
        );
    };

    let keyboards = hyprland::data::Devices::get()?.keyboards;
    if let Some(laptop_kb) = keyboards
        .iter()
        .find(|kb| kb.name == "at-translated-set-2-keyboard")
    {
        printer(&laptop_kb.active_keymap);
    }

    if args.subscribe {
        let mut listener = EventListenerMutable::new();
        listener.add_keyboard_layout_change_handler(move |event, _| {
            if let Some(layout) = event.keyboard_name.split(',').nth(1) {
                printer(layout);
            }
        });
        listener.start_listener()?;
    }

    Ok(())
}
