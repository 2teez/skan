#![allow(dead_code, unused)]
//! skan crate
//! skan is a mini scanner like java scanner
//! use to get input from user from the cli

use std::io::{self, BufRead, BufReader, Cursor, Read, Stdin};

#[derive(Debug, PartialEq)]
pub struct Scanner {
    data: Vec<u8>,
    counter: usize,
}

impl Scanner {
    pub fn new<T: Read>(f: T) -> Self {
        let mut values: Vec<u8> = Vec::new();
        let mut buf = BufReader::new(f);
        buf.read_to_end(&mut values).expect("Invalid read");
        Self {
            data: values,
            counter: 0,
        }
    }

    pub fn from_str(str: &str) -> Self {
        let mut data: Vec<u8> = Vec::new();
        let mut cursor = Cursor::new(str.as_bytes());
        cursor.read_to_end(&mut data).expect("Invalid stream");
        Self { data, counter: 0 }
    }

    ///
    /// has_next function doctest
    /// ```
    /// use skan::scanner::Scanner;
    /// assert_eq!(Scanner::from_str("Got it.").has_next(), true)
    /// ```
    pub fn has_next(&self) -> bool {
        return if self.counter < self.data.len() - 1 {
            true
        } else {
            false
        };
    }
}

impl Iterator for Scanner {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        return if self.has_next() {
            Some(self.data[self.counter])
        } else {
            None
        };
    }
}

#[cfg(test)]
mod unittest;
