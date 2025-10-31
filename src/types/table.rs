use crate::types::{
    identifier::Identifier,
    player::Player,
    set::Set,
    tile::Tile,
    turn::{Move, Turn},
};
use bevy::{asset::uuid::Uuid, platform::collections::HashMap};
use bevy_ecs::prelude::*;

pub struct Table {
    players: [Player; 4],
    pool: Vec<Tile>,
    sets: HashMap<Uuid, Set>,
}

pub enum TableError {
    PoolEmpty,
    WrongTurn,
    UnknownSet,
    MissingTile,
    IllegalMove,
    MeldThreshold,
}

impl Table {
    fn draw(&self, player: &mut Player) -> Result<(), TableError> {
        if self.pool.is_empty() {
            return Err(TableError::PoolEmpty);
        }

        if player.has_won() || !player.is_turn {
            return Err(TableError::WrongTurn);
        }

        panic!("TODO: Add rng");
    }

    pub fn get_current_turn_player(&self) -> &Player {
        self.players.iter().find(|player| player.is_turn).unwrap()
    }

    pub fn get_current_turn_player_index(&self) -> (usize, &Player) {
        self.players
            .iter()
            .enumerate()
            .find(|(_, player)| player.is_turn)
            .unwrap()
    }

    pub fn next_turn(&mut self) -> &Player {
        // TODO: Is this really the best way to end a turn?
        // Shouldn't it be triggered with the context of the current player?
        let (index, _) = self.get_current_turn_player_index();
        if index == self.players.len() - 1 {
            self.players[0].toggle_turn();
            &self.players[0]
        } else {
            self.players[index + 1].toggle_turn();
            &self.players[index + 1]
        }
    }

    fn get_set(&self, set_id: Identifier) -> Option<&Set> {
        self.sets.get::<Uuid>(&set_id.into())
    }

    fn get_set_mut(&mut self, set_id: Identifier) -> Option<&mut Set> {
        self.sets.get_mut::<Uuid>(&set_id.into())
    }

    pub fn move_tile_to_set(
        &mut self,
        origin_set_id: Identifier,
        target_set_id: Identifier,
        tile_id: Identifier,
        index: usize,
    ) -> Result<(), TableError> {
        if !self
            .sets
            .contains_key::<Uuid>(&target_set_id.clone().into())
        {
            return Err(TableError::UnknownSet);
        }

        let tile = self.remove_tile_from_set(origin_set_id, tile_id)?;

        let Some(target_set) = self.get_set_mut(target_set_id) else {
            return Err(TableError::UnknownSet);
        };

        target_set
            .add_tile(tile, index)
            .map_err(|_| TableError::IllegalMove)?;

        Ok(())
    }

    pub fn add_tile_to_set(
        &mut self,
        player: &mut Player,
        set_id: Identifier,
        tile_id: Identifier,
        index: usize,
    ) -> Result<(), TableError> {
        let Some(tile) = player.remove_tile_from_rack(tile_id) else {
            return Err(TableError::MissingTile);
        };

        if let Some(set) = self.get_set_mut(set_id) {
            set.add_tile(tile, index)
                .map_err(|_| TableError::IllegalMove)
        } else {
            Err(TableError::UnknownSet)
        }
    }

    pub fn remove_tile_from_set(
        &mut self,
        set_id: Identifier,
        tile_id: Identifier,
    ) -> Result<Tile, TableError> {
        let Some(set) = self.get_set_mut(set_id) else {
            return Err(TableError::UnknownSet);
        };

        let set_id = set.id;

        let Some((index, tile)) = set.remove_tile_with_index(tile_id) else {
            return Err(TableError::MissingTile);
        };

        if index > 0 && index < set.len() - 1 {
            let tiles = set.get_tiles_mut();
            let split_tiles = tiles.split_off(index);
            let mut new_set = Set::new();
            new_set.set_tiles(split_tiles);
            self.sets.insert(new_set.id, new_set);
        } else if set.len() == 0 {
            self.sets.remove::<Uuid>(&set_id.into());
        }

        // TODO: Determine what to do with this shi
        // let mut new_set = Set::new();
        // new_set
        //     .add_tile(tile, 0)
        //     .expect("Cannot create new set from a removed tile");
        // self.sets.insert(new_set.id, new_set);

        Ok(tile)
    }

    pub fn create_set(
        &mut self,
        player: &mut Player,
        tile_ids: Vec<Identifier>,
    ) -> Result<(), TableError> {
        let tiles_len = tile_ids.len();
        let tiles: Vec<_> = tile_ids
            .into_iter()
            .filter_map(|tile_id| player.remove_tile_from_rack(tile_id))
            .collect();

        if tiles.len() < tiles_len {
            return Err(TableError::MissingTile);
        }

        let mut set = Set::new();
        set.set_tiles(tiles);

        self.sets.insert(set.id, set);

        Ok(())
    }

    pub fn undo_move(&mut self, player: &mut Player, mv: Move) -> Result<(), TableError> {
        match mv {
            Move::Create { set_id } => {
                let Some(mut set) = self.sets.remove::<Uuid>(&set_id.into()) else {
                    return Err(TableError::UnknownSet);
                };
                player.get_rack_mut().append(set.get_tiles_mut());
            }
            Move::Draw { tile_id } => {
                let Some(tile) = player.remove_tile_from_rack(tile_id) else {
                    return Err(TableError::MissingTile);
                };

                self.pool.push(tile);
            }
            Move::Add { set_id, tile_id } => {
                let Some(set) = self.get_set_mut(set_id) else {
                    return Err(TableError::UnknownSet);
                };

                let Some(tile) = set.remove_tile(tile_id) else {
                    return Err(TableError::MissingTile);
                };

                player.add_tile_to_rack(tile);
            }
            Move::Remove {
                set_id: _,
                tile_id: _,
            } => {
                todo!("Um where do I take the tile from?");
            }
            Move::Move {
                origin_set_id,
                target_set_id,
                tile_id,
                index,
            } => {
                self.move_tile_to_set(target_set_id, origin_set_id, tile_id, index)?;
            }
        };

        Ok(())
    }

    pub fn end_turn(&mut self, turn: Turn) -> Result<(), TableError> {
        if !turn.player.is_post_meld {
            return turn
                .moves
                .into_iter()
                .filter_map(|mv| match mv {
                    Move::Create { set_id } => self
                        .get_set(set_id)
                        .map(|set| set.validate().map(|_| set).or(Err(TableError::IllegalMove)))
                        .map(|set_result| set_result.map(|set| set.get_sum() as u16))
                        .or(Some(Err(TableError::UnknownSet))),
                    _ => Some(Err(TableError::IllegalMove)),
                })
                .collect::<Result<Vec<u16>, TableError>>()
                .map(|sum_vec| sum_vec.iter().sum::<u16>())
                .map(|sum| match sum > 30 {
                    true => Ok(()),
                    false => Err(TableError::MeldThreshold),
                })?;
        }

        self.sets
            .values()
            .map(|set| set.validate().or(Err(TableError::IllegalMove)))
            .collect()
    }
}
