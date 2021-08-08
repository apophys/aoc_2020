
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
    fn row(&self) -> i32 {
        resolve(self.row, 'B')
    }

    fn column(&self) -> i32 {
        resolve(self.column, 'R')
    }

    pub fn seat_id(&self) -> i32 {
        self.row() * 8 + self.column()
    }
}

fn resolve(vector: &str, one: char) -> i32 {
    vector.chars().rev().enumerate().map(|(i, c)| if c == one {2_i32.pow(i as u32)} else {0}).sum()
}
