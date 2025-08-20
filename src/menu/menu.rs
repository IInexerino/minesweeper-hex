use bevy::{app::{Plugin, Update}, ecs::{schedule::IntoScheduleConfigs, system::{Commands, Res}}, input::{keyboard::KeyCode, ButtonInput}};

use crate::game::game::{GameSystems, MainMenuSet};

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        
        app.add_systems(Update, main_menu_loop.in_set(MainMenuSet));
    }
}

pub fn main_menu_loop(
    keys: Res<ButtonInput<KeyCode>>,
    game_systems: Res<GameSystems>,
    mut commands: Commands
) {
    if keys.just_pressed(KeyCode::Digit7) {
        commands.run_system(game_systems.0["hexgrid_init_system"]);
    }
}