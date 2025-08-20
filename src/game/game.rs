use std::{collections::VecDeque};

use bevy::{ app::{Plugin, Update}, asset::AssetServer, ecs::{entity::Entity, hierarchy::Children, observer::Trigger, resource::Resource, schedule::{IntoScheduleConfigs, SystemSet}, system::{Commands, Query, Res, RunSystemOnce}, world::World}, math::Vec3, picking::{ events::{Click, Pointer}, pointer::PointerButton, Pickable}, sprite::Sprite, state::{app::AppExtStates, condition::in_state, state::{OnEnter, States}} };
use bevy2d_utilities::grids::hexgrid::{build_change_hexgrid_textures_system, HexGrid, HexGridOrientation, HexTile, TileTextures};

use crate::game::mines::{build_insert_new_minefieldtile_components_system, MinefieldTile, MinefieldTileMarker, TileMap};

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::app::App) {

        app.init_resource::<TileMap>();
        
        // STATES
        app.insert_state(AppState::MainMenu);
        app.insert_state(PauseMenuState::Running);

        // SYSTEM SETS CONFIGS
        app.configure_sets(Update, (
            MainMenuSet.run_if(in_state(AppState::MainMenu)),
            PauseMenuSet
                .run_if(in_state(AppState::InGame))
                .run_if(in_state(PauseMenuState::Paused)),
            GamePlaySet
                .run_if(in_state(AppState::InGame))
                .run_if(in_state(PauseMenuState::Running))
        ));
        
        // DEFAULT SETTINGS
        app.insert_resource(GridSettings{
            difficulty: GameDifficulty::Easy,
            grid: HexGrid::new(HexGridOrientation::Vertical, 10, 10, 85.),
        });

        
        app.add_systems(OnEnter(AppState::InGame), hexgrid_init);
    }
}

#[derive(States, Clone, Debug, Hash, Eq, PartialEq)]
pub enum AppState {
    MainMenu,
    InGame,
}

#[derive(States, Default, Clone, Debug, Hash, Eq, PartialEq)]
pub enum PauseMenuState {
    #[default]
    Running,
    Paused
}


#[derive(Resource)]
pub struct GridSettings {
    pub grid: HexGrid,
    pub difficulty: GameDifficulty
}

#[derive(Clone)]
pub enum GameDifficulty {
    Easy,
    Medium,
    Hard,
    VeryHard,
    Impossible
}


#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MainMenuSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PauseMenuSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct GamePlaySet;

fn reveal_tile_floodfill(
    start_entity: Entity,
    query: &mut Query<(Entity, &HexTile, &mut MinefieldTile, &mut Sprite)>,
    asset_server: &Res<AssetServer>,
    grid: &HexGrid,
    tilemap: &TileMap,
) {
    let mut queue = VecDeque::new();
    queue.push_back(start_entity);

    while let Some(entity) = queue.pop_front() {
        if let Ok((_, hex_tile,  mut mine_tile, mut sprite)) = query.get_mut(entity) {
            if !mine_tile.hidden {
                continue; // already revealed
            }

            // reveal tile
            mine_tile.unhide();
            mine_tile.marked = None;

            if let Some(n) = mine_tile.number_of_neighbor_mines {
                if mine_tile.contains_mine {
                    sprite.image = asset_server.load("hex_revealed/hex_exploded.png");
                } else {
                    let tex = if n == 0 {
                        asset_server.load("hex_revealed/hex_0.png")
                    } else {
                        asset_server.load(format!("hex_revealed/hex_{}.png", n))
                    };
                    sprite.image = tex;

                    if n == 0 {
                        for (nx, ny) in hex_tile.get_neighbors(grid.columns, grid.rows, grid.orientation) {
                            if let Some(&neighbor_entity) = tilemap.coords_to_entity.get(&(nx, ny)) {
                                queue.push_back(neighbor_entity);
                            }
                        }
                    }
                }
            }
        }
    }
}

fn apply_click_to_tile(
    target: Entity,
    button: PointerButton,
    asset_server: &Res<AssetServer>,
    fill_query: &mut Query<(Entity, &HexTile, &mut MinefieldTile, &mut Sprite)>,
    tilemap: &TileMap,
    grid_settings: &GridSettings,
) {
    if let Ok((_e, _hex, mut minefield_tile, mut sprite)) = fill_query.get_mut(target) {
        match button {
            PointerButton::Primary => {
                // ignore flagged tiles
                if matches!(minefield_tile.marked, Some(MinefieldTileMarker::Flag)) {
                    return;
                }

                if minefield_tile.hidden {
                    if !minefield_tile.contains_mine {
                        // unchanged: your flood fill
                        reveal_tile_floodfill(
                            target,
                            fill_query,
                            asset_server,
                            &grid_settings.grid,
                            tilemap,
                        );
                    } else {
                        // game over
                        sprite.image = asset_server.load("hex_revealed/hex_exploded.png");
                        minefield_tile.unhide();
                    }
                }
            }
            PointerButton::Secondary => {
                if minefield_tile.hidden {
                    match minefield_tile.marked {
                        None => {
                            sprite.image = asset_server.load("hex_red_marker.png");
                            minefield_tile.marked = Some(MinefieldTileMarker::Flag);
                        }
                        Some(MinefieldTileMarker::Flag) => {
                            sprite.image = asset_server.load("hex_question_marker.png");
                            minefield_tile.marked = Some(MinefieldTileMarker::QuestionMark);
                        }
                        Some(MinefieldTileMarker::QuestionMark) => {
                            sprite.image = asset_server.load("hex.png");
                            minefield_tile.marked = None;
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

fn add_picking_observers(grid_id: u64) -> impl FnMut(
    Commands,
    Query<(&HexGrid, &Children)>
) {

    move |    
        mut commands: Commands,
        hexgrid_q: Query<(&HexGrid, &Children)>
    | {
        for (hexgrid, children) in hexgrid_q.iter() {

            if hexgrid.id == grid_id {
                println!("Grid id found");
                for hextile_entity in children {
                    commands.entity(*hextile_entity)
                        .insert((
                            Pickable::default(),
                        ))
                        .observe( |
                            trigger: Trigger<Pointer<Click>>,
                            asset_server: Res<AssetServer>,
                            mut fill_query: Query<(Entity, &HexTile, &mut MinefieldTile, &mut Sprite)>,
                            tilemap: Res<TileMap>,
                            grid_settings: Res<GridSettings>,
                        | {
                            apply_click_to_tile(
                                trigger.target(),
                                trigger.event().event.button,
                                &asset_server,
                                &mut fill_query,
                                &tilemap,
                                &grid_settings,
                            );
                        });
                } 
            break
        } else {
            //debug
            println!("Grid id not found")
            }
        }
    }
}

pub fn hexgrid_init(
    grid_settings: Res<GridSettings>,
    mut commands: Commands,
) {

    let grid = grid_settings.grid.clone();
    let difficulty = grid_settings.difficulty.clone();


    commands.queue(move |world: &mut World| {

        let _ = world.run_system_once(grid.clone().build_spawn_hexgrid_entity_system(Vec3::new(0.,0.,0.)));
        let _ = world.run_system_once(build_change_hexgrid_textures_system(TileTextures::Single("hex.png".into()), grid.id));

        let number_of_tiles = grid.columns * grid.rows;

        let amount_of_mines = match difficulty {
            GameDifficulty::Easy => {
                number_of_tiles / 10
            },
            GameDifficulty::Medium => {
                number_of_tiles / 7
            },
            GameDifficulty::Hard => {
                number_of_tiles / 5
            },
            GameDifficulty::VeryHard => {
                number_of_tiles / 4
            },
            GameDifficulty::Impossible => {
                number_of_tiles / 3
            },
        };

        let _ = world.run_system_once(build_insert_new_minefieldtile_components_system(amount_of_mines, grid.id, grid.columns, grid.rows));

        let _ = world.run_system_once(add_picking_observers(grid.id));

        let _ = world.run_system_once(reveal_one_init);

    });

}

fn reveal_one_init(
    asset_server: Res<AssetServer>,
    mut fill_query: Query<(Entity, &HexTile, &mut MinefieldTile, &mut Sprite)>,
    tilemap: Res<TileMap>,
    grid_settings: Res<GridSettings>,
) {

    let entity = fill_query.iter().find( | (_, _, minefield, _) | minefield.number_of_neighbor_mines == Some(0))
    .map(| (entity, _, _, _) | entity).unwrap();

    apply_click_to_tile(
        entity,
        PointerButton::Primary, // or Secondary
        &asset_server,
        &mut fill_query,
        &tilemap,
        &grid_settings,
    );
}