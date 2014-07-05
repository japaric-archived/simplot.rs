extern crate simplot;

use std::io::stdio;

use simplot::Figure;

fn main() {
    Figure::new().
        set_output_file(Path::new("output.png")).
        set_size((1366, 768)).
        plot(range(0, 3u), range(0, 3u)).
        plot(range(0, 3u), range(0, 3u).rev()).
        echo(&mut stdio::stdout()).
        draw();
}
