extern crate limited_table;

use std::convert::Into;

use limited_table::{Key, LimitedTable};

pub type Token = Key;

pub struct TokenGenerator {
    limited: LimitedTable<()>,
}

impl TokenGenerator {
    pub fn new<T: Into<Token>>(begin: T, limit: usize) -> Self {
        Self {
            limited: LimitedTable::new(begin.into(), limit),
        }
    }

    pub fn is_assigned(&self, token: Token) -> bool {
        self.limited.contains(token)
    }

    pub fn gen(&mut self) -> Option<Token> {
        self.limited.insert(())
    }

    pub fn restore(&mut self, token: Token) -> bool {
        self.limited.remove(token).is_some()
    }
}
