//! skan crates unitest
//!
//! The Library skan shows all the unittest here
//!

use super::*;
use std::io::Cursor;

#[test]
fn test_new_associate_function() {
    let got = Scanner::from_str("Hello, World");
    let mut cursor = Cursor::new("Hello, World");
    let mut vec = Vec::new();
    cursor.read_to_end(&mut vec).unwrap();
    assert_eq!(
        got,
        Scanner {
            data: vec,
            counter: 0
        }
    );
}
