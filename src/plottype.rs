use std::fmt::{Formatter, Show};
use std::fmt;

pub enum PlotType {
    Lines,
    Points,
}

impl Show for PlotType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Lines => {
                write!(f, "lines")
            },
            Points => {
                write!(f, "points")
            },
        }
    }
}
