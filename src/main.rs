// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

use avian3d::PhysicsPlugins;
use bevy::prelude::*;
use you_are_bugs::AppPlugin;

fn main() -> AppExit {
    App::new().add_plugins((
        AppPlugin,
        PhysicsPlugins::default(),
    ))
    .run()
}
