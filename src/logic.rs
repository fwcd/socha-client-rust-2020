use socha_client_base::client::SCClientDelegate;
use socha_plugin_2020::{plugin::SCPlugin2020, game::*};
use rand::seq::SliceRandom;
use log::{info, debug};

/// An empty game logic structure that
/// implements the client delegate trait
/// and thus is responsible e.g. for picking
/// a move when requested.
pub struct OwnGameLogic;

impl SCClientDelegate for OwnGameLogic {
	type Plugin = SCPlugin2020;
	
	fn request_move(&mut self, state: &GameState, my_color: PlayerColor) -> Move {
		// Implement custom game logic here!
		let mut random = rand::thread_rng();
		let moves = state.possible_moves(my_color);
		let game_move = moves.choose(&mut random).cloned().expect("No move found");
		info!("Chose {:?} from {} moves", game_move, moves.len());
		game_move
	}
	
	fn on_update_state(&mut self, state: &GameState) {
		debug!("New board:\n{}", state.board);
	}
}
