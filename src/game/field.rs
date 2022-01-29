use std::{convert::TryFrom, fmt, str::FromStr};

use regex::Regex;
use lazy_static::lazy_static;
use crate::util::{SCError, SCResult, FromXmlNode, XmlNode};
use super::{Piece, PieceType, PlayerColor};

/// A field on the game board.
/// 
/// Note that the field structure intentionally does _not_
/// store a position. If this is desired, you should use
/// `PositionedField` or a tuple, depending on whether you
/// want to express ownership over the field.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Field {
    piece_stack: Vec<Piece>,
    is_obstructed: bool
}

impl Field {
    /// Creates a new field.
    pub fn new(piece_stack: impl IntoIterator<Item=Piece>, is_obstructed: bool) -> Self {
        Self { piece_stack: piece_stack.into_iter().collect(), is_obstructed: is_obstructed }
    }

    /// Fetches the player color "owning" the field.
    pub fn owner(&self) -> Option<PlayerColor> { self.piece().map(|p| p.owner) }
    
    /// Tests whether the field is owned by the given owner.
    #[inline]
    pub fn is_owned_by(&self, color: PlayerColor) -> bool { self.owner() == Some(color) }

    /// Tests whether the field is (directly) obstructed.
    #[inline]
    pub fn is_obstructed(&self) -> bool { self.is_obstructed }
    
    /// Tests whether the field is occupied.
    #[inline]
    pub fn is_occupied(&self) -> bool { self.is_obstructed || self.has_pieces() }
    
    /// Tests whether the field is not occupied.
    #[inline]
    pub fn is_empty(&self) -> bool { !self.is_occupied() }
    
    /// Fetches the top-most piece.
    #[inline]
    pub fn piece(&self) -> Option<Piece> { self.piece_stack.last().cloned() }
    
    /// Tests whether the field contains pieces.
    #[inline]
    pub fn has_pieces(&self) -> bool { !self.piece_stack.is_empty() }
    
    /// Fetches the piece stack.
    #[inline]
    pub fn piece_stack(&self) -> &Vec<Piece> { &self.piece_stack }
    
    /// Pushes a piece onto the piece stack.
    #[inline]
    pub fn push(&mut self, piece: Piece) { self.piece_stack.push(piece) }
    
    /// Pops a piece from the piece stack or
    /// returns `None` if the stack is empty.
    #[inline]
    pub fn pop(&mut self) -> Option<Piece> { self.piece_stack.pop() }
}

lazy_static! {
    /// The syntax used for fields when parsing
    /// ASCII hex grid fields.
    static ref FIELD_SYNTAX: Regex = Regex::new(r"^([A-Z])([A-Z])$").unwrap();
}

impl FromStr for Field {
    type Err = SCError;
    
    /// Converts a field in a two-character notation
    /// to a field. The first character denotes the
    /// player color and the second character describes the
    /// piece type.
    /// 
    /// Obstructed fields and piece stacks are not (yet)
    /// supported.
    fn from_str(raw: &str) -> SCResult<Self> {
        if raw.is_empty() {
            Ok(Self::default())
        } else {
            let groups = FIELD_SYNTAX.captures(raw).ok_or_else(|| SCError::from(format!("{} does not match field syntax {}", raw, FIELD_SYNTAX.as_str())))?;
            let owner = PlayerColor::try_from(groups[1].chars().next().unwrap())?;
            let piece_type = PieceType::try_from(groups[2].chars().next().unwrap())?;
            let piece = Piece { piece_type: piece_type, owner: owner };
            Ok(Self { piece_stack: vec![piece], is_obstructed: false })
        }
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(piece) = self.piece() {
            write!(f, "{}{}", char::from(piece.owner), char::from(piece.piece_type))
        } else {
            write!(f, "[]")
        }
    }
}

impl FromXmlNode for Field {
    fn from_node(node: &XmlNode) -> SCResult<Self> {
        Ok(Self {
            piece_stack: node.childs_by_name("piece").map(Piece::from_node).collect::<Result<_, _>>()?,
            is_obstructed: node.attribute("isObstructed")?.parse()?
        })
    }
}
