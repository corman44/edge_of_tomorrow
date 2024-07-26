// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

use bevy::prelude::*;
use you_are_bugs::{genetics::*, AppPlugin};

fn main() {
    // App::new().add_plugins(AppPlugin).run()
    run_simulation();
}
