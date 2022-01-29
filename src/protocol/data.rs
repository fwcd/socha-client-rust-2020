use std::convert::TryFrom;
use crate::{util::{SCError, SCResult, FromXmlNode, XmlNode}, game::{Move, PlayerColor, GameState}};
use super::GameResult;

/// A polymorphic container for game data
/// used by the protocol. It is parameterized
/// by the player color (`C`), the game state (`S`)
/// and the player structure (`P`). These types
/// are implemented independently of the base
/// protocol for each year's game.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Data {
    WelcomeMessage { color: PlayerColor },
    Memento { state: GameState },
    Move(Move),
    MoveRequest,
    GameResult(GameResult),
    Error { message: String }
}

impl FromXmlNode for Data {
    fn from_node(node: &XmlNode) -> SCResult<Self> {
        let class = node.attribute("class")?;
        match class {
            "welcomeMessage" => Ok(Self::WelcomeMessage { color: node.attribute("color")?.parse()? }),
            "memento" => Ok(Self::Memento { state: GameState::from_node(node.child_by_name("state")?)? }),
            "sc.framework.plugins.protocol.MoveRequest" => Ok(Self::MoveRequest),
            "result" => Ok(Self::GameResult(GameResult::from_node(node)?)),
            "error" => Ok(Self::Error { message: node.attribute("message")?.to_owned() }),
            _ => Err(format!("Unrecognized data class: {}", class).into())
        }
    }
}

impl TryFrom<Data> for XmlNode {
    type Error = SCError;

    fn try_from(data: Data) -> SCResult<XmlNode> {
        match data {
            Data::Move(game_move) => Ok(game_move.into()),
            _ => Err(format!("{:?} can currently not be serialized", data).into())
        }
    }
}
