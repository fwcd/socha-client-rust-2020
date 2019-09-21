use socha_client_base::client::SCClientDelegate;
use socha_plugin_2020::{plugin::SCPlugin2020, game::*};
use log::info;

/// An empty game logic structure that
/// implements the client delegate trait
/// and thus is responsible e.g. for picking
/// a move when requested.
pub struct OwnGameLogic;

impl SCClientDelegate for OwnGameLogic {
	type Plugin = SCPlugin2020;
	
	fn request_move(&mut self, state: &GameState, my_color: PlayerColor) -> Move {
		// Implement custom game logic here!
		let moves = state.possible_moves(my_color);
		let first_move = moves.iter().next().cloned();
		info!("Chose {:?} from {} moves", first_move, moves.len());
		first_move.expect("No move found")
	}
}
