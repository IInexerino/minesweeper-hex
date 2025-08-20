use bevy::{app::{Plugin, Update}, ecs::{schedule::IntoScheduleConfigs, system::{Res, ResMut}}, input::{keyboard::KeyCode, ButtonInput}, state::state::NextState};

use crate::game::game::{AppState, MainMenuSet};

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        
        app.add_systems(Update, main_menu_loop.in_set(MainMenuSet));
    }
}

pub fn main_menu_loop(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_app: ResMut<NextState<AppState>>,
) {
    // configure grid settings
    if keys.just_pressed(KeyCode::Digit7) {
        next_app.set(AppState::InGame);
    }
}