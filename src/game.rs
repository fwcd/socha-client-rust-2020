//! The data structures used by the XML protocol
//! and the "Hive" game.

use std::str::FromStr;
use std::collections::HashMap;
use crate::util::SCError;
use crate::xml_node::XmlNode;

/// A message indicating that the client
/// has joined a room with the specified id.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Joined {
	pub room_id: String
}

/// A message in a room together with some data.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Room {
	pub room_id: String,
	pub data: Data
}

/// A polymorphic container for game data
/// used by the protocol.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Data {
	WelcomeMessage { color: PlayerColor },
	Memento { state: GameState },
	MoveRequest
}

/// A player color in the game.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlayerColor {
	Red,
	Blue
}

/// Metadata about a player.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
	pub color: PlayerColor,
	pub display_name: String
}

/// A snapshot of the game's state at
/// a specific turn. Consists of the
/// board and information about both players.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameState {
	pub turn: u32,
	pub start_player_color: PlayerColor,
	pub current_player_color: PlayerColor,
	pub red_player: Player,
	pub blue_player: Player,
	pub board: Board
}

/// Axial coordinates on the hex grid.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AxialCoords {
	pub x: i32,
	pub y: i32
}

/// Cube coordinates on the hex grid.
/// These are used by the protocol internally.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CubeCoords {
	pub x: i32,
	pub y: i32,
	pub z: i32
}

/// The game board which is a symmetric hex grid with
/// a side length of 6 fields.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
	// TODO: Store fields contiguously in a Vec
	// and convert between coords and indices
	fields: HashMap<AxialCoords, Field>
}

/// A field on the game board.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Field {
	pub is_obstructed: bool
}

// General implementations

impl Board {
	/// Fetches a reference to the field at the given
	/// coordinates. The coordinates can be of and type
	/// (e.g. axial/cube) as long as they are convertible
	/// to axial coordinates.
	pub fn field(&self, coords: impl Into<AxialCoords>) -> Option<&Field> {
		self.fields.get(&coords.into())
	}
}

// General conversions

impl From<CubeCoords> for AxialCoords {
	fn from(coords: CubeCoords) -> Self { Self { x: coords.x, y: coords.y } }
}

impl From<AxialCoords> for CubeCoords {
	fn from(coords: AxialCoords) -> Self { Self { x: coords.x, y: coords.y, z: -(coords.x + coords.y) } }
}

impl FromStr for PlayerColor {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, String> {
		match s {
			"RED" => Ok(Self::Red),
			"BLUE" => Ok(Self::Blue),
			_ => Err(format!("Did not recognize player color {}", s))
		}
	}
}

// XML conversions

impl From<&XmlNode> for Result<Joined, SCError> {
	fn from(node: &XmlNode) -> Self { Ok(Joined { room_id: node.attribute("room_id")?.to_owned() }) }
}

impl From<&XmlNode> for Result<Room, SCError> {
	fn from(node: &XmlNode) -> Self {
		Ok(Room {
			room_id: node.attribute("room_id")?.to_owned(),
			data: <Result<Data, _>>::from(node.child_by_name("data")?)?
		})
	}
}

impl From<&XmlNode> for Result<Data, SCError> {
	fn from(node: &XmlNode) -> Self {
		let class = node.attribute("class")?;
		match class {
			"welcomeMessage" => Ok(Data::WelcomeMessage { color: node.attribute("color")?.parse()? }),
			"memento" => Ok(Data::Memento { state: <Result<GameState, _>>::from(node.child_by_name("state")?)? }),
			"sc.framework.plugins.protocol.MoveRequest" => Ok(Data::MoveRequest),
			_ => Err(format!("Unrecognized data class: {}", class).into())
		}
	}
}

impl From<&XmlNode> for Result<GameState, SCError> {
	fn from(node: &XmlNode) -> Self {
		Ok(GameState {
			turn: node.attribute("turn")?.parse()?,
			start_player_color: node.attribute("startPlayerColor")?.parse()?,
			current_player_color: node.attribute("currentPlayerColor")?.parse()?,
			red_player: <Result<Player, _>>::from(node.child_by_name("red")?)?,
			blue_player: <Result<Player, _>>::from(node.child_by_name("blue")?)?,
			board: <Result<Board, _>>::from(node.child_by_name("board")?)?
		})
	}
}

impl From<&XmlNode> for Result<Player, SCError> {
	fn from(node: &XmlNode) -> Self {
		Ok(Player {
			color: node.attribute("color")?.parse()?,
			display_name: node.attribute("displayName")?.to_owned()
		})
	}
}

impl From<&XmlNode> for Result<Board, SCError> {
	fn from(node: &XmlNode) -> Self {
		Ok(Board {
			fields: node.child_by_name("fields")?
				.childs_by_name("field")
				.map(|f| Ok((
					CubeCoords {
						x: f.attribute("x")?.parse()?,
						y: f.attribute("y")?.parse()?,
						z: f.attribute("z")?.parse()?
					}.into(),
					<Result<Field, _>>::from(f)?
				)))
				.collect::<Result<HashMap<AxialCoords, Field>, SCError>>()?
		})
	}
}

impl From<&XmlNode> for Result<Field, SCError> {
	fn from(node: &XmlNode) -> Self {
		Ok(Field {
			is_obstructed: node.attribute("isObstructed")?.parse()?
		})
	}
}
