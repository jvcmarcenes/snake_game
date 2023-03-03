
use Direction::*;
use bevy::prelude::KeyCode;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Direction {
    North, East, South, West
}

impl Direction {
    pub fn rev(self) -> Direction {
        match self {
            North => South,
            East  => West,
            South => North,
            West  => East,
        }
    }
    
    pub fn rotate_right(self) -> Direction {
        match self {
            North => East,
            East  => South,
            South => West,
            West  => North,
        }
    }

    pub fn rotate_left(self) -> Direction {
        match self {
            North => West,
            East  => North,
            South => East,
            West  => South,
        }
    }
}

impl TryFrom<KeyCode> for Direction {
    type Error = ();
    fn try_from(key: KeyCode) -> Result<Direction, Self::Error> {
        use KeyCode::*;
        use Direction::*;

        match key {
            W | Up     => Ok(North),
            A | Left   => Ok(West),
            S | Down   => Ok(South),
            D | Right  => Ok(East),
            _ => Err(()),
        }
    }
}
