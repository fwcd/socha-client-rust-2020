use crate::util::{SCResult, FromXmlNode, XmlNode};
use super::{PieceType, PlayerColor};

/// A game piece.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Piece {
    pub owner: PlayerColor,
    pub piece_type: PieceType
}

impl FromXmlNode for Piece {
    fn from_node(node: &XmlNode) -> SCResult<Self> {
        Ok(Self {
            owner: node.attribute("owner")?.parse()?,
            piece_type: node.attribute("type")?.parse()?
        })
    }
}

impl From<Piece> for XmlNode {
    fn from(piece: Piece) -> Self {
        XmlNode::new("piece")
            .attribute("owner", piece.owner)
            .attribute("type", piece.piece_type)
            .build()
    }
}
