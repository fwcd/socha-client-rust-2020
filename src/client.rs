/// A handler that implements the game player's
/// behavior, usually employing some custom move
/// selection strategy.
pub trait SCClientDelegate {
	// TODO
}

/// The client which handles XML requests, manages
/// the game state and invokes the delegate.
pub struct SCClient<D> {
	delegate: D
}

impl<D> SCClient<D> {
	/// Creates a new client using the specified delegate.
	pub fn new(delegate: D) -> Self {
		Self { delegate: delegate }
	}
}
