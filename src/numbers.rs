#![allow(dead_code, unused)]
//! skan crate
//! skan is a mini scanner like java scanner
//! use to get input from user from the cli

mod private {
    pub trait Sealed {}
}

pub trait Int: private::Sealed + std::str::FromStr {}
pub trait Float: private::Sealed + std::str::FromStr {}

impl private::Sealed for i8 {}
impl private::Sealed for i16 {}
impl private::Sealed for i32 {}
impl private::Sealed for i64 {}

impl private::Sealed for u8 {}
impl private::Sealed for u16 {}
impl private::Sealed for u32 {}
impl private::Sealed for u64 {}

impl Int for i8 {}
impl Int for i16 {}
impl Int for i32 {}
impl Int for i64 {}

impl Int for u8 {}
impl Int for u16 {}
impl Int for u32 {}
impl Int for u64 {}

impl private::Sealed for f32 {}
impl private::Sealed for f64 {}

impl Float for f32 {}
impl Float for f64 {}
