//! Game mechanics and content.

use bevy::prelude::*;

mod animation;
pub mod assets;
pub mod audio;
pub mod camera_movement;
pub mod enemy;
pub mod moneys;
mod movement;
pub mod player;
pub mod spawn;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        // animation::plugin,
        // audio::plugin,
        assets::plugin,
        camera_movement::plugin,
        enemy::plugin,
        // movement::plugin,
        moneys::plugin,
        spawn::plugin,
        player::plugin,
    ));
}
