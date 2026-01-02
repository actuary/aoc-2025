use std::{fmt, ops, slice};

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, PartialOrd, Ord)]
pub struct Position {
    pub x: i64,
    pub y: i64,
}

impl fmt::Display for Position {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl ops::Mul<i64> for Position {
    type Output = Position;

    fn mul(self, _rhs: i64) -> Position {
        Position {
            x: self.x * _rhs,
            y: self.y * _rhs,
        }
    }
}

impl ops::Mul<&Position> for i64 {
    type Output = Position;

    fn mul(self, _rhs: &Position) -> Position {
        Position {
            x: _rhs.x * self,
            y: _rhs.y * self,
        }
    }
}

impl<'a, 'b> ops::Add<&'b Position> for &'a Position {
    type Output = Position;

    fn add(self, _rhs: &'b Position) -> Position {
        Position {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
        }
    }
}

impl<'a> ops::Add<Position> for &'a Position {
    type Output = Position;

    fn add(self, _rhs: Position) -> Position {
        Position {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
        }
    }
}

impl ops::Add<Position> for Position {
    type Output = Position;

    fn add(self, _rhs: Position) -> Position {
        Position {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
        }
    }
}

impl ops::Mul<Position> for i64 {
    type Output = Position;

    fn mul(self, _rhs: Position) -> Position {
        Position {
            x: _rhs.x * self,
            y: _rhs.y * self,
        }
    }
}

impl ops::Rem<&Position> for Position {
    type Output = Position;

    fn rem(self, _rhs: &Position) -> Position {
        Position {
            x: _rhs.x.rem_euclid(self.x),
            y: _rhs.y.rem_euclid(self.y),
        }
    }
}
impl ops::Sub<Position> for Position {
    type Output = Position;

    fn sub(self, _rhs: Position) -> Position {
        Position {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
        }
    }
}

pub const GRID_DIRECTIONS: [Position; 8] = [
    Position { x: -1, y: -1 },
    Position { x: -1, y: 0 },
    Position { x: -1, y: 1 },
    Position { x: 0, y: -1 },
    Position { x: 0, y: 1 },
    Position { x: 1, y: -1 },
    Position { x: 1, y: 0 },
    Position { x: 1, y: 1 },
];

#[derive(Debug)]
pub struct Cell<T> {
    pub value: T,
    pub position: Position,
}

#[derive(Debug)]
pub struct Grid<T> {
    pub cells: Vec<Vec<Cell<T>>>,
}

impl<T> Grid<T> {
    pub fn new(cells: Vec<Vec<T>>) -> Self {
        if cells.len() == 0 {
            return Grid { cells: vec![] };
        }

        if cells.len() > 1 && cells.iter().any(|row| row.len() != cells[0].len()) {
            panic!("Grid rows must be the same size")
        }

        Grid {
            cells: cells
                .into_iter()
                .enumerate()
                .map(|(x, row)| {
                    row.into_iter()
                        .enumerate()
                        .map(|(y, val)| Cell {
                            value: val,
                            position: Position {
                                x: x as i64,
                                y: y as i64,
                            },
                        })
                        .collect()
                })
                .collect(),
        }
    }

    pub fn at(&self, position: &Position) -> Option<&Cell<T>> {
        if position.x < 0 || position.y < 0 {
            return None;
        }

        if position.x as usize >= self.cells.len() || position.y as usize >= self.cells[0].len() {
            return None;
        }

        return Some(&self.cells[position.x as usize][position.y as usize]);
    }

    pub fn at_mut(&mut self, position: &Position) -> Option<&mut Cell<T>> {
        if position.x < 0 || position.y < 0 {
            return None;
        }

        if position.x as usize >= self.cells.len() || position.y as usize >= self.cells[0].len() {
            return None;
        }

        return Some(&mut self.cells[position.x as usize][position.y as usize]);
    }

    pub fn neighbours(&self, position: &Position) -> Vec<&Cell<T>> {
        let mut result = vec![];
        for movement in GRID_DIRECTIONS {
            match self.at(&(position + movement)) {
                Some(cell) => result.push(cell),
                None => (),
            }
        }

        result
    }

    pub fn size(&self) -> Position {
        if self.cells.len() == 0 {
            return Position { x: 0, y: 0 };
        }

        return Position {
            x: self.cells.len() as i64,
            y: self.cells[0].len() as i64,
        };
    }
}

impl<'a, T> IntoIterator for &'a Grid<T> {
    type Item = &'a Cell<T>;
    type IntoIter = GridIntoIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        GridIntoIterator {
            grid: self,
            index: Position { x: 0, y: 0 },
        }
    }
}

impl<'a, T> Iterator for GridIntoIterator<'a, T> {
    type Item = &'a Cell<T>;
    fn next(&mut self) -> Option<&'a Cell<T>> {
        if self.grid.size() == (Position { x: 0, y: 0 }) {
            return None;
        }

        if self.index.x > self.grid.size().x - 1 || self.index.y > self.grid.size().y - 1 {
            return None;
        }

        let result = self
            .grid
            .at(&self.index)
            .expect("Iterator position not in bounds.");

        if self.index.y >= self.grid.size().y - 1 {
            self.index = Position {
                x: self.index.x + 1,
                y: 0,
            };
        } else {
            self.index = Position {
                x: self.index.x,
                y: self.index.y + 1,
            };
        }

        Some(result)
    }
}

pub struct GridIntoIterator<'a, T> {
    grid: &'a Grid<T>,
    index: Position,
}

pub fn gcd(a: i64, b: i64) -> i64 {
    match (a, b) {
        (d, 0) => d,
        (c, d) => gcd(d, c % d),
    }
}

pub fn lcm(a: i64, b: i64) -> i64 {
    (a * b).abs() / gcd(a, b)
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum Move {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
}

impl Move {
    pub fn advance_by(&self) -> Position {
        match self {
            Move::Up => Position { x: -1, y: 0 },
            Move::Right => Position { x: 0, y: 1 },
            Move::Down => Position { x: 1, y: 0 },
            Move::Left => Position { x: 0, y: -1 },
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd)]
pub enum Direction {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

impl Direction {
    pub fn from(c: char) -> Result<Direction, ()> {
        match c {
            '^' => Ok(Direction::North),
            '>' => Ok(Direction::East),
            'v' => Ok(Direction::South),
            '<' => Ok(Direction::West),
            _ => Err(()),
        }
    }

    pub fn advance(&self) -> (i32, i32) {
        match self {
            Self::North => (-1, 0),
            Self::East => (0, 1),
            Self::South => (1, 0),
            Self::West => (0, -1),
        }
    }

    pub fn turn(&self) -> Self {
        match *self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }

    pub fn iterator() -> slice::Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 4] = [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ];
        DIRECTIONS.iter()
    }

    pub fn advance_by(&self) -> Position {
        match self {
            Direction::North => Position { x: -1, y: 0 },
            Direction::East => Position { x: 0, y: 1 },
            Direction::South => Position { x: 1, y: 0 },
            Direction::West => Position { x: 0, y: -1 },
        }
    }
}

pub fn triangular(start: u64, end: u64) -> u64 {
    assert!(start <= end);
    if start == 0 {
        return end * (end + 1) / 2;
    }

    triangular(0, end) - triangular(0, start - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_create() {
        let values: Vec<Vec<i64>> = vec![vec![1, 2, 3], vec![3, 2, 1]];
        let grid = Grid::new(values);
        assert_eq!(grid.cells.len(), 2);
        assert_eq!(grid.cells[0].len(), 3);
    }

    #[test]
    fn grid_empty() {
        let values: Vec<Vec<i64>> = vec![];
        let grid = Grid::new(values);
        assert_eq!(grid.cells.len(), 0);
    }

    #[test]
    fn grid_at_valid_position() {
        let values: Vec<Vec<i64>> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let grid = Grid::new(values);

        grid.at(&Position { x: 0, y: 0 }).unwrap();
    }

    #[test]
    fn grid_at_invalid_position() {
        let values: Vec<Vec<i64>> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let grid = Grid::new(values);

        assert!(grid.at(&Position { x: 2, y: 3 }).is_none());
    }

    #[test]
    fn grid_neighbours_all() {
        let values: Vec<Vec<i64>> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

        let grid = Grid::new(values);

        let neighbours = grid.neighbours(&Position { x: 1, y: 1 });
        assert_eq!(neighbours.len(), 8);
    }

    #[test]
    fn grid_neighbours_corner() {
        let values: Vec<Vec<i64>> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

        let grid = Grid::new(values);

        let neighbours = grid.neighbours(&Position { x: 0, y: 0 });
        assert_eq!(neighbours.len(), 3);
        assert_eq!(neighbours.iter().map(|cell| cell.value).sum::<i64>(), 11);

        let neighbours = grid.neighbours(&Position { x: 2, y: 2 });
        assert_eq!(neighbours.len(), 3);
        assert_eq!(neighbours.iter().map(|cell| cell.value).sum::<i64>(), 19);
    }

    #[test]
    fn grid_neighbours_side() {
        let values: Vec<Vec<i64>> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

        let grid = Grid::new(values);

        let neighbours = grid.neighbours(&Position { x: 0, y: 1 });
        assert_eq!(neighbours.len(), 5);
        assert_eq!(neighbours.iter().map(|cell| cell.value).sum::<i64>(), 19);
    }

    #[test]
    fn grid_iterator() {
        let values: Vec<Vec<i64>> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

        let grid = Grid::new(values);

        let res: i64 = grid.into_iter().map(|c| c.value).sum();
        assert_eq!(res, 45);
    }
}
