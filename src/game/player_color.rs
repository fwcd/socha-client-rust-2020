use std::{convert::TryFrom, str::FromStr};
use crate::util::{SCError, SCResult};

/// A player color in the game.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PlayerColor {
    Red,
    Blue
}

impl PlayerColor {
    pub fn opponent(self) -> Self {
        match self {
            Self::Red => Self::Blue,
            Self::Blue => Self::Red
        }
    }
}

impl FromStr for PlayerColor {
    type Err = SCError;

    fn from_str(raw: &str) -> SCResult<Self> {
        match raw.to_uppercase().as_str() {
            "RED" => Ok(Self::Red),
            "BLUE" => Ok(Self::Blue),
            _ => Err(format!("Did not recognize player color {}", raw).into())
        }
    }
}

impl TryFrom<char> for PlayerColor {
    type Error = SCError;

    fn try_from(c: char) -> SCResult<Self> {
        match c.to_uppercase().next() {
            Some('R') => Ok(Self::Red),
            Some('B') => Ok(Self::Blue),
            _ => Err(format!("Did not recognize player color {}", c).into())
        }
    }
}

impl From<PlayerColor> for char {
    fn from(color: PlayerColor) -> char {
        match color {
            PlayerColor::Red => 'R',
            PlayerColor::Blue => 'B'
        }
    }
}

impl From<PlayerColor> for String {
    fn from(color: PlayerColor) -> String {
        match color {
            PlayerColor::Red => "RED",
            PlayerColor::Blue => "BLUE"
        }.to_owned()
    }
}
