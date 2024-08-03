pub mod systems;

use bevy::prelude::*;
use systems::{enemy_movement, spawn_enemy, spawn_trig};

use crate::screen::Screen;

pub(super) fn plugin(app: &mut App) {
    app
        .observe(spawn_enemy)
        .add_systems(Update, (
            enemy_movement, 
            spawn_trig,
        ).run_if(in_state(Screen::Playing)))
    ;

}

