use crate::util::{XmlNode, XmlNodeBuilder};
use super::{AxialCoords, Piece, PositionedField};

/// A transition between two game states.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Move<C=AxialCoords> {
    SetMove { piece: Piece, destination: PositionedField<C> },
    DragMove { start: PositionedField<C>, destination: PositionedField<C> }
}

impl From<Move> for XmlNode {
    fn from(game_move: Move) -> Self {
        match game_move {
            Move::SetMove { piece, destination } => XmlNode::new("data")
                .attribute("class", "setmove")
                .child(piece)
                .child(XmlNodeBuilder::from(destination).name("destination"))
                .build(),
            Move::DragMove { start, destination } => XmlNode::new("data")
                .attribute("class", "dragmove")
                .child(XmlNodeBuilder::from(start).name("start"))
                .child(XmlNodeBuilder::from(destination).name("destination"))
                .build()
        }
    }
}
