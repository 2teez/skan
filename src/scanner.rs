#![allow(dead_code, unused)]
//! skan crate
//! skan is a mini scanner like java scanner
//! use to get input from user from the cli

use crate::numbers::Int;
use std::error::Error;
use std::fmt::{self, Display};
use std::io::{BufReader, Cursor, Read};
use std::result::Result;

#[derive(Debug)]
pub enum ScannerError<E> {
    NoMoreData,
    ParseError(E),
}

impl<E: std::fmt::Debug + std::fmt::Display> Display for ScannerError<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ScannerError::NoMoreData => write!(f, "no more data"),
            ScannerError::ParseError(e) => write!(f, "Parse error {}", e),
        }
    }
}

impl<E: std::fmt::Debug + std::fmt::Display> Error for ScannerError<E> {}

#[derive(Debug, PartialEq)]
pub struct Scanner {
    data: Vec<u8>,
    wrds: Option<Vec<String>>,
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
            wrds: None,
        }
    }

    pub fn from_str(str: &str) -> Self {
        let mut data: Vec<u8> = Vec::new();
        let mut cursor = Cursor::new(str.as_bytes());
        cursor.read_to_end(&mut data).expect("Invalid stream");
        Self {
            data,
            counter: 0,
            wrds: None,
        }
    }

    ///
    /// has_next function doctest
    /// ```
    /// use skan::scanner::Scanner;
    /// assert!(Scanner::from_str("Got it.").has_next());
    /// ```
    pub fn has_next(&self) -> bool {
        if self.data.len() == 0 {
            return false;
        }
        self.counter < self.data.len()
    }

    ///
    /// next_byte function doctest
    /// ```
    /// use skan::scanner::Scanner;
    /// assert_eq!(Scanner::from_str("Hot it.").next_byte(), Some(72));
    ///
    pub fn next_byte(&self) -> Option<u8> {
        if self.has_next() {
            Some(self.data[self.counter])
        } else {
            None
        }
    }

    pub(crate) fn delimiter(&self, sep: char) -> Vec<String> {
        String::from_utf8_lossy(&self.data)
            .split(sep)
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect()
    }

    ///
    /// next_word function doctest
    /// ```
    /// use skan::scanner::Scanner;
    /// let mut nw = Scanner::from_str("Hot it.");
    /// nw.next_word();
    /// nw.next_word();
    /// assert_eq!(nw.next_word(), None);
    ///
    pub fn next_word(&mut self) -> Option<String> {
        if self.wrds.is_none() {
            self.wrds = Some(self.delimiter(' '));
        }
        let aword = self.wrds.as_ref().expect("can't use as reference");
        if self.counter < aword.len() {
            let word = aword[self.counter].clone();
            self.counter += 1;
            Some(word)
        } else {
            None
        }
    }

    ///
    /// next_line function doctest
    /// ```
    /// use skan::scanner::Scanner;
    /// let mut nw = Scanner::from_str("Hot it.\nGot it.");
    /// nw.next_line();
    /// assert_eq!(nw.next_line(), Some("Got it.".to_string()));
    ///
    pub fn next_line(&mut self) -> Option<String> {
        if self.wrds.is_none() {
            self.wrds = Some(self.delimiter('\n'));
        }
        let aword = self.wrds.as_ref().expect("can't use as reference");
        if self.counter < aword.len() {
            let word = aword[self.counter].clone();
            self.counter += 1;
            Some(word)
        } else {
            None
        }
    }

    ///
    /// next_number function doctest
    /// ```
    /// use skan::scanner::Scanner;
    /// let mut nw = Scanner::from_str("from 23 to 45");
    /// nw.next_word();
    /// nw.next_word();nw.next_word();
    /// assert_eq!(nw.next_number::<u32>().unwrap(), 45);
    ///
    pub fn next_number<T>(&mut self) -> Result<T, ScannerError<<T as std::str::FromStr>::Err>>
    where
        T: std::str::FromStr,
    {
        if self.wrds.is_none() {
            self.wrds = Some(self.delimiter(' '));
        }
        let aword = self.wrds.as_ref().expect("can't use as reference");

        if self.counter < aword.len() {
            let word = aword[self.counter].clone();
            self.counter += 1;
            word.trim_ascii()
                .parse::<T>()
                .map_err(ScannerError::ParseError)
        } else {
            Err(ScannerError::NoMoreData)
        }
    }

    ///
    /// next_int function doctest
    /// ```
    /// use skan::scanner::Scanner;
    /// use skan::numbers::Int;
    /// let mut nw = Scanner::from_str("from 47 until 100");
    /// nw.next_word();
    /// nw.next_word();nw.next_word();
    /// assert_eq!(nw.next_int::<i32>().unwrap(), 100);
    ///
    pub fn next_int<T>(&mut self) -> Result<T, ScannerError<<T as std::str::FromStr>::Err>>
    where
        T: Int,
    {
        self.next_number::<T>()
    }
}

impl Iterator for Scanner {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        return if self.has_next() {
            Some(
                self.data[{
                    let tmp = self.counter;
                    self.counter += 1;
                    tmp
                }],
            )
        } else {
            None
        };
    }
}

#[cfg(test)]
mod unittest;
