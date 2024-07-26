//! Game mechanics and content.

use bevy::prelude::*;

mod animation;
pub mod assets;
pub mod audio;
pub mod camera_movement;
pub mod moneys;
mod movement;
pub mod spawn;
pub mod player;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        animation::plugin,
        audio::plugin,
        assets::plugin,
        camera_movement::plugin,
        movement::plugin,
        moneys::plugin,
        spawn::plugin,
        player::plugin,
    ));
}
