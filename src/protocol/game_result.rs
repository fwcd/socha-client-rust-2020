use crate::{util::{SCResult, FromXmlNode, XmlNode}, game::Player};
use super::{PlayerScore, ScoreDefinition};

/// The final result of a game.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameResult {
    pub definition: ScoreDefinition,
    pub scores: Vec<PlayerScore>,
    pub winners: Vec<Player>
}

impl FromXmlNode for GameResult {
    fn from_node(node: &XmlNode) -> SCResult<Self> {
        Ok(Self {
            definition: ScoreDefinition::from_node(node.child_by_name("definition")?)?,
            scores: node.childs_by_name("score").map(PlayerScore::from_node).collect::<SCResult<_>>()?,
            winners: node.childs_by_name("winner").map(Player::from_node).collect::<SCResult<_>>()?
        })
    }
}
