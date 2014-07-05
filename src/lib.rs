#![allow(unused_must_use)]
#![feature(macro_rules)]

// FIXME rust-lang/rust#15319 Remove `crate_id`
#![crate_id = "simplot"]

pub use figure::Figure;

mod data;
mod figure;
mod line;
