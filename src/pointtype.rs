use std::fmt::{Formatter, Show};
use std::fmt;

pub enum PointType {
    Circle = 7,
    Square = 5,
    Triangle = 9,
}

impl Show for PointType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", *self as int)
    }
}
