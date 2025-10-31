use bevy::asset::uuid::Uuid;

use crate::types::{identifier::Identifier, tile::Tile};

#[derive(Debug, Clone)]
pub struct Player {
    pub id: Uuid,
    pub display_name: String,
    pub is_turn: bool,
    pub is_post_meld: bool,
    rack: Vec<Tile>,
}

impl Player {
    pub fn has_won(&self) -> bool {
        self.rack.is_empty()
    }

    pub fn toggle_turn(&mut self) -> bool {
        self.is_turn = !self.is_turn;
        self.is_turn
    }

    pub fn set_rack(&mut self, rack: Vec<Tile>) {
        self.rack = rack;
    }

    pub fn remove_tile_from_rack(&mut self, tile_id: Identifier) -> Option<Tile> {
        if let Some(index) =
            self.rack
                .iter()
                .enumerate()
                .find_map(|(index, tile)| match tile_id == tile.id {
                    true => Some(index),
                    _ => None,
                })
        {
            return Some(self.rack.remove(index));
        }

        return None;
    }

    pub fn get_rack(&self) -> &Vec<Tile> {
        &self.rack
    }

    pub fn get_rack_mut(&mut self) -> &mut Vec<Tile> {
        &mut self.rack
    }

    pub fn add_tile_to_rack(&mut self, tile: Tile) {
        self.rack.push(tile);
    }
}
