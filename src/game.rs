//! The data structures used by the XML protocol
//! and the "Hive" game.

use std::collections::HashMap;
use crate::xml_node::XmlNode;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Joined {
	pub room_id: String
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Room {
	pub room_id: String,
	pub data: Data
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Data {
	WelcomeMessage { color: PlayerColor },
	Memento { state: GameState }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlayerColor {
	Red,
	Blue
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameState {
	pub turn: u32,
	pub start_player_color: PlayerColor,
	pub current_player_color: PlayerColor,
	pub red_player: PlayerColor,
	pub blue_player: PlayerColor,
	pub board: Board
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AxialCoords {
	pub x: i32,
	pub y: i32
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CubeCoords {
	pub x: i32,
	pub y: i32,
	pub z: i32
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
	// TODO: Store fields contiguously in a Vec
	// and convert between coords and indices
	pub fields: HashMap<AxialCoords, Field>
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Field {
	pub is_obstructed: bool
}

// General conversions

impl From<CubeCoords> for AxialCoords {
	fn from(coords: CubeCoords) -> Self {
		Self { x: coords.x, y: coords.y }
	}
}

impl From<AxialCoords> for CubeCoords {
	fn from(coords: AxialCoords) -> Self {
		Self { x: coords.x, y: coords.y, z: -(coords.x + coords.y) }
	}
}

// XML conversions

impl From<XmlNode> for Joined {
	fn from(node: XmlNode) -> Self { Self {
		room_id: node.attributes["room_id"]
	} }
}

impl From<XmlNode> for Room {
	fn from(node: XmlNode) -> Self { Self {
		room_id: node.attributes["room_id"],
		data: node.childs.first().into()
	} }
}

impl From<XmlNode> for Data {
	fn from(node: XmlNode) -> Self { Self {

	} }
}
