use std::fmt::{Formatter,Show};
use std::fmt;

pub enum LineType {
    Dash = 2,
    Dot = 3,
    DotDash = 4,
    DotDotDash = 5,
    SmallDot = 0,
    Solid = 1,
}

impl Show for LineType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", *self as int)
    }
}
