# Skan

`skan` is a small Rust crate that provides functionality similar to Java's `Scanner` class. It allows reading input from various sources (strings, files, etc.) and parsing them into different types, such as words, lines, integers, and floats.

---

## Features

- Create a scanner from a string or any type implementing `Read`.
- Read input byte-by-byte, word-by-word, or line-by-line.
- Parse numbers (integers, floats, doubles) with error handling.
- Iterator support for byte-wise reading.
- Custom errors for parsing and end-of-data scenarios.

---

## Installation

You can install `skan` either from Crates.io or directly from the Git repository.

### Installing from Crates.io

1. Open your `Cargo.toml` file.
2. Add `skan` to your dependencies:

```toml
[dependencies]
skan = "0.1.0"
```

3. In your Rust code, import the scanner:

```rust
use skan::scanner::Scanner;
```

4. Create a `Scanner` instance to read input:

```rust
let mut sc = Scanner::from_str("Hello world");
let word = sc.next_word();
```
### Installing directly from Git

1. Open your `Cargo.toml` file.
2. Add `skan` as a dependency using the Git repository URL:

```toml
[dependencies]
skan = { git = "https://github.com/2teez/skan" }
```

3. Import and use the crate as usual:

```rust
use skan::scanner::Scanner;
let mut sc = Scanner::from_str("Hello world");
let word = sc.next_word();
```

---

## Modules

### `scanner`

This module contains the main `Scanner` struct and related functionality.

#### `ScannerError<E>`

An enum representing possible scanner errors:

- `NoMoreData` – when there are no more items to read.
- `ParseError(E)` – when parsing a string into a number fails.

Implements `Display` and `Error` traits for descriptive error messages.

#### `Scanner` struct

Holds the scanner state:

- `data: Vec<u8>` – the raw input bytes.
- `wrds: Option<Vec<String>>` – optional cached split words/lines.
- `counter: usize` – current reading position.

---

## Creating a Scanner

#### From a reader:

```rust
use skan::scanner::Scanner;
use std::fs::File;

let file = File::open("input.txt").unwrap();
let mut scanner = Scanner::new(file);
```

#### From a string:

```rust
let mut scanner = Scanner::from_str("Hello World");
```

---

## Methods

#### `has_next() -> bool`

Checks if there is more data to read.

```rust
let scanner = Scanner::from_str("Got it.");
assert!(scanner.has_next());
```

---

#### `next_byte() -> Option<u8>`

Returns the next byte from the input without advancing the counter if there is data.

```rust
let scanner = Scanner::from_str("Hot it.");
assert_eq!(scanner.next_byte(), Some(72)); // 'H'
```

---

#### `next_word() -> Option<String>`

Returns the next word (split by space) from the input.

```rust
let mut scanner = Scanner::from_str("Hot it.");
scanner.next_word();
scanner.next_word();
assert_eq!(scanner.next_word(), None);
```

---

#### `next_line() -> Option<String>`

Returns the next line (split by `\n`) from the input.

```rust
let mut scanner = Scanner::from_str("Hot it.\nGot it.");
scanner.next_line();
assert_eq!(scanner.next_line(), Some("Got it.".to_string()));
```

---

#### `next_number<T>() -> Result<T, ScannerError<T::Err>>`

Parses the next word as a number of type `T`.

```rust
let mut scanner = Scanner::from_str("from 23 to 45");
scanner.next_word();
scanner.next_word();
scanner.next_word();
assert_eq!(scanner.next_number::<u32>().unwrap(), 45);
```

---

#### `next_int<T>() -> Result<T, ScannerError<T::Err>>`

Parses the next word as an integer. Requires `T` to implement the `Int` trait.

```rust
use skan::numbers::Int;
let mut scanner = Scanner::from_str("from 47 until 100");
scanner.next_word();
scanner.next_word();
scanner.next_word();
assert_eq!(scanner.next_int::<i32>().unwrap(), 100);
```

---

#### `next_float<T>() -> Result<T, ScannerError<T::Err>>`

Parses the next word as a floating-point number. Requires `T` to implement the `Float` trait.

```rust
use skan::numbers::Float;
let mut scanner = Scanner::from_str("from 3.194");
scanner.next_word();
assert_eq!(scanner.next_float::<f32>().unwrap(), 3.194);
```

---

#### `next_double() -> Result<f64, ScannerError<f64::Err>>`

Parses the next word as an `f64` floating-point number.

```rust
let mut scanner = Scanner::from_str("6.9669987654230");
assert_eq!(scanner.next_double().unwrap(), 6.9669987654230);
```

---

#### `Iterator` Implementation

`Scanner` implements `Iterator<Item = u8>` to read input byte-by-byte.

```rust
let mut scanner = Scanner::from_str("Hello");
for byte in &mut scanner {
    println!("{}", byte); // prints ASCII values
}
```

---

### Internal Helper Methods

- `delimiter(&self, sep: char) -> Vec<String>` – splits the input by the given separator (`' '` or `'\n'`) and returns non-empty strings.

---

## Example Usage

```rust
use skan::scanner::Scanner;
use skan::numbers::{Int, Float};

let mut scanner = Scanner::from_str("Age 30, Score 95.5");

scanner.next_word(); // skip "Age"
let age: i32 = scanner.next_int().unwrap();

scanner.next_word(); // skip "Score"
let score: f32 = scanner.next_float().unwrap();

println!("Age: {}, Score: {}", age, score);
```

---

## Error Handling

`ScannerError` helps identify when parsing fails or when there is no more data. Always handle `Result` when calling number parsing methods:

```rust
match scanner.next_int::<i32>() {
    Ok(value) => println!("Next int: {}", value),
    Err(e) => eprintln!("Error: {}", e),
}
```

---
