#![allow(dead_code, unused)]

use skan::scanner::Scanner;
use std::fs::{self, File};
use std::io::{self, Read, Write};

fn main() {
    let mut scan = Scanner::from_str("23 78 9");
    scan.next_word();
    println!("{:?}", scan.next_word());
    /*for value in scan {
        print!("{:}", value as char);
    }

    print!("\nEnter words: (Ctrl+D to Exit): ");
    io::stdout().flush().unwrap();
    let scan = Scanner::new(io::stdin());
    for value in scan {
        print!("{:}", value as char);
    }*/
}
