use super::PieceType;

pub const ROUND_LIMIT: usize = 30;
pub const BOARD_RADIUS: usize = 6;
pub const FIELD_COUNT: usize = 91; // def count(radius): 1 if (radius == 1) else (radius - 1) * 6 + count(radius - 1)
pub const INITIAL_PIECE_TYPES: [PieceType; 11] = [
    PieceType::Bee,
    PieceType::Spider,
    PieceType::Spider,
    PieceType::Spider,
    PieceType::Grasshopper,
    PieceType::Grasshopper,
    PieceType::Beetle,
    PieceType::Beetle,
    PieceType::Ant,
    PieceType::Ant,
    PieceType::Ant
];
