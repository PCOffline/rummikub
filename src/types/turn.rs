use crate::types::{identifier::Identifier, player::Player};

// Take ownership only of tiles that are moving around, or a set that is created
// Might decide to change this to be identifiers only, if for example:
// impl Table { ... fn create_set(&self, set: Set) { self.sets.push(set) } }
// will not be called using the move, but directly
#[derive(Clone, Debug)]
pub enum Move {
    Draw {
        tile_id: Identifier,
    },
    Create {
        set_id: Identifier,
    },
    Add {
        set_id: Identifier,
        tile_id: Identifier,
    },
    Move {
        origin_set_id: Identifier,
        target_set_id: Identifier,
        tile_id: Identifier,
        index: usize,
    },
    Remove {
        set_id: Identifier,
        tile_id: Identifier,
    },
}

impl Move {
    pub fn is_draw(&self) -> bool {
        match self {
            Self::Draw { tile_id: _ } => true,
            _ => false,
        }
    }

    pub fn is_create(&self) -> bool {
        match self {
            Self::Create { set_id: _ } => true,
            _ => false,
        }
    }
}

pub enum TurnError {
    IllegalMove,
}

// We don't validate single moves anymore. Instead, we validate only the result
// at the end of the turn.
pub struct Turn<'a> {
    pub player: &'a mut Player,
    pub moves: Vec<Move>,
}

impl<'a> Turn<'a> {
    /// Assumptions:
    /// 1. Player exists
    /// 2. Tile belongs to player up until this move
    /// 3. It is the player's turn
    pub fn add_move(&mut self, new_move: Move) -> Result<(), TurnError> {
        if self
            .moves
            .get(1)
            .is_some_and(|mv| mv.is_draw() || new_move.is_draw())
        {
            return Err(TurnError::IllegalMove);
        }

        if !self.player.is_post_meld && !new_move.is_create() && !new_move.is_draw() {
            return Err(TurnError::IllegalMove);
        }

        if self.player.has_won() {
            return Err(TurnError::IllegalMove);
        }

        Ok(())
    }

    pub fn clear(&mut self) {
        self.moves.clear();
    }

    pub fn length(&self) -> usize {
        self.moves.len()
    }

    pub fn undo(&mut self) -> Option<Move> {
        self.moves.pop()
    }
}
