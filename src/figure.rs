use std::io::{Command,File};

use data::Data;
use line::Line;
use option::{PlotOption,LineType};

pub struct Figure {
    lines: Vec<Line>,
    output: Option<Path>,
    size: Option<(uint, uint)>,
    title: Option<String>,
    xlabel: Option<String>,
    ylabel: Option<String>,
}

impl Figure {
    pub fn new() -> Figure {
        Figure {
            lines: Vec::new(),
            output: None,
            size: None,
            title: None,
            xlabel: None,
            ylabel: None,
        }
    }

    pub fn set_output_file<'a>(&'a mut self, path: Path) -> &'a mut Figure {
        self.output = Some(path);

        self
    }

    pub fn set_size<'a>(&'a mut self, size: (uint, uint)) -> &'a mut Figure {
        self.size = Some(size);

        self
    }

    pub fn set_title<'a, T: ToStr>(&'a mut self, title: T) -> &'a mut Figure {
        self.title = Some(title.to_str());

        self
    }

    pub fn set_xlabel<'a, T: ToStr>(&'a mut self, label: T) -> &'a mut Figure {
        self.xlabel = Some(label.to_str());

        self
    }

    pub fn set_ylabel<'a, T: ToStr>(&'a mut self, label: T) -> &'a mut Figure {
        self.ylabel = Some(label.to_str());

        self
    }

    pub fn plot<'a,
                A: Data,
                B: Data,
                X: Iterator<A>,
                Y: Iterator<B>>(
                &'a mut self,
                xs: X,
                ys: Y,
                options: &[PlotOption])
                -> &'a mut Figure {
        self.lines.push(Line::new());

        {
            let l = self.lines.mut_last().unwrap();

            let mut nrecords = 0u;
            for (x, y) in xs.zip(ys) {
                l.data.write_le_f64(x.get());
                l.data.write_le_f64(y.get());
                nrecords += 1;
            }

            write!(l.args, " binary endian=little");
            write!(l.args, " record={}", nrecords);
            write!(l.args, r#" format="%float64""#);
            write!(l.args, " using 1:2");
            write!(l.args, " with lines");

            for option in options.iter() {
                match *option {
                    LineType(lt) => {
                        write!(l.args, " lt {}", lt);
                    },
                }
            }

            write!(l.args, ",");
        }

        self
    }

    pub fn echo<'a, W: Writer>(&'a mut self, dst: &mut W) -> &'a mut Figure {
        match self.output {
            Some(ref output) => {
                writeln!(dst, "set output \"{}\"", output.display());
            },
            None => fail!("No output file specified"),
        }

        match self.size {
            Some((width, height)) => {
                writeln!(dst,
                         "set terminal pngcairo dashed size {}, {}",
                         width,
                         height);
            },
            None => {
                writeln!(dst, "set terminal pngcairo dashed");
            }
        }

        match self.title {
            Some(ref title) => {
                writeln!(dst, r#"set title "{}""#, title);
            },
            None => {},
        }

        match self.xlabel {
            Some(ref label) => {
                writeln!(dst, r#"set xlabel "{}""#, label);
            },
            None => {},
        }

        match self.ylabel {
            Some(ref label) => {
                writeln!(dst, r#"set ylabel "{}""#, label);
            },
            None => {},
        }

        if self.lines.len() == 0 {
            fail!("Nothing to plot!");
        }

        // XXX Disables legend
        writeln!(dst, "set nokey");

        write!(dst, "plot");
        for line in self.lines.iter() {
            write!(dst, " '-'");
            dst.write(line.args.get_ref());
        }

        write!(dst, "\n");
        for line in self.lines.iter() {
            dst.write(line.data.get_ref());
        }

        self
    }

    pub fn save_script<'a>(&'a mut self, path: &Path) -> &'a mut Figure {
        match File::create(path) {
            Err(e) => fail!("Couldn't create {}: {}", path.display(), e),
            Ok(mut f) => self.echo(&mut f),
        }
    }

    pub fn draw<'a>(&'a mut self) -> &'a mut Figure {
        let mut p = match Command::new("gnuplot").spawn() {
            Err(e) => fail!("`gnuplot`: {}", e),
            Ok(p) => p,
        };

        self.echo(p.stdin.as_mut().unwrap())
    }
}
