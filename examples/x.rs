extern crate simplot;

use std::io::stdio;

use simplot::Figure;
use simplot::option::LineType;
use simplot::linetype::{SmallDot,Solid};

fn main() {
    Figure::new().
        set_output_file(Path::new("output.png")).
        set_size((1366, 768)).
        set_title("X").
        set_xlabel("x -->").
        set_ylabel("y -->").
        plot(range(0, 3u), range(0, 3u), [LineType(Solid)]).
        plot(range(0, 3u), range(0, 3u).rev(), [LineType(SmallDot)]).
        echo(&mut stdio::stdout()).
        draw();
}
