#![allow(unused_must_use)]
#![feature(macro_rules)]

#![crate_name = "simplot"]

pub use figure::Figure;
pub use option::PlotOption;

mod data;
mod figure;
mod line;

pub mod linetype;
pub mod option;
pub mod plottype;
