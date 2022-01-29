use std::convert::TryFrom;
use std::net::TcpStream;
use std::io::{self, BufWriter, BufReader, Read, Write};
use log::{info, debug, warn, error};
use xml::reader::{XmlEvent as XmlReadEvent, EventReader};
use xml::writer::EmitterConfig;
use crate::game::{GameState, PlayerColor, Move};
use crate::util::{SCResult, XmlNode, FromXmlNode};
use crate::protocol::{Joined, Left, Room, Data, GameResult};

const GAME_TYPE: &str = "swc_2020_hive";

/// A handler that implements the game player's
/// behavior, usually employing some custom move
/// selection strategy.
pub trait SCClientDelegate {
    /// Invoked whenever the game state updates.
    fn on_update_state(&mut self, _state: &GameState) {}
    
    /// Invoked when the game ends.
    fn on_game_end(&mut self, _result: GameResult) {}
    
    /// Invoked when the welcome message is received
    /// with the player's color.
    fn on_welcome_message(&mut self, _color: &PlayerColor) {}
    
    /// Requests a move from the delegate. This method
    /// should implement the "main" game logic.
    fn request_move(&mut self, state: &GameState, my_color: PlayerColor) -> Move;
}

/// A configuration that determines whether
/// the reader and/or the writer of a stream
/// should be swapped by stdio to ease debugging.
pub struct DebugMode {
    pub debug_reader: bool,
    pub debug_writer: bool,
}

/// The client which handles XML requests, manages
/// the game state and invokes the delegate.
pub struct SCClient<D> where D: SCClientDelegate {
    delegate: D,
    debug_mode: DebugMode,
    game_state: Option<GameState>,
}

impl<D> SCClient<D> where D: SCClientDelegate {
    /// Creates a new client using the specified delegate.
    pub fn new(delegate: D, debug_mode: DebugMode) -> Self {
        Self { delegate, debug_mode, game_state: None }
    }
    
    /// Blocks the thread and begins reading XML messages
    /// from the provided address via TCP.
    pub fn run(self, host: &str, port: u16, reservation: Option<&str>) -> SCResult<()> {
        let address = format!("{}:{}", host, port);
        let stream = TcpStream::connect(&address)?;
        info!("Connected to {}", address);
        
        {
            let mut writer = BufWriter::new(&stream);
            writer.write("<protocol>".as_bytes())?;
            
            let join_xml = match reservation {
                Some(res) => format!("<joinPrepared reservationCode=\"{}\" />", res),
                None => format!("<join gameType=\"{}\" />", GAME_TYPE)
            };
            info!("Sending join message {}", join_xml);
            writer.write(join_xml.as_bytes())?;
        }
        
        // Begin parsing game messages from the stream.
        // List all combinations of modes explicitly,
        // since they generate different generic instantiations
        // of `run_game`.

        let mode = &self.debug_mode;
        if mode.debug_reader && !mode.debug_writer {
            self.run_game(io::stdin(), BufWriter::new(stream))?;
        } else if !mode.debug_reader && mode.debug_writer {
            self.run_game(BufReader::new(stream), io::stdout())?;
        } else if mode.debug_reader && mode.debug_writer {
            self.run_game(io::stdin(), io::stdout())?;
        } else {
            let reader = BufReader::new(stream.try_clone()?);
            let writer = BufWriter::new(stream);
            self.run_game(reader, writer)?;
        }
        
        Ok(())
    }
    
    /// Blocks the thread and parses/handles game messages
    /// from the provided reader.
    fn run_game<R, W>(mut self, reader: R, writer: W) -> SCResult<()> where R: Read, W: Write {
        let mut xml_reader = EventReader::new(reader);

        let mut emitter_config = EmitterConfig::new();
        emitter_config.write_document_declaration = false;

        let mut xml_writer = emitter_config.create_writer(writer);
        
        // Read initial protocol element
        info!("Waiting for initial <protocol>...");
        while match xml_reader.next() {
            Ok(XmlReadEvent::StartElement { name, .. }) => Some(name),
            _ => None
        }.filter(|n| n.local_name == "protocol").is_none() {}

        loop {
            let node = XmlNode::read_from(&mut xml_reader)?;
            debug!("Got XML node {}", node);
            
            match node.name() {
                // Try parsing as room message (the game is running)
                "room" => match Room::from_node(&node) {
                    Ok(room) => match room.data {
                        Data::WelcomeMessage { color } => {
                            info!("Got welcome message with color: {:?}", color);
                            self.delegate.on_welcome_message(&color);
                        },
                        Data::Memento { state } => {
                            info!("Got updated game state");
                            self.delegate.on_update_state(&state);
                            self.game_state = Some(state);
                        },
                        Data::MoveRequest => {
                            if let Some(ref state) = self.game_state {
                                let turn = state.turn;
                                let color = state.current_player_color;
                                info!("Got move request @ turn: {}, color: {:?}", turn, color);

                                let new_move = self.delegate.request_move(state, color);
                                let move_node = XmlNode::try_from(Room {
                                    room_id: room.room_id,
                                    data: Data::Move(new_move)
                                })?;

                                debug!("Sending move {}", move_node);
                                move_node.write_to(&mut xml_writer)?;
                                xml_writer.inner_mut().flush()?;
                            } else {
                                error!("Got move request, which cannot be fulfilled since no game state is present!");
                            }
                        },
                        Data::GameResult(result) => {
                            info!("Got game result: {:?}", result);
                            self.delegate.on_game_end(result);
                        },
                        Data::Error { message } => {
                            warn!("Got error from server: {}", message);
                        },
                        _ => warn!("Could not handle room data: {:?}", room.data)
                    },
                    Err(e) => error!("Could not parse node as room: {:?}", e)
                },

                // Try parsing as 'joined' message
                "joined" => match Joined::from_node(&node) {
                    Ok(joined) => info!("Joined room {}", joined.room_id),
                    Err(e) => error!("Could not parse node as 'joined': {:?}", e)
                },

                // Try parsing as 'left' message
                "left" => match Left::from_node(&node) {
                    Ok(left) => info!("Left room {}", left.room_id),
                    Err(e) => error!("Could not parse node as 'left': {:?}", e)
                },
                
                "close" | "sc.protocol.responses.CloseConnection" => {
                    info!("Closing connection as requested by server...");
                    break;
                },
                
                _ => warn!("Unrecognized message: <{}>", node.name())
            }
        }
        
        Ok(())
    }
}
