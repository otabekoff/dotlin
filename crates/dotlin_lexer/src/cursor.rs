use std::iter::Peekable;

pub struct Cursor<'a> {
    iter: Peekable<std::str::CharIndices<'a>>,
    src_len: usize,
}

impl<'a> Cursor<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            iter: src.char_indices().peekable(),
            src_len: src.len(),
        }
    }

    /// Return the byte index of the next character (or `src.len()` at EOF)
    pub fn pos(&mut self) -> usize {
        match self.iter.peek() {
            Some((i, _)) => *i,
            None => self.src_len,
        }
    }

    pub fn peek(&mut self) -> Option<char> {
        self.iter.peek().map(|(_, c)| *c)
    }

    pub fn next(&mut self) -> Option<char> {
        self.iter.next().map(|(_, c)| c)
    }

    pub fn next_if(&mut self, expected: char) -> bool {
        if let Some((_, ch)) = self.iter.peek() {
            if *ch == expected {
                self.iter.next();
                return true;
            }
        }
        false
    }
}
