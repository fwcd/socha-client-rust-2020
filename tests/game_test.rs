use std::iter::once;
use std::collections::HashMap;
use std::convert::TryFrom;
use more_asserts::assert_lt;
use socha_client_2020::game::{Board, PlayerColor, Field, Piece, PieceType, BOARD_RADIUS, FIELD_COUNT, AxialCoords, CubeCoords};

macro_rules! assert_unordered_eq {
    ($a:expr, $b:expr) => {
        assert_eq!(
            $a.into_iter().collect::<::std::collections::HashSet<_>>(),
            $b.into_iter().collect::<::std::collections::HashSet<_>>()
        )
    };
}

#[test]
pub fn test_empty_ascii_hex_grid() {
    let ascii_hex = r#"    /\  /\    
   /  \/  \   
   |   |   |  
  /\  /\  /\  
 /  \/  \/  \ 
 |   |   |   |
 \  /\  /\  / 
  \/  \/  \/  
   |   |   |  
   \  /\  /   
    \/  \/    "#;
    let board = Board::from_ascii_hex_grid(ascii_hex).expect("Board could not be converted");
    assert_eq!(board.fields().count(), 7);
    assert!(!board.has_pieces());
}

#[test]
pub fn test_filled_ascii_hex_grid() {
    let ascii_hex = r#"    /\  /\  /\
   /  \/  \ / \
   |   |   |   | 
  /\  /\  /\  /\
 /  \/  \/  \/  \
 |   |RB |BA |BA |
 \  /\  /\  /\  /
  \/  \/  \/  \/
   |   |RG |   |
  /\  /\  /\  /\
 /  \/  \/  \/  \
 |   |   |   |   |
 \  /\  /\  /\  /
  \/  \/  \/  \/
   |   |   |   | 
   \  /\  /\  /
    \/  \/  \/"#;
    let board = Board::from_ascii_hex_grid(ascii_hex).expect("Board could not be converted");
    assert_eq!(board.fields().count(), 17);
    assert!(board.has_pieces());
    assert_unordered_eq!(board.fields_owned_by(PlayerColor::Red).map(|(c, f)| (c, f.clone())), vec![
        (AxialCoords::new(0, 0), Field::new(once(Piece {
            piece_type: PieceType::Grasshopper,
            owner: PlayerColor::Red
        }), false)),
        (AxialCoords::new(0, 1), Field::new(once(Piece {
            piece_type: PieceType::Bee,
            owner: PlayerColor::Red
        }), false))
    ]);
    assert_unordered_eq!(board.fields_owned_by(PlayerColor::Blue).map(|(c, f)| (c, f.clone())), vec![
        (AxialCoords::new(1, 0), Field::new(once(Piece {
            piece_type: PieceType::Ant,
            owner: PlayerColor::Blue
        }), false)),
        (AxialCoords::new(2, -1), Field::new(once(Piece {
            piece_type: PieceType::Ant,
            owner: PlayerColor::Blue
        }), false))
    ]);
}

#[test]
fn test_filling_radius() {
    let board = Board::filling_radius(BOARD_RADIUS, HashMap::new());
    assert_eq!(board.fields().count(), FIELD_COUNT);
    for coords in board.fields().map(|(c, _)| CubeCoords::from(c)) {
        assert_eq!(coords.x() + coords.y() + coords.z(), 0);
    }
}

#[test]
fn fields_and_neighbors() {
    let board = Board::filling_radius(2, HashMap::new());
    let origin = AxialCoords::new(0, 0);
    assert_unordered_eq!(board.fields().map(|(c, _)| c), once(origin).chain(origin.coord_neighbors()))
}

#[test]
fn board_display() {
    let board = Board::filling_radius(4, HashMap::new());
    assert_eq!(format!("{}", board), r#"000000[][][][]
0000[][][][][]
00[][][][][][]
[][][][][][][]
[][][][][][]00
[][][][][]0000
[][][][]000000
"#.to_owned());
}

#[test]
fn neighbors_in_bounds() {
    let board = Board::filling_radius(BOARD_RADIUS, HashMap::new());
    let radius = i32::try_from(BOARD_RADIUS).unwrap();
    for coords in board.fields().flat_map(|(c, _)| board.neighbors(c)).map(|(c, _)| CubeCoords::from(c)) {
        assert_lt!(coords.x().abs(), radius);
        assert_lt!(coords.y().abs(), radius);
        assert_lt!(coords.z().abs(), radius);
    }
}
