use std::{convert::TryFrom, str::FromStr};
use crate::util::{SCError, SCResult};

/// A game piece type.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PieceType {
    Ant,
    Bee,
    Beetle,
    Grasshopper,
    Spider
}

impl FromStr for PieceType {
    type Err = SCError;
    
    fn from_str(raw: &str) -> SCResult<Self> {
        match raw.to_uppercase().as_str() {
            "ANT" => Ok(Self::Ant),
            "BEE" => Ok(Self::Bee),
            "BEETLE" => Ok(Self::Beetle),
            "GRASSHOPPER" => Ok(Self::Grasshopper),
            "SPIDER" => Ok(Self::Spider),
            _ => Err(format!("Did not recognize piece type {}", raw).into())
        }
    }
}

impl TryFrom<char> for PieceType {
    type Error = SCError;
    
    fn try_from(c: char) -> SCResult<Self> {
        match c.to_uppercase().next() {
            Some('A') => Ok(Self::Ant),
            Some('B') => Ok(Self::Bee),
            Some('T') => Ok(Self::Beetle),
            Some('G') => Ok(Self::Grasshopper),
            Some('S') => Ok(Self::Spider),
            _ => Err(format!("Did not recognize piece type {}", c).into())
        }
    }
}

impl From<PieceType> for char {
    fn from(piece_type: PieceType) -> char {
        match piece_type {
            PieceType::Ant => 'A',
            PieceType::Bee => 'B',
            PieceType::Beetle => 'T',
            PieceType::Grasshopper => 'G',
            PieceType::Spider => 'S'
        }
    }
}

impl From<PieceType> for String {
    fn from(piece_type: PieceType) -> String {
        match piece_type {
            PieceType::Ant => "ANT",
            PieceType::Bee => "BEE",
            PieceType::Beetle => "BEETLE",
            PieceType::Grasshopper => "GRASSHOPPER",
            PieceType::Spider => "SPIDER"
        }.to_owned()
    }
}
