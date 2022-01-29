use log::trace;
use crate::util::{SCResult, FromXmlNode, XmlNode};
use super::{Board, INITIAL_PIECE_TYPES, Move, Piece, PieceType, Player, PlayerColor, PositionedField, Adjacentable, AxialCoords, LineFormable};

/// A snapshot of the game's state at
/// a specific turn. Consists of the
/// board and information about both players.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameState {
    pub turn: u32,
    pub start_player_color: PlayerColor,
    pub current_player_color: PlayerColor,
    pub board: Board,
    red_player: Player,
    blue_player: Player,
    undeployed_red_pieces: Vec<Piece>,
    undeployed_blue_pieces: Vec<Piece>
}

impl GameState {
    /// Fetches the undeployed pieces for a specific color.
    pub fn undeployed_pieces(&self, color: PlayerColor) -> &Vec<Piece> {
        match color {
            PlayerColor::Red => &self.undeployed_red_pieces,
            PlayerColor::Blue => &self.undeployed_blue_pieces
        }
    }
    
    /// Fetches the player data for a given color.
    pub fn player(&self, color: PlayerColor) -> &Player {
        match color {
            PlayerColor::Red => &self.red_player,
            PlayerColor::Blue => &self.blue_player
        }
    } 

    /// Fetches the current _round_ (which is half the turn).
    pub fn round(&self) -> u32 { self.turn / 2 }

    /// Ensures that the destination is a direct neighbor of the start.
    fn validate_adjacent(&self, start: AxialCoords, destination: AxialCoords) -> SCResult<()> {
        if start.is_adjacent_to(destination) { Ok(()) } else { Err("Coords are not adjacent to each other".into()) }
    }
    
    fn validate_ant_move(&self, start: AxialCoords, destination: AxialCoords) -> SCResult<()> {
        if self.board.connected_by_boundary_path(start, destination) { Ok(()) } else { Err("Could not find path for ant".into()) }
    }
    
    fn validate_bee_move(&self, start: AxialCoords, destination: AxialCoords) -> SCResult<()> {
        self.validate_adjacent(start, destination)?;
        if self.board.can_move_between(start, destination) { Ok(()) } else { Err(format!("Cannot move between {:?} and {:?}", start, destination).into()) }
    }
    
    fn validate_beetle_move(&self, start: AxialCoords, destination: AxialCoords) -> SCResult<()> {
        self.validate_adjacent(start, destination)?;
        if self.board.shared_neighbors(start, destination, None).iter().any(|(_, f)| f.has_pieces()) || self.board.field(destination).map(|f| f.has_pieces()).unwrap_or(false) {
            Ok(())
        } else {
            Err("Beetle has to move along swarm".into())
        }
    }
    
    fn validate_grasshopper_move(&self, start: AxialCoords, destination: AxialCoords) -> SCResult<()> {
        if !start.forms_line_with(destination) {
            Err("Grasshopper can only move along straight lines".into())
        } else if start.is_adjacent_to(destination) {
            Err("Grasshopper must not move to a neighbor".into())
        } else if start.line_iter(destination).map(|c| AxialCoords::from(c)).any(|c| self.board.field(c).map(|f| f.is_empty()).unwrap_or(false)) {
            Err("Grasshopper cannot move over empty fields".into())
        } else {
            Ok(())
        }
    }
    
    fn validate_spider_move(&self, start: AxialCoords, destination: AxialCoords) -> SCResult<()> {
        if self.board.bfs_reachable_in_3_steps(start, destination) { Ok(()) } else { Err("No 3-step path found for Spider move".into()) }
    }

    fn validate_set_move(&self, color: PlayerColor, piece: Piece, destination_coords: impl Into<AxialCoords>) -> SCResult<()> {
        let destination = destination_coords.into();
        if !self.board.contains_coords(destination) {
            Err(format!("Move destination is out of bounds: {:?}", destination).into())
        } else if self.board.field(destination).map(|f| f.is_obstructed()).unwrap_or(true) {
            Err(format!("Move destination is obstructed: {:?}", destination).into())
        } else if !self.board.fields().any(|(_, f)| f.has_pieces()) {
            Ok(())
        } else if self.board.fields_owned_by(color).count() == 0 {
            if self.board.is_next_to(color.opponent(), destination) {
                Ok(())
            } else {
                Err("Piece has to be placed next to an opponent's piece".into())
            }
        } else if (self.round() == 3) && (!self.board.has_placed_bee(color)) && (piece.piece_type != PieceType::Bee) {
            Err("Bee has to be placed in the fourth round or earlier".into())
        } else if !self.undeployed_pieces(color).contains(&piece) {
            Err("Piece is not undeployed".into())
        } else if !self.board.neighbors(destination).any(|(_, f)| f.is_owned_by(color)) {
            Err("Piece is not placed next to an own piece".into())
        } else if self.board.neighbors(destination).any(|(_, f)| f.is_owned_by(color)) {
            Err("Piece must not be placed next to an opponent's piece".into())
        } else {
            Ok(())
        }
    }

    fn validate_drag_move(&self, color: PlayerColor, start_coords: impl Into<AxialCoords>, destination_coords: impl Into<AxialCoords>) -> SCResult<()> {
        let start = start_coords.into();
        let destination = destination_coords.into();
        if !self.board.has_placed_bee(color) {
            Err("Bee has to be placed before committing a drag move".into())
        } else if !self.board.contains_coords(start) {
            Err(format!("Move start is out of bounds: {:?}", start).into())
        } else if !self.board.contains_coords(destination) {
            Err(format!("Move destination is out of bounds: {:?}", destination).into())
        } else if let Some(dragged_piece) = self.board.field(start).and_then(|f| f.piece()) {
            if dragged_piece.owner != color {
                Err("Cannot move opponent's piece".into())
            } else if start == destination {
                Err("Cannot move when start == destination".into())
            } else if self.board.field(destination).and_then(|f| f.piece()).map(|p| p.piece_type == PieceType::Beetle).unwrap_or(false) {
                Err("Only beetles can climb other pieces".into())
            } else if {
                let mut without_piece = self.board.clone();
                without_piece.field_mut(start).ok_or_else(|| "Start field does not exist")?.pop();
                !without_piece.is_swarm_connected()
            } {
                Err("Drag move would disconnect the swarm".into())
            } else {
                match dragged_piece.piece_type {
                    PieceType::Ant => self.validate_ant_move(start, destination),
                    PieceType::Bee => self.validate_bee_move(start, destination),
                    PieceType::Beetle => self.validate_beetle_move(start, destination),
                    PieceType::Grasshopper => self.validate_grasshopper_move(start, destination),
                    PieceType::Spider => self.validate_spider_move(start, destination)
                }
            }
        } else {
            Err("No piece to move".into())
        }
    }
    
    //// Tests whether the given move is valid.
    pub fn validate_move(&self, color: PlayerColor, game_move: &Move) -> SCResult<()> {
        match game_move {
            Move::SetMove { piece, destination } => self.validate_set_move(color, *piece, destination.coords),
            Move::DragMove { start, destination } => self.validate_drag_move(color, start.coords, destination.coords)
        }
    }
    
    /// Fetches a list of possible `SetMove`s.
    fn possible_set_moves(&self, color: PlayerColor) -> Vec<Move> {
        trace!("Finding possible SetMoves");

        let undeployed = self.undeployed_pieces(color);
        let opponent = color.opponent();
        let destination_coords: Vec<_> = if undeployed.len() == INITIAL_PIECE_TYPES.len() {
            // No pieces placed yet
            if self.undeployed_pieces(opponent).len() == INITIAL_PIECE_TYPES.len() {
                // First turn
                trace!("Finding SetMoves during first turn...");
                self.board.empty_fields().map(|(c, _)| c).collect()
            } else {
                // Second turn
                trace!("Finding SetMoves during second turn...");
                self.board.fields_owned_by(opponent).flat_map(|(c, _)| self.board.empty_neighbors(c)).map(|(c, _)| c).collect()
            }
        } else {
            trace!("Querying SetMove destinations...");
            self.board.possible_set_move_destinations(color).collect()
        };

        let destinations = destination_coords.into_iter()
            .filter_map(|c| self.board.field(c).map(|f| PositionedField { coords: c, field: f.clone() }));
        trace!("Found SetMove destinations at {:#?}", destinations);
        
        if !self.board.has_placed_bee(color) && self.turn > 5 {
            trace!("Player has not placed bee yet, therefore placing it is the only valid move.");
            destinations
                .map(|d| Move::SetMove {
                    piece: Piece { piece_type: PieceType::Bee, owner: color },
                    destination: d
                })
                .collect()
        } else {
            trace!("Creating set moves from {:?} x {:?}", destinations, undeployed);
            destinations
                .flat_map(|d| undeployed.iter().map(move |&p| Move::SetMove { piece: p, destination: d.clone() }))
                .collect()
        }
    }
    
    /// Returns the validated move.
    fn validated(&self, color: PlayerColor, game_move: Move) -> SCResult<Move> {
        self.validate_move(color, &game_move).map(|_| game_move)
    }
    
    /// Fetches a list of possible `DragMove`s.
    fn possible_drag_moves(&self, color: PlayerColor) -> Vec<Move> {
        trace!("Finding possible DragMoves");

        self.board.fields_owned_by(color).flat_map(|(start_coords, start_field)| {
            let mut targets: Vec<_> = self.board.swarm_boundary().collect();

            if start_field.piece().filter(|f| f.piece_type == PieceType::Beetle).is_some() {
                targets.extend(self.board.neighbors(start_coords));
            }
            
            trace!("Drag targets from {}: {:#?}", start_coords, targets);
            targets.into_iter()
                .filter_map(move |(c, f)| self.validated(color, Move::DragMove {
                    start: PositionedField { coords: start_coords, field: start_field.clone() },
                    destination: PositionedField { coords: c, field: f.clone() }
                }).ok())
        }).collect()
    }
    
    /// Fetches a list of possible moves for a given color.
    pub fn possible_moves(&self, color: PlayerColor) -> Vec<Move> {
        trace!("Finding possible moves for color {:?}", color);
        trace!("Current board state:\n{}", self.board);

        let mut moves = self.possible_set_moves(color);
        moves.extend(self.possible_drag_moves(color));
        moves
    }
}

impl FromXmlNode for GameState {
    fn from_node(node: &XmlNode) -> SCResult<Self> {
        Ok(Self {
            turn: node.attribute("turn")?.parse()?,
            start_player_color: node.attribute("startPlayerColor")?.parse()?,
            current_player_color: node.attribute("currentPlayerColor")?.parse()?,
            red_player: Player::from_node(node.child_by_name("red")?)?,
            blue_player: Player::from_node(node.child_by_name("blue")?)?,
            board: Board::from_node(node.child_by_name("board")?)?,
            undeployed_red_pieces: node.child_by_name("undeployedRedPieces")?.childs_by_name("piece").map(Piece::from_node).collect::<Result<_, _>>()?,
            undeployed_blue_pieces: node.child_by_name("undeployedBluePieces")?.childs_by_name("piece").map(Piece::from_node).collect::<Result<_, _>>()?
        })
    }
}
