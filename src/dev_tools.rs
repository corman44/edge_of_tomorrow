//! Development tools for the game. This plugin is only enabled in dev builds.

use std::time::Duration;

use bevy::{dev_tools::states::log_transitions, input::mouse::MouseWheel, prelude::*};
#[cfg(not(feature = "wasm"))]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::{screen::Screen, AppSet};

pub(super) fn plugin(app: &mut App) {
    // Print state transitions in dev builds
    app.add_systems(Update, log_transitions::<Screen>);
    // app.add_systems(Update, scroll_events);
    #[cfg(not(feature = "wasm"))]
    app.add_plugins(WorldInspectorPlugin::new());
    app.insert_resource(DevCyclicTimer {
        timer: Timer::new(Duration::from_secs(1), TimerMode::Repeating),
    });
    app.add_systems(Update, tick_dev_timer.in_set(AppSet::TickTimers));
}

fn scroll_events(
    mut evr_scroll: EventReader<MouseWheel>,
) {
    use bevy::input::mouse::MouseScrollUnit;
    for ev in evr_scroll.read() {
        match ev.unit {
            MouseScrollUnit::Line => {
                println!("Scroll (line units): vertical: {}, horizontal: {}", ev.y, ev.x);
            }
            MouseScrollUnit::Pixel => {
                println!("Scroll (pixel units): vertical: {}, horizontal: {}", ev.y, ev.x);
            }
        }
    }
}

#[derive(Resource)]
pub struct DevCyclicTimer {
    pub timer: Timer,
}

fn tick_dev_timer(
    time: Res<Time>,
    mut dev_timer: ResMut<DevCyclicTimer>,
) {
    dev_timer.timer.tick(time.delta());
}
