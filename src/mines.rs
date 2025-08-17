use bevy::prelude::*;

// vertical hextiles
const HEXTILE_WIDTH: f32 = 30.;
const HEXTILE_HEIGHT: f32 = HEXTILE_WIDTH * 0.866;
const COLUMNS: u32 = 10;
const ROWS: u32 = 10;

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
    pub number_of_neighbor_mines: Option<u32>,
}

impl MinefieldTile {

    pub fn new() -> Self {
        MinefieldTile { 
            hidden: true,
            contains_mine: false,
            marked: None,
            number_of_neighbor_mines: None,
        }
    }

    pub fn hide(&mut self) {
        self.hidden = true;
    }

    pub fn unhide(&mut self) {
        self.hidden = false;
    }
}