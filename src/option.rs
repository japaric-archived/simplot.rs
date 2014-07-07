use linetype::LineType;
use pointtype::PointType;

pub enum PlotOption<'a> {
    LineType(LineType),
    PointType(PointType),
    Title(&'a str),
}
