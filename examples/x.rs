extern crate simplot;

use std::io::stdio;

use simplot::Figure;
use simplot::linetype::SmallDot;
use simplot::option::{LineType,PointType,Title};
use simplot::plottype::{Lines,Points};
use simplot::pointtype::Square;

fn main() {
    Figure::new().
        set_output_file(Path::new("output.png")).
        set_size((1366, 768)).
        set_title("X").
        set_xlabel("x -->").
        set_ylabel("y -->").
        plot(Points,
             range(0, 20u),
             range(0, 20u),
             [PointType(Square), Title("Rising")]).
        plot(Lines, range(0, 20u), range(0, 20u).rev(), [LineType(SmallDot)]).
        echo(&mut stdio::stdout()).
        draw();
}
