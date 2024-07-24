//! Development tools for the game. This plugin is only enabled in dev builds.

use bevy::{dev_tools::states::log_transitions, prelude::*};
#[cfg(not(feature = "wasm"))]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::screen::Screen;

pub(super) fn plugin(app: &mut App) {
    // Print state transitions in dev builds
    app.add_systems(Update, log_transitions::<Screen>);
    #[cfg(not(feature = "wasm"))]
    app.add_plugins(WorldInspectorPlugin::new());
}
