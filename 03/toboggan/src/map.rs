use std::error::Error;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Map(Vec<MapRow>);

#[derive(Debug, Clone)]
pub struct MapRow(Vec<bool>);

#[derive(Debug, Clone)]
pub enum ParseMapRowError {
    InvalidCharacter(String),
}

impl Error for ParseMapRowError {}

impl fmt::Display for ParseMapRowError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseMapRowError::InvalidCharacter(message) => write!(f, "{}", message),
        }
    }
}

#[derive(Debug)]
pub enum ParseMapError {
    RowError(ParseMapRowError),
}

impl fmt::Display for ParseMapError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RowError(error) => write!(f, "{}", error),
        }
    }
}

impl Error for ParseMapError {}

impl From<ParseMapRowError> for ParseMapError {
    fn from(error: ParseMapRowError) -> Self {
        Self::RowError(error)
    }
}

impl FromStr for MapRow {
    type Err = ParseMapRowError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut row: Vec<bool> = Vec::with_capacity(s.len());

        for character in s.chars() {
            match character {
                '.' => row.push(false),
                '#' => row.push(true),
                _ => {
                    return Err(ParseMapRowError::InvalidCharacter(format!(
                        "Found invalid character {}",
                        character
                    )))
                }
            }
        }
        Ok(MapRow(row))
    }
}

impl FromStr for Map {
    type Err = ParseMapError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map: Result<_, _> = s.lines().map(str::parse).collect();

        Ok(Map(map?))
    }
}

impl MapRow {
    pub fn is_tree_at(&self, index: usize) -> bool {
        self.0[index % self.0.len()]
    }
}

impl IntoIterator for Map {
    type Item = MapRow;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
