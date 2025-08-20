use std::collections::HashMap;

use bevy::prelude::*;
use bevy2d_utilities::grids::hexgrid::{HexGrid, HexTile};
use rand::prelude::*;

#[derive(Resource, Default)]
pub struct TileMap {
    pub coords_to_entity: HashMap<(u32, u32), Entity>,
}

#[derive(Clone)]
pub enum MinefieldTileMarker {
    Flag,
    QuestionMark,
}

#[derive(Component, Clone)]
pub struct MinefieldTile {
    pub hidden: bool,
    pub contains_mine: bool,
    pub marked: Option<MinefieldTileMarker>,
    /// `None` is to be used before its calculated, if there are no neighboring mines the number will be `Some(0)` rather than `None`
    pub number_of_neighbor_mines: Option<u32>,
}

impl MinefieldTile {

    pub fn new(contains_mine: bool, mines: u32) -> Self {
        MinefieldTile { 
            hidden: true,
            contains_mine,
            marked: None,
            number_of_neighbor_mines: Some(mines),
        }
    }

    pub fn unhide(&mut self) {
        self.hidden = false;
    }
}

pub fn unique_rand(mine_locations: &Vec<u32>, columns: u32, rows: u32) -> u32 {
    let mine_location = rand::rng().random_range(1..(columns * rows));
    if mine_locations.contains(&mine_location)  {
        unique_rand(mine_locations, columns, rows)
    } else {
        mine_location
    }
}

pub fn build_insert_new_minefieldtile_components_system(amount_of_mines: u32, grid_id: u64, columns: u32, rows: u32) -> impl FnMut(
    Commands,
    ResMut<TileMap>,
    Query<(&HexGrid, &Children)>,
    Query<&HexTile>,
) {
    let mut mine_locations = Vec::new();
    
    for _ in 0..amount_of_mines {
        let mine_location = unique_rand(&mine_locations, columns, rows).clone();
        mine_locations.push(mine_location);
    }

    move | 
        mut commands: Commands, 
        mut tile_map: ResMut<TileMap>,
        hexgrid_q: Query<(&HexGrid, &Children)>,
        hextile_q: Query<&HexTile>,
    | {
        'outer: for (grid, tile_entities) in hexgrid_q.iter() {
            if grid.id == grid_id {

                let tile_entities = tile_entities;

                for hextile_entity in tile_entities.iter() {
                    let hextile = hextile_q.get(hextile_entity.clone()).unwrap();

                    tile_map.coords_to_entity.insert((hextile.x, hextile.y), hextile_entity);
                    
                    let hextile_order = hextile.coord_to_order(columns);
                    let contains_mine = if mine_locations.contains(&hextile_order) { true } else { false };

                    let mut mine_neighbors_counter = 0;
                    let neighbors = hextile.get_neighbors(columns, rows, grid.orientation.clone());
                    for neighbor in neighbors {
                        if mine_locations.contains(&((neighbor.0 + 1) + (neighbor.1 * columns))) {
                            mine_neighbors_counter += 1;
                        }
                    }

                    let minefieldtile = MinefieldTile::new(contains_mine, mine_neighbors_counter);

                    commands.entity(hextile_entity).insert((
                        minefieldtile,
                    ));
                }
                break 'outer;
            }
        }
    }
}
