use crate::util::{SCResult, FromXmlNode, XmlNode};
use super::ScoreCause;

/// The score of a game player.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayerScore {
    pub cause: ScoreCause,
    pub reason: String
}

impl FromXmlNode for PlayerScore {
    fn from_node(node: &XmlNode) -> SCResult<Self> {
        Ok(Self {
            cause: node.attribute("cause")?.parse()?,
            reason: node.attribute("reason").map(|s| s.to_owned()).unwrap_or_default()
        })
    }
}
