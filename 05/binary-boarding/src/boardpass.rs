#[derive(Debug)]
pub struct BoardingPass<'a> {
    row: &'a str,
    column: &'a str,
}

impl<'a> BoardingPass<'a> {
    pub fn new(source: &'a str) -> BoardingPass<'a> {
        BoardingPass {
            row: &source[..7],
            column: &source[7..],
        }
    }
}

impl<'a> BoardingPass<'a> {
    pub fn row(&self) -> i32 {
        resolve(self.row, 'F', 'B')
    }

    pub fn column(&self) -> i32 {
        resolve(self.column, 'L', 'R')
    }

    pub fn seat_id(&self) -> i32 {
        self.row() * 8 + self.column()
    }
}

fn resolve(vector: &str, lower: char, upper: char) -> i32 {
    let (mut lower_value, mut upper_value) = (0, 2_i32.pow(vector.len() as u32) - 1);

    let mut last_char: char = '.';

    for character in vector.chars() {
        if character == lower {
            upper_value = upper_value - (upper_value - lower_value) / 2 - 1;
        }
        if character == upper {
            lower_value = lower_value + (upper_value - lower_value) / 2 + 1;
        }
        last_char = character;
    }

    if last_char == lower {
        lower_value
    } else if last_char == upper {
        upper_value
    } else {
        unreachable!()
    }
}
