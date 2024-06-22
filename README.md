## Hyprquery | Tiny tool for getting information from Hyprland's IPC

[Hyprland](https://hyprland.org/) is a great dynamic wayland compositor. [hyprland-rs](https://crates.io/crates/hyprland) is an unofficial rust wrapper for Hyprland's IPC. The functionality of the tool in this repo can easily be replicated with a couple lines of Bash, but I wanted to do this in Rust, just because. Use case? Calling the binary with `-sq` from within [Eww](https://github.com/elkowar/eww) and updating a `deflisten` variable with it.

#### Todo:
- [x] Add a `flake.nix`
- [x] Fix the `SerdeError(Error("data did not match any variant of untagged enum Aux", line: 0, column: 0))` error that occurs with the `active-window` query (probably an issue with `hyprland-rs`)
