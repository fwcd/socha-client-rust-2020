use std::net::TcpStream;
use std::io::{self, Read, Write};
use log::info;

/// A handler that implements the game player's
/// behavior, usually employing some custom move
/// selection strategy.
pub trait SCClientDelegate {
	// TODO
}

/// The client which handles XML requests, manages
/// the game state and invokes the delegate.
pub struct SCClient<D> {
	delegate: D,
	debug_enabled: bool
}

impl<D> SCClient<D> {
	/// Creates a new client using the specified delegate.
	pub fn new(delegate: D, debug_enabled: bool) -> Self {
		Self { delegate: delegate, debug_enabled: debug_enabled }
	}
	
	/// Blocks the thread and begins reading XML messages
	/// from the provided address via TCP.
	pub fn run(self, host: &str, port: u16) -> io::Result<()> {
		let address = format!("{}:{}", host, port);
		let mut stream = TcpStream::connect(&address)?;
		info!("Connected to {}", address);
		
		if self.debug_enabled {
			// In debug mode, only the XML messages will be output
			io::copy(&mut stream, &mut io::stdout())?;
		}
		
		Ok(())
	}
}
