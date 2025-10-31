use bevy::asset::uuid::Uuid;
use std::fmt::Debug;

const JOKER_TILE_VALUE: u8 = 0;
const MIN_TILE_VALUE: u8 = 1;
const MAX_TILE_VALUE: u8 = 13;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Color {
    Blue,
    Red,
    Orange,
    Black,
    Joker,
}

pub enum TileError {
    IllegalTile,
}

impl Debug for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let color_to_string = match self {
            Color::Blue => "blue",
            Color::Red => "red",
            Color::Orange => "orange",
            Color::Black => "black",
            Color::Joker => "joker",
        };

        write!(f, "{color_to_string}")?;
        Ok(())
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Tile {
    pub id: Uuid,
    pub value: u8,
    pub color: Color,
}

impl Tile {
    pub fn new(value: u8, color: Color) -> Result<Self, TileError> {
        let new = Self {
            id: Uuid::new_v4(),
            value,
            color,
        };

        match new.is_valid() {
            true => Ok(new),
            false => Err(TileError::IllegalTile),
        }
    }

    pub fn is_joker(&self) -> bool {
        self.value == JOKER_TILE_VALUE && self.color == Color::Joker
    }

    pub fn is_valid(&self) -> bool {
        self.is_joker()
            || (self.color != Color::Joker
                && self.value >= MIN_TILE_VALUE
                && self.value <= MAX_TILE_VALUE)
    }
}
