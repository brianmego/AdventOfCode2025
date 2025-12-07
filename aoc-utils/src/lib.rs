use itertools::Itertools;
use nom::{
    character::complete::{newline, one_of},
    combinator::map,
    multi::{many0, many1},
    sequence::terminated,
    IResult,
};
use num::Integer;
use std::fmt::{Debug, Display};

#[derive(Debug)]
pub struct BadTileTypeError;

pub fn parse_tile_type<T>(inp: &str) -> IResult<&str, T>
where
    T: ParseableCharacters + TryFrom<char>,
    <T as TryFrom<char>>::Error: Debug,
{
    let valid_tile_chars = T::valid_chars().iter().join("");
    let res = map(one_of(valid_tile_chars.as_str()), |c| {
        T::try_from(c).unwrap()
    })(inp);
    res
}

pub fn parse_collection<T>(inp: &str) -> IResult<&str, Collection<T>>
where
    T: ParseableCharacters + TryFrom<char> + Copy + PartialEq,
    <T as TryFrom<char>>::Error: Debug,
{
    let (inp, rows) = many1(terminated(many1(parse_tile_type), newline))(inp)?;
    let mut collection = Collection(vec![]);
    for (row_num, row) in rows.iter().enumerate() {
        for (col_num, tile) in row.iter().enumerate() {
            collection.push(Tile::new(
                *tile,
                Loc::new(col_num as isize, row_num as isize),
            ));
        }
    }
    Ok((inp, collection))
}

pub fn parse_collection_group<T>(inp: &str) -> IResult<&str, CollectionGroup<T>>
where
    T: ParseableCharacters + TryFrom<char> + Copy + PartialEq,
    <T as TryFrom<char>>::Error: Debug,
{
    many1(terminated(parse_collection, many0(newline)))(inp)
}

// MODELS
pub trait ParseableCharacters {
    fn valid_chars() -> Vec<char>;
}

#[derive(Debug, PartialEq, Copy, Clone, PartialOrd, Eq, Ord)]
pub struct Loc {
    x: isize,
    y: isize,
}
impl Display for Loc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?},{:?}", self.x, self.y)
    }
}

impl Loc {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
    pub fn get_nearby(&self, direction: Direction, distance: isize) -> Option<Self> {
        Some(match direction {
            Direction::North => Self::new(self.x, self.y.checked_sub(distance)?),
            Direction::East => Self::new(self.x.checked_add(distance)?, self.y),
            Direction::South => Self::new(self.x, self.y.checked_add(distance)?),
            Direction::West => Self::new(self.x.checked_sub(distance)?, self.y),
            Direction::NorthEast => {
                Self::new(self.x.checked_add(distance)?, self.y.checked_sub(distance)?)
            }
            Direction::SouthEast => {
                Self::new(self.x.checked_add(distance)?, self.y.checked_add(distance)?)
            }
            Direction::SouthWest => {
                Self::new(self.x.checked_sub(distance)?, self.y.checked_add(distance)?)
            }
            Direction::NorthWest => {
                Self::new(self.x.checked_sub(distance)?, self.y.checked_sub(distance)?)
            }
        })
    }
    pub fn get_x(&self) -> isize {
        self.x
    }
    pub fn get_y(&self) -> isize {
        self.y
    }
    pub fn connect_with_line(&self, other: Loc) -> Vec<Loc> {
        let mut y_slope = other.y - self.y;
        let mut x_slope = other.x - self.x;
        let divisor = x_slope.gcd(&y_slope);
        x_slope /= divisor;
        y_slope /= divisor;
        let mut traveling_point = *self;
        let mut line_points = vec![];
        while traveling_point != other {
            traveling_point.x += x_slope;
            traveling_point.y += y_slope;
            line_points.push(traveling_point);
        }
        line_points.pop();
        line_points
    }
}

#[derive(PartialEq, Debug)]
pub struct Row<'a, T>(Vec<&'a Tile<T>>);
impl<'a, T> IntoIterator for Row<'a, T> {
    type Item = &'a Tile<T>;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
pub type Column<'a, T> = Row<'a, T>;
impl<T> Display for Row<'_, T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out: String = self.0.iter().map(|t| t.to_string()).join("");
        f.write_str(&out)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Collection<T>(Vec<Tile<T>>);
impl<T> Collection<T>
where
    T: ParseableCharacters + Copy + TryFrom<char> + PartialEq,
{
    fn push(&mut self, tile: Tile<T>) {
        self.0.push(tile)
    }
    pub fn get_row(&self, row_num: isize) -> Row<'_, T> {
        Row(self.0.iter().filter(|t| t.loc.y == row_num).collect())
    }
    pub fn get_column(&self, col_num: isize) -> Column<'_, T> {
        Row(self.0.iter().filter(|t| t.loc.x == col_num).collect())
    }
    pub fn count_rows(&self) -> usize {
        self.0.iter().unique_by(|t| t.loc.y).count()
    }
    pub fn count_columns(&self) -> usize {
        self.0.iter().unique_by(|t| t.loc.x).count()
    }
    pub fn count_tile_type(&self, tile_type: &T) -> usize {
        self.tiles()
            .iter()
            .filter(|t| t.get_type() == tile_type)
            .count()
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub fn tiles(&self) -> &Vec<Tile<T>> {
        &self.0
    }
    pub fn get_tile(&self, loc: Loc) -> Option<&Tile<T>> {
        self.get_row(loc.y)
            .0
            .iter()
            .find(|t| t.loc.x == loc.x)
            .copied()
    }

    pub fn from_puzzle_input(puzzle_input: &str) -> Collection<T>
    where
        <T as TryFrom<char>>::Error: Debug,
    {
        parse_collection(puzzle_input).unwrap().1
    }
}

pub type CollectionGroup<T> = Vec<Collection<T>>;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    North,
    East,
    South,
    West,
    NorthEast,
    SouthEast,
    SouthWest,
    NorthWest,
}
impl Direction {
    pub fn rotate_clockwise(&self) -> Self {
        match self {
            Direction::North => Self::East,
            Direction::East => Self::South,
            Direction::South => Self::West,
            Direction::West => Self::North,
            Direction::NorthEast => Self::SouthEast,
            Direction::SouthEast => Self::SouthWest,
            Direction::SouthWest => Self::NorthWest,
            Direction::NorthWest => Self::NorthEast,
        }
    }

    pub fn rotate_counterclockwise(&self) -> Self {
        match self {
            Direction::North => Self::West,
            Direction::East => Self::North,
            Direction::South => Self::East,
            Direction::West => Self::South,
            Direction::NorthEast => Self::NorthWest,
            Direction::SouthEast => Self::NorthEast,
            Direction::SouthWest => Self::SouthEast,
            Direction::NorthWest => Self::SouthWest,
        }
    }
    pub fn get_cardinal() -> Vec<Self> {
        vec![
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ]
    }
    pub fn get_all() -> Vec<Self> {
        vec![
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
            Direction::NorthEast,
            Direction::SouthEast,
            Direction::SouthWest,
            Direction::NorthWest,
        ]
    }
}

#[derive(Debug, Copy, Clone, Ord, Eq, PartialOrd)]
pub struct Tile<T> {
    tile_type: T,
    loc: Loc,
}
impl<T> Tile<T> {
    pub fn get_type(&self) -> &T {
        &self.tile_type
    }
    pub fn get_type_owned(self) -> T {
        self.tile_type
    }
    pub fn set_type(&mut self, new_type: T) {
        self.tile_type = new_type;
    }
    pub fn loc(&self) -> &Loc {
        &self.loc
    }
}
impl<T> PartialEq for Tile<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.tile_type == other.tile_type
    }
}

impl<T> Tile<T> {
    pub fn new(tile_type: T, loc: Loc) -> Self {
        Self { tile_type, loc }
    }
}

impl<T> Display for Tile<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.tile_type.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[derive(Debug, PartialEq, Copy, Clone)]
    enum LavaTile {
        Ash,
        Rocks,
    }
    impl Display for LavaTile {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(match self {
                LavaTile::Ash => ".",
                LavaTile::Rocks => "#",
            })
        }
    }

    impl TryFrom<char> for LavaTile {
        type Error = BadTileTypeError;

        fn try_from(value: char) -> Result<Self, Self::Error> {
            match value {
                '.' => Ok(Self::Ash),
                '#' => Ok(Self::Rocks),
                _ => unreachable!(),
            }
        }
    }
    impl ParseableCharacters for LavaTile {
        fn valid_chars() -> Vec<char> {
            vec!['#', '.']
        }
    }

    #[test_case(".", Ok(("", LavaTile::Ash)); "Ash")]
    #[test_case("#", Ok(("", LavaTile::Rocks)); "Rocks")]
    fn test_tile_type(inp: &str, exp: IResult<&str, LavaTile>) {
        let actual = parse_tile_type(inp);
        assert_eq!(actual, exp);
    }

    #[test]
    fn test_parse_collection() {
        let inp = include_str!("./data/lava_sample.txt");
        let actual: Collection<LavaTile> = parse_collection(inp).unwrap().1;
        assert_eq!(actual.0.len(), 63);
    }

    #[test]
    fn test_parse_collection_group() {
        let inp = include_str!("./data/lava_sample.txt");
        let actual = parse_collection_group::<LavaTile>(inp);
        assert!(actual.is_ok());
        let unwrapped = actual.unwrap();
        assert_eq!(unwrapped.0, "");
        assert_eq!(unwrapped.1[0].0.len(), 63);
        assert_eq!(unwrapped.1[1].0.len(), 63);
    }

    #[test_case((Loc::new(1,1), Loc::new(5, 5)), vec![Loc::new(2, 2), Loc::new(3, 3), Loc::new(4, 4)])]
    #[test_case((Loc::new(1,1), Loc::new(-2, -2)), vec![Loc::new(0,0), Loc::new(-1, -1)])]
    #[test_case((Loc::new(1,1), Loc::new(-2, -5)), vec![Loc::new(0, -1), Loc::new(-1, -3)])]
    fn test_loc_connect_with_line((point_1, point_2): (Loc, Loc), exp: Vec<Loc>) {
        let actual = point_1.connect_with_line(point_2);
        assert_eq!(actual, exp);
    }
}
