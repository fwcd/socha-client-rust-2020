use std::net::TcpStream;
use std::io::{self, BufWriter, Read, Write};
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
	debug_mode: bool
}

impl<D> SCClient<D> {
	/// Creates a new client using the specified delegate.
	pub fn new(delegate: D, debug_mode: bool) -> Self {
		Self { delegate: delegate, debug_mode: debug_mode }
	}
	
	/// Blocks the thread and begins reading XML messages
	/// from the provided address via TCP.
	pub fn run(self, host: &str, port: u16, reservation: Option<&str>) -> io::Result<()> {
		let address = format!("{}:{}", host, port);
		let mut stream = TcpStream::connect(&address)?;
		info!("Connected to {}", address);
		
		{
			let mut writer = BufWriter::new(&stream);
			writer.write("<protocol>".as_bytes())?;
			
			let join_xml = match reservation {
				Some(res) => format!("<joinPrepared reservationCode=\"{}\" />", res),
				None => "<join gameType=\"swc_2020_hive\" />".to_owned()
			};
			info!("Sending join message {}", join_xml);
			writer.write(join_xml.as_bytes())?;
		}
		
		if self.debug_mode {
			// In debug mode, only the XML messages will be output
			io::copy(&mut stream, &mut io::stdout())?;
		}
		
		Ok(())
	}
}
