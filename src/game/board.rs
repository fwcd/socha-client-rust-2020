use std::{cmp::max, cmp::min, collections::HashMap, collections::HashSet, collections::VecDeque, convert::TryFrom, fmt, str::FromStr};

use arrayvec::ArrayVec;
use itertools::Itertools;
use log::{debug, trace};
use super::{Field, Piece, PieceType, PlayerColor, AxialCoords, CubeCoords, DoubledCoords};
use crate::util::{SCResult, FromXmlNode, XmlNode};

/// The game board which is a symmetric hex grid with
/// a side length of 6 fields.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    // TODO: Store fields contiguously in a Vec
    // or ideally a fixed-size container such
    // as an array or an ArrayVec.
    // and convert between coords and indices
    fields: HashMap<AxialCoords, Field>
}

impl Board {
    /// Creates a new board with the given fields.
    pub fn new(fields: impl Into<HashMap<AxialCoords, Field>>) -> Self {
        Self { fields: fields.into() }
    }
    
    /// Creates a new hexagonal board. In addition to the provided
    /// fields, the board is padded with empty fields up to the
    /// given radius.
    pub fn filling_radius(radius: usize, fields: impl Into<HashMap<AxialCoords, Field>>) -> Self {
        let mut fields_mut: HashMap<_, _> = fields.into();
        trace!("Filling up board, occupied fields: {:?}", fields_mut.iter().filter(|(_, f)| f.is_occupied()).collect::<Vec<_>>());

        let outer = i32::try_from(radius).expect("Radius is too large to fit in a 32-bit (signed) int");
        let inner = outer - 1;
        let all_coords = ((-inner)..=inner)
            .flat_map(|y| (max(-(inner + y), -inner)..=min(inner - y, inner))
                .map(move |x| AxialCoords::new(x, y)));
        
        for coords in all_coords {
            if !fields_mut.contains_key(&coords) {
                fields_mut.insert(coords, Field::default());
                trace!("Filling up field at {}", coords);
            }
        }
        
        let board = Self { fields: fields_mut };
        trace!("Created board with occupied fields {:?}", board.occupied_fields().collect::<Vec<_>>());
        board
    }

    /// Parses a board from a plain text
    /// hex grid of the following format:
    ///
    /// ```ignore 
    ///     /\  /\      
    ///    /  \/  \     
    ///    |BR |   |    
    ///   /\  /\  /\    
    ///  /  \/  \/  \   
    ///  |   |GB |   |  
    ///  \  /\  /\  /   
    ///   \/  \/  \/    
    ///    |   |   |    
    ///    \  /\  /     
    ///     \/  \/      
    /// ```
    /// 
    /// The rows should be "indented" alternatingly
    /// with the first row indented as depicted
    /// by the example above and the board should
    /// have a perfectly centered field.
    /// 
    /// Each hex field may or may not contain
    /// a `Field` described by a two-character
    /// notation where the _first_ character
    /// denotes the owner color and the _second_
    /// character the piece type (more details
    /// can be found in `Field`'s `FromStr`
    /// implementation). Empty or invalid field contents
    /// are ignored.
    /// 
    /// Note that the format currently does
    /// not support stacked pieces or obstructed
    /// fields.
    /// 
    /// The fields will be returned in the format
    /// of axial coordinates with the origin being
    /// located in the center of the board. The x-axis
    /// points to the right and the y-axis diagonally
    /// to the top-left.
    pub fn from_ascii_hex_grid(grid: impl Into<String>) -> SCResult<Self> {
        let double_positioned: Vec<_> = grid.into().lines()
            .map(|l| l.trim())
            .skip_while(|l| l.is_empty())
            .skip(2)
            .step_by(3)
            .enumerate()
            .map(|(y, line)| (i32::try_from(y).unwrap(), line))
            .flat_map(|(y, line)| line
                .split("|")
                .filter(|frag| !frag.is_empty())
                .enumerate()
                .map(|(x, frag)| (i32::try_from(x).unwrap(), frag))
                .map(move |(x, frag)| (
                    DoubledCoords::new((2 * x) + ((y + 1) % 2), y),
                    Field::from_str(frag.trim()).unwrap_or_else(|e| {
                        debug!("Could not parse {}: {:?}", frag, e);
                        Field::default()
                    })
                )))
            .collect();
        let center = DoubledCoords::new(
            double_positioned.iter().map(|(c, _)| c.x()).max().unwrap_or(0),
            double_positioned.iter().map(|(c, _)| c.y()).max().unwrap_or(0)
        ) / 2;
        debug!("Determined center at {:?}", center);
        debug!("Parsed fields at {:?}", double_positioned.iter().map(|(c, _)| *c - center).collect::<Vec<_>>());
        let fields: HashMap<_, _> = double_positioned.into_iter()
            .map(|(c, f)| (AxialCoords::from(c - center), f))
            .collect();
        debug!("Fields: {:?}", fields);
        Ok(Board::new(fields))
    }

    /// Fetches a reference to the field at the given
    /// coordinates. The coordinates can be of and type
    /// (e.g. axial/cube) as long as they are convertible
    /// to axial coordinates.
    #[inline]
    pub fn field(&self, coords: impl Into<AxialCoords>) -> Option<&Field> {
        self.fields.get(&coords.into())
    }
    
    /// Mutably borrows a field.
    pub fn field_mut(&mut self, coords: impl Into<AxialCoords>) -> Option<&mut Field> {
        self.fields.get_mut(&coords.into())
    }
    
    /// Tests whether a given position is occupied.
    pub fn is_occupied(&self, coords: impl Into<AxialCoords>) -> bool {
        self.field(coords).map(|f| f.is_occupied()).unwrap_or(true)
    }
    
    /// Fetches all fields owned by the given color.
    pub fn fields_owned_by(&self, color: PlayerColor) -> impl Iterator<Item=(AxialCoords, &Field)> {
        self.fields().filter(move |(_, f)| f.is_owned_by(color))
    }
    
    /// Fetches all empty fields.
    pub fn empty_fields(&self) -> impl Iterator<Item=(AxialCoords, &Field)> {
        self.fields().filter(|(_, f)| f.is_empty())
    }
    
    /// Fetches all occupied fields.
    pub fn occupied_fields(&self) -> impl Iterator<Item=(AxialCoords, &Field)> {
        self.fields().filter(|(_, f)| f.is_occupied())
    }
    
    /// Fetches empty fields connected to the swarm.
    pub fn swarm_boundary(&self) -> impl Iterator<Item=(AxialCoords, &Field)> {
        self.fields().filter(|(_, f)| f.is_occupied())
            .flat_map(move |(c, _)| self.empty_neighbors(c))
    }
    
    /// Fetches all fields.
    #[inline]
    pub fn fields(&self) -> impl Iterator<Item=(AxialCoords, &Field)> {
        self.fields.iter().map(|(&c, f)| (c, f))
    }
    
    /// Tests whether the board contains the given coordinate.
    #[inline]
    pub fn contains_coords(&self, coords: impl Into<AxialCoords>) -> bool {
        self.fields.contains_key(&coords.into())
    }
    
    /// Tests whether the board has any pieces.
    pub fn has_pieces(&self) -> bool {
        self.fields().any(|(_, f)| f.has_pieces())
    }
    
    /// Fetches the (existing) neighbor fields on the board.
    #[inline]
    pub fn neighbors<'a>(&'a self, coords: impl Into<AxialCoords>) -> impl Iterator<Item=(AxialCoords, &Field)> + 'a {
        coords.into().coord_neighbors().into_iter().filter_map(move |c| self.field(c).map(|f| (c, f)))
    }
    
    /// Fetches the unoccupied neighbor fields.
    pub fn empty_neighbors(&self, coords: impl Into<AxialCoords>) -> impl Iterator<Item=(AxialCoords, &Field)> {
        self.neighbors(coords).filter(|(_, f)| f.is_empty())
    }
    
    /// Tests whether the bee of the given color has been placed.
    pub fn has_placed_bee(&self, color: PlayerColor) -> bool {
        let bee = Piece { piece_type: PieceType::Bee, owner: color };
        self.fields().flat_map(|(_, f)| f.piece_stack()).any(|&p| p == bee)
    }
    
    /// Tests whether the field at the given coordinates is next to
    /// a given color.
    pub fn is_next_to(&self, color: PlayerColor, coords: impl Into<AxialCoords>) -> bool {
        self.neighbors(coords).any(|(_, f)| f.is_owned_by(color))
    }
    
    /// Tests whether the field at the given coordinates is adjacent
    /// to a field.
    pub fn is_next_to_piece(&self, coords: impl Into<AxialCoords>) -> bool {
        self.neighbors(coords).any(|(_, f)| f.has_pieces())
    }
    
    /// Fetches the possible destinations for a SetMove.
    pub fn possible_set_move_destinations<'a>(&'a self, color: PlayerColor) -> impl Iterator<Item=AxialCoords> + 'a {
        let opponent = color.opponent();

        trace!("Looking for SetMove destinations on board...");
        trace!("Fields owned by {:?}: {:#?}", color, self.fields_owned_by(color).collect::<Vec<_>>());
        trace!("Fields owned by {:?} (opponent): {:#?}", opponent, self.fields_owned_by(opponent).collect::<Vec<_>>());

        self.fields_owned_by(color)
            .flat_map(move |(c, _)| self.empty_neighbors(c))
            .unique()
            .filter_map(move |(c, _)| if self.is_next_to(opponent, c) { None } else {
                trace!("SetMove destination {} does not touch an opponent's ({:?}'s) piece, neighbors: {:#?}", c, opponent, self.neighbors(c).collect::<Vec<_>>());
                Some(c)
            })
    }
    
    /// Performs a depth-first search on the board's non-empty fields
    /// starting at the given coordinates and removing visited
    /// locations from the set.
    fn dfs_swarm(&self, coords: AxialCoords, unvisited: &mut HashSet<AxialCoords>) {
        if self.field(coords).filter(|f| f.has_pieces()).is_some() {
            unvisited.remove(&coords);
            for (neighbor, _) in self.neighbors(coords) {
                if unvisited.contains(&neighbor) {
                    self.dfs_swarm(neighbor, unvisited)
                }
            }
        }
    }
    
    /// Tests whether a field satisfying the search condition can be
    /// reached by breadth-first searching the accessible fields.
    fn bfs_accessible(&self, start: AxialCoords, search_condition: impl Fn(AxialCoords, &Field) -> bool) -> bool {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        queue.push_back(start);
        
        while let Some(coords) = queue.pop_front() {
            visited.insert(coords);

            if let Some(field) = self.field(coords) {
                if search_condition(coords, field) {
                    return true;
                } else {
                    queue.extend(self.accessible_neighbors_except(Some(start), coords).filter_map(|(c, _)| if !visited.contains(&c) { Some(c) } else { None }));
                }
            }
        }

        false
    }
    
    /// Tests whether the given field can be reached in 3 moves
    /// by breadth-first searching the accessible fields.
    pub fn bfs_reachable_in_3_steps(&self, start: AxialCoords, destination: AxialCoords) -> bool {
        let mut paths_queue: VecDeque<ArrayVec<AxialCoords, 3>> = VecDeque::new();
        paths_queue.push_back({
            let mut path = ArrayVec::new();
            path.push(start);
            path
        });

        while let Some(path) = paths_queue.pop_front() {
            let mut neighbors = self.accessible_neighbors_except(Some(start), path.last().cloned().unwrap()).filter(|(c, _)| !path.contains(c));
            if path.len() < 3 {
                paths_queue.extend(neighbors.map(|(c, _)| {
                    let mut next_path = path.clone();
                    next_path.push(c);
                    next_path
                }));
            } else if neighbors.any(|(c, _)| c == destination) {
                return true;
            }
        }

        false
    }
    
    /// Finds the intersection between `a`'s and `b`'s neighbors,
    /// optionally given an exception whose field won't be included
    /// if it contains exactly one piece.
    pub fn shared_neighbors(&self, a: impl Into<AxialCoords>, b: impl Into<AxialCoords>, exception: Option<AxialCoords>) -> Vec<(AxialCoords, &Field)> {
        let a_neighbors: HashSet<_> = self.neighbors(a).collect();
        let b_neighbors: HashSet<_> = self.neighbors(b).collect();
        a_neighbors.intersection(&b_neighbors)
            .filter(|(c, f)| f.piece_stack().len() != 1 || exception == Some(*c))
            .cloned().collect()
    }
    
    /// Tests whether a move between the given two
    /// locations is possible, optionally given an
    /// exception.
    pub fn can_move_between_except(&self, exception: Option<AxialCoords>, a: impl Into<AxialCoords>, b: impl Into<AxialCoords>) -> bool {
        let shared = self.shared_neighbors(a, b, exception);
        (shared.len() == 1 || shared.iter().any(|(_, f)| f.is_empty())) && shared.iter().any(|(_, f)| f.has_pieces())
    }
    
    /// Tests whether a move between the given two
    /// locations is possible.
    pub fn can_move_between(&self, a: impl Into<AxialCoords>, b: impl Into<AxialCoords>) -> bool {
        self.can_move_between_except(None, a, b)
    }
    
    /// Finds the accessible neighbors, optionally except an ignored field.
    pub fn accessible_neighbors_except<'a>(&'a self, exception: Option<AxialCoords>, coords: impl Into<AxialCoords> + Copy + 'a) -> impl Iterator<Item=(AxialCoords, &Field)> + 'a {
        self.neighbors(coords).filter(move |(c, f)| f.is_empty() && self.can_move_between_except(exception, coords, *c))
    }
    
    /// Finds the accessible neighbors.
    pub fn accessible_neighbors<'a>(&'a self, coords: impl Into<AxialCoords> + Copy + 'a) -> impl Iterator<Item=(AxialCoords, &Field)> + 'a {
        self.neighbors(coords).filter(move |(c, f)| f.is_empty() && self.can_move_between(coords, *c))
    }
    
    /// Tests whether two coordinates are connected by a path
    /// along the swarm's boundary.
    pub fn connected_by_boundary_path(&self, start_coords: impl Into<AxialCoords>, destination_coords: impl Into<AxialCoords>) -> bool {
        let start = start_coords.into();
        let destination = destination_coords.into();
        self.bfs_accessible(start, |c, _| c == destination)
    }
    
    /// Performs a depth-first search on the board at the given
    /// position to test whether the swarm is connected.
    pub fn is_swarm_connected(&self) -> bool {
        let mut unvisited = self.fields.iter()
            .filter_map(|(&c, f)| if f.has_pieces() { Some(c) } else { None })
            .collect::<HashSet<AxialCoords>>();

        if let Some(start) = unvisited.iter().next() {
            self.dfs_swarm(*start, &mut unvisited);
            unvisited.is_empty()
        } else {
            true // An empty swarm is connected
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let min_x = self.fields().map(|(c, _)| c.x()).min().ok_or(fmt::Error)?;
        let min_y = self.fields().map(|(c, _)| c.y()).min().ok_or(fmt::Error)?;
        let max_x = self.fields().map(|(c, _)| c.x()).max().ok_or(fmt::Error)?;
        let max_y = self.fields().map(|(c, _)| c.y()).max().ok_or(fmt::Error)?;

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if let Some(field) = self.field(AxialCoords::new(-y, -x)) {
                    write!(f, "{}", field)?;
                } else {
                    write!(f, "00")?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl FromXmlNode for Board {
    fn from_node(node: &XmlNode) -> SCResult<Self> {
        Ok(Self::filling_radius(6, node.childs_by_name("fields")
            .flat_map(|child| child.childs_by_name("field")
                .map(|f| Ok((
                    CubeCoords::new(
                        f.attribute("x")?.parse()?,
                        f.attribute("y")?.parse()?,
                        f.attribute("z")?.parse()?
                    ).into(),
                    Field::from_node(f)?
                ))))
            .collect::<SCResult<HashMap<AxialCoords, Field>>>()?
        ))
    }
}
