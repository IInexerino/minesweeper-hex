use bevy::prelude::*;
use bevy2d_utilities::{
    dynamic_camera::{
        CameraMoveConfigs, 
        CameraZoomConfigs, 
        Dynamic2dCameraPlugin
    }, 
    grids::hexgrid::{
        build_change_hexgrid_textures_system, HexGrid, HexGridOrientation, TileTextures
    },
    Bevy2dUtilitiesPlugin,
};

fn main() {

    let hexgrid = HexGrid::new(
        HexGridOrientation::Vertical,
        20,
        15,
        50.,
    );

    App::new()
        .add_plugins(
            DefaultPlugins
                .set(
                    WindowPlugin{
                        primary_window: Some(Window{
                            title: "Minesweeper".into(),
                            position: WindowPosition::Centered(MonitorSelection::Primary),
                            resolution: Vec2::new(512., 512.).into(),
                            ..Default::default()
                        }),
                    ..Default::default()
                    }
                )
                .set(
                    ImagePlugin::default_nearest()
                )
        )
        .add_plugins(
            Bevy2dUtilitiesPlugin
                .set(Dynamic2dCameraPlugin {
                    enable_scroll_zoom: Some(CameraZoomConfigs::new(
                        false,
                        0.1,
                        Some(0.01), 
                        Some(5.0)
                    )),
                    enable_wasd_movment: Some(CameraMoveConfigs::new(
                        false,
                        10.0, 
                        None
                    )),
                    ..Default::default()
                }
            )
        )
        .add_systems(
            Startup, 
            ( 
                hexgrid.build_spawn_hexgrid_entity_system(Vec3::new(0.,0.,0.)),
                build_change_hexgrid_textures_system(TileTextures::Single("hex.png".into()), 1),
            ).chain()
        )
        .run();
}

