#![allow(unused)]
#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub line: usize,
}

impl Span {
    pub fn new(line: usize, start: usize, end: usize) -> Self {
        Self { start, end, line }
    }
    pub fn right_shift(&mut self, ch: char) {
        if ch == '\0' {
            return;
        }
        if ch == '\n' {
            self.line += 1;
        }
        self.end += ch.to_string().as_bytes().len();
    }

    pub fn range(&self) -> std::ops::Range<usize> {
        self.start..self.end
    }
}

impl From<&Span> for Span {
    fn from(other: &Self) -> Self {
        Self {
            start: other.end,
            end: other.end,
            line: other.line,
        }
    }
}

impl From<(Span, Span)> for Span {
    fn from((x, y): (Self, Self)) -> Self {
        Self {
            start: x.start,
            end: y.end,
            line: x.line,
        }
    }
}
