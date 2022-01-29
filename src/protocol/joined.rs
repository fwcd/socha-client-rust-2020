use crate::util::{SCResult, FromXmlNode, XmlNode};

/// A message indicating that the client
/// has joined a room with the specified id.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Joined {
    pub room_id: String
}

impl FromXmlNode for Joined {
    fn from_node(node: &XmlNode) -> SCResult<Self> { Ok(Self { room_id: node.attribute("roomId")?.to_owned() }) }
}
