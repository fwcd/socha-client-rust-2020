use crate::util::{SCResult, FromXmlNode, XmlNode};
use super::PlayerColor;

/// Metadata about a player.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Player {
    pub color: PlayerColor,
    pub display_name: String
}

impl FromXmlNode for Player {
    fn from_node(node: &XmlNode) -> SCResult<Self> {
        Ok(Self {
            color: node.attribute("color")?.parse()?,
            display_name: node.attribute("displayName")?.to_owned()
        })
    }
}
