use bevy::prelude::*;
use plugin::CharacterControllerPlugin;
use systems::spawn_bullet;

pub mod plugin;
pub mod systems;

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_bullet);
    app.add_plugins((
        systems::plugin,
        CharacterControllerPlugin
    ));

}