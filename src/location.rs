#[derive(Debug, Clone, Copy)]
pub struct Location {
    col: usize,
    line: usize,
    index: usize,
}

impl Location {
    pub fn col(&self) -> usize {
        self.col
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn new() -> Self {
        Location {
            col: 1,
            line: 1,
            index: 0,
        }
    }

    pub fn advance(&mut self, ch: char) {
        self.index += 1;
        if ch == '\n' {
            self.line += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }
    }
    pub fn backtrack(&mut self) {
        self.index -= 1;
        self.col -= 1;
    }
}
