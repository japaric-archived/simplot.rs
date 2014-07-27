use std::fmt::{Formatter,Show};
use std::fmt;

pub enum Terminal {
    Png,
    Svg,
}

impl Show for Terminal {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Png => {
                write!(f, "pngcairo")
            },
            Svg => {
                write!(f, "svg dynamic")
            },
        }
    }
}
