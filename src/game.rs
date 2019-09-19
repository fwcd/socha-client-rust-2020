//! The data structures used by the XML protocol
//! and the "Hive" game.

pub struct Joined {
	pub room_id: String
}

pub struct Room {
	pub room_id: String,
	pub data: Data
}

pub struct Data {

}
