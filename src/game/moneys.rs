use bevy::prelude::*;

use crate::{dev_tools::DevCyclicTimer, screen::Screen};

use super::spawn::level::PlayerMoneyText;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<PlayerMoney>();
    app.add_systems(Update, (grow_money,update_player_money_text).run_if(in_state(Screen::Playing)));
}

#[derive(Default, Resource, Debug)]
pub struct PlayerMoney(pub i32);

fn grow_money(
    dev_timer: ResMut<DevCyclicTimer>,
    mut money_res: ResMut<PlayerMoney>,
) {
    if dev_timer.timer.finished() {
        money_res.0 += 5;
    }
}

fn update_player_money_text(
    mut player_money_text_query: Query<&mut Text, With<PlayerMoneyText>>,
    player_money_res: Res<PlayerMoney>,
) {
    for mut each in player_money_text_query.iter_mut() {
        each.sections[0].value = format!("MONEY: {}",player_money_res.0.to_string());
    }
}
