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

#[test]
fn test_next_bytes_function() {
    let mut scan = Scanner::from_str("Green latern");
    scan.next(); // 0 - G
    scan.next(); // 1 - r
    scan.next(); // 3 - e
    scan.next(); // 4 - e
    scan.next(); // 5 - n
    scan.next(); // 6 - ' '
    assert_eq!(scan.next_byte(), Some(108)); // next?
}
