use bevy::asset::uuid::Uuid;
use std::{cmp::max, collections::HashSet};

use crate::types::identifier::Identifier;

use super::tile::*;

const MIN_LENGTH: usize = 3;
const MAX_GROUP_LENGTH: usize = 4;
const MAX_RUN_LENGTH: usize = 13;

const ASSERT_NON_JOKER_TILE_MESSAGE: &'static str = "A set should have a tile that isn't a joker";

#[derive(Debug)]
pub enum SetError {
    MinLength,
    MaxLength,
    /// When two tiles don't make any sensible set (i.e. 9, 11, 12) or (8, joker, 12)
    BadTile,
    OutOfBounds,
}

#[derive(Clone, Debug)]
pub enum SetOrder {
    /// A group is a set of either three or four tiles of the same number in different colors.
    Group,
    /// A run is a set of three or more consecutive numbers all in the same color.
    /// The number 1 is always played as the lowest number, it cannot follow the number 13.
    Run,
}

#[derive(Debug, Clone)]
pub struct Set {
    pub id: Uuid,
    tiles: Vec<Tile>,
}

impl Set {
    pub fn new() -> Self {
        let tiles = Vec::with_capacity(MAX_RUN_LENGTH);

        Self {
            id: Uuid::new_v4(),
            tiles: tiles,
        }
    }

    pub fn set_tiles(&mut self, tiles: Vec<Tile>) {
        self.tiles = tiles;
    }

    // TODO: Cache these
    fn get_tiles_as_run(&self) -> Result<Vec<Tile>, SetError> {
        if self.tiles.len() < MIN_LENGTH {
            return Err(SetError::MinLength);
        }

        if self
            .tiles
            .iter()
            .skip(1)
            .any(|tile| tile.value != self.tiles[0].value)
        {
            return Err(SetError::BadTile);
        }

        let mut result = self.tiles.windows(2).map(|pair| {
            if pair[0].is_joker() {
                let curr = &pair[0];
                let other = &pair[1];

                return [
                    Tile {
                        id: curr.id.clone(),
                        value: other.value - 1,
                        color: other.color,
                    },
                    other.clone(),
                ];
            } else if pair[1].is_joker() {
                let other = &pair[0];
                let curr = &pair[1];

                return [
                    other.clone(),
                    Tile {
                        id: curr.id.clone(),
                        value: other.value + 1,
                        color: other.color,
                    },
                ];
            }

            [pair[0].clone(), pair[1].clone()]
        });

        if result.any(|pair| pair[1].value - pair[0].value != 1) {
            return Err(SetError::BadTile);
        }

        if result.any(|pair| !pair[0].is_valid() || !pair[1].is_valid()) {
            return Err(SetError::BadTile);
        }

        Ok(result.flatten().collect())
    }

    // TODO: Cache these
    fn get_tiles_as_group(&self) -> Result<Vec<Tile>, SetError> {
        if self.tiles.len() < MIN_LENGTH {
            return Err(SetError::MinLength);
        }

        if self.tiles.len() > MAX_GROUP_LENGTH {
            return Err(SetError::MaxLength);
        }

        let mut seen = HashSet::<Color>::new();

        if self
            .tiles
            .iter()
            .any(|tile| !tile.is_joker() && !seen.insert(tile.color))
        {
            return Err(SetError::BadTile);
        }

        let value = self
            .tiles
            .iter()
            .find(|tile| !tile.is_joker())
            .expect(ASSERT_NON_JOKER_TILE_MESSAGE)
            .value;

        return Ok(self
            .tiles
            .iter()
            .map(|tile| {
                if tile.is_joker() {
                    Tile {
                        id: tile.id.clone(),
                        color: tile.color,
                        value,
                    }
                } else {
                    tile.clone()
                }
            })
            .collect());
    }

    // TOOD: Do we want get_order?
    pub fn get_order(&self) -> Option<SetOrder> {
        if self.get_tiles_as_group().is_ok() {
            return Some(SetOrder::Group);
        } else if self.get_tiles_as_run().is_ok() {
            return Some(SetOrder::Run);
        }

        return None;
    }

    fn sum_tiles(tiles: &Vec<Tile>) -> u8 {
        tiles.iter().map(|tile| tile.value).sum()
    }

    pub fn get_sum(&self) -> u8 {
        if self.tiles.iter().any(|tile| tile.is_joker()) {
            let run = self.get_tiles_as_run().unwrap_or(vec![]);
            let group = self.get_tiles_as_group().unwrap_or(vec![]);

            return max(Self::sum_tiles(&run), Self::sum_tiles(&group));
        }

        self.tiles.iter().map(|tile| tile.value).sum()
    }

    pub fn get_tiles(&self) -> &Vec<Tile> {
        &self.tiles
    }

    pub fn get_tiles_mut(&mut self) -> &mut Vec<Tile> {
        &mut self.tiles
    }

    pub fn add_tile(&mut self, tile: Tile, index: usize) -> Result<(), SetError> {
        if index > self.tiles.len() {
            return Err(SetError::OutOfBounds);
        }

        self.tiles.insert(index, tile);
        Ok(())
    }

    pub fn remove_tile_with_index(&mut self, tile_id: Identifier) -> Option<(usize, Tile)> {
        if let Some(index) = self
            .tiles
            .iter()
            .enumerate()
            .find(|(_, tile)| tile_id == tile.id)
            .map(|(index, _)| index)
        {
            Some((index, self.tiles.remove(index)))
        } else {
            None
        }
    }

    pub fn remove_tile(&mut self, tile_id: Identifier) -> Option<Tile> {
        self.remove_tile_with_index(tile_id).map(|(_, tile)| tile)
    }

    pub fn validate(&self) -> Result<(), SetError> {
        self.get_tiles_as_run().or(self.get_tiles_as_group()).map(|_| ())
    }

    pub fn len(&self) -> usize {
        self.tiles.len()
    }
}
