use crate::client::SCClientDelegate;

/// An empty game logic structure that
/// implements the client delegate trait
/// and thus is responsible e.g. for picking
/// a move when requested.
pub struct OwnGameLogic;

impl SCClientDelegate for OwnGameLogic {
	// TODO
}
