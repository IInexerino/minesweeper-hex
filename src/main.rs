
mod game;
mod menu;

use bevy::{ prelude::*, window::{WindowMode, WindowResolution} };      
use bevy_inspector_egui::{ bevy_egui::EguiPlugin, quick::WorldInspectorPlugin };   

use bevy2d_utilities::{ dynamic_camera::{ CameraZoomConfigs,  Dynamic2dCameraPlugin }, Bevy2dUtilitiesPlugin };   

use game::game::{ GamePlugin};

use crate::menu::menu::MenuPlugin;

fn main() {
    let mut app = App::new();




    // PLUGINS
    app.add_plugins(DefaultPlugins.set(
        WindowPlugin{
                    primary_window: Some(Window{
                        title: "Minesweeper".into(),
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        resolution: WindowResolution::new(1920., 1080.),
                        mode: WindowMode::Fullscreen(MonitorSelection::Primary, VideoModeSelection::Current),
                        ..Default::default()
                }), ..Default::default()
            }
        ).set( ImagePlugin::default_nearest() )
    );
    app.add_plugins(EguiPlugin { enable_multipass_for_primary_context: true });
    app.add_plugins(WorldInspectorPlugin::new());
    app.add_plugins(Bevy2dUtilitiesPlugin.set(
        Dynamic2dCameraPlugin {
            enable_scroll_zoom: Some(CameraZoomConfigs::new(true,0.05,Some(0.01),Some(5.0))),
            ..Default::default()
        }
    ));

    app.add_plugins(GamePlugin);
    app.add_plugins(MenuPlugin);


    // We start the game in the main menu
    // In the main menu we configure our difficulty, and the grid size
    // When the player presses some button to play a game, the state changes to loading_screen




    




    app.run();
}