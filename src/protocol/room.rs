use std::convert::TryFrom;
use crate::util::{SCError, SCResult, FromXmlNode, XmlNode};
use super::Data;

/// A message in a room together with some data.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Room {
    pub room_id: String,
    pub data: Data
}

impl FromXmlNode for Room {
    fn from_node(node: &XmlNode) -> SCResult<Self> {
        Ok(Self {
            room_id: node.attribute("roomId")?.to_owned(),
            data: Data::from_node(node.child_by_name("data")?)?
        })
    }
}

impl TryFrom<Room> for XmlNode {
    type Error = SCError;

    fn try_from(room: Room) -> SCResult<XmlNode> {
        Ok(XmlNode::new("room")
            .attribute("roomId", room.room_id)
            .try_child(room.data)?
            .into())
    }
}
