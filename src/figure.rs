use std::io::process::{Command,ProcessOutput};
use std::io::File;

use data::Data;
use line::Line;
use option::{
    LineType,
    PlotOption,
    PointType,
    Title,
};
use plottype::PlotType;
use terminal::{Png,Terminal};

pub struct Figure {
    font: Option<String>,
    lines: Vec<Line>,
    logscale: Option<(bool, bool)>,
    output: Option<Path>,
    size: Option<(uint, uint)>,
    terminal: Option<Terminal>,
    title: Option<String>,
    xlabel: Option<String>,
    xrange: Option<(f64, f64)>,
    xtics: Option<Vec<(String, String)>>,
    ylabel: Option<String>,
    yrange: Option<(f64, f64)>,
    ytics: Option<Vec<(String, String)>>,
}

impl Figure {
    pub fn new() -> Figure {
        Figure {
            font: None,
            lines: Vec::new(),
            logscale: None,
            output: None,
            size: None,
            terminal: None,
            title: None,
            xlabel: None,
            xrange: None,
            xtics: None,
            ylabel: None,
            yrange: None,
            ytics: None,
        }
    }

    pub fn set_font<S: ToString>(&mut self, font: S) -> &mut Figure {
        self.font = Some(font.to_string());

        self
    }

    pub fn set_logscale(&mut self, axes: (bool, bool)) -> &mut Figure {
        self.logscale = Some(axes);

        self
    }

    pub fn set_output_file(&mut self, path: Path) -> &mut Figure {
        self.output = Some(path);

        self
    }

    pub fn set_size(&mut self, size: (uint, uint)) -> &mut Figure {
        self.size = Some(size);

        self
    }

    pub fn set_terminal(&mut self, terminal: Terminal) -> &mut Figure {
        self.terminal = Some(terminal);

        self
    }

    pub fn set_title<T: ToString>(
                     &mut self,
                     title: T)
                     -> &mut Figure {
        self.title = Some(title.to_string());

        self
    }

    pub fn set_xlabel<T: ToString>(
                      &mut self,
                      label: T)
                      -> &mut Figure {
        self.xlabel = Some(label.to_string());

        self
    }

    pub fn set_xrange<A: Data>(
                      &mut self,
                      range: (A, A))
                      -> &mut Figure {
        let (low, high) = range;
        self.xrange = Some((low.get(), high.get()));

        self
    }

    pub fn set_xtics<A: Data,
                     S: ToString,
                     L: Iterator<S>,
                     P: Iterator<A>>(
                     &mut self,
                     labels: L,
                     positions: P)
                     -> &mut Figure {
        self.xtics = Some(labels.zip(positions).map(|(l, p)| {
            (l.to_string(), format!("{}", p.get()))
        }).collect());

        self
    }

    pub fn set_ylabel<T: ToString>(
                      &mut self,
                      label: T)
                      -> &mut Figure {
        self.ylabel = Some(label.to_string());

        self
    }

    pub fn set_yrange<A: Data>(
                      &mut self,
                      range: (A, A))
                      -> &mut Figure {
        let (low, high) = range;
        self.yrange = Some((low.get(), high.get()));

        self
    }

    pub fn set_ytics<A: Data,
                     S: ToString,
                     L: Iterator<S>,
                     P: Iterator<A>>(
                     &mut self,
                     labels: L,
                     positions: P)
                     -> &mut Figure {
        self.ytics = Some(labels.zip(positions).map(|(l, p)| {
            (l.to_string(), format!("{}", p.get()))
        }).collect());

        self
    }

    pub fn plot<A: Data,
                B: Data,
                X: Iterator<A>,
                Y: Iterator<B>>(
                &mut self,
                plot_type: PlotType,
                xs: X,
                ys: Y,
                options: &[PlotOption])
                -> &mut Figure {
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
            write!(l.args, " with {}", plot_type);

            for option in options.iter() {
                match *option {
                    LineType(lt) => {
                        write!(l.args, " lt {}", lt);
                    },
                    PointType(pt) => {
                        write!(l.args, " pt {}", pt);
                    },
                    Title(s) => {
                        write!(l.args, r#" title "{}""#, s);
                    },
                }
            }

            if !options.iter().any(|option| match *option {
                Title(_) => true,
                _ => false,
            }) {
                write!(l.args, " notitle");
            }

            write!(l.args, ",");
        }

        self
    }

    // TODO DRY: This method is similar to the `plot` method
    pub fn xerrorbars<A: Data,
                      B: Data,
                      C: Data,
                      D: Data,
                      X: Iterator<A>,
                      Y: Iterator<B>,
                      YL: Iterator<C>,
                      YH: Iterator<D>>(
                      &mut self,
                      xs: X,
                      ys: Y,
                      ylows: YL,
                      yhighs: YH,
                      options: &[PlotOption])
                      -> &mut Figure {
        self.lines.push(Line::new());

        {
            let l = self.lines.mut_last().unwrap();

            let mut nrecords = 0u;
            for (((x, y), ylow), yhigh) in xs.zip(ys).zip(ylows).zip(yhighs) {
                l.data.write_le_f64(x.get());
                l.data.write_le_f64(y.get());
                l.data.write_le_f64(ylow.get());
                l.data.write_le_f64(yhigh.get());
                nrecords += 1;
            }

            write!(l.args, " binary endian=little");
            write!(l.args, " record={}", nrecords);
            write!(l.args, r#" format="%float64""#);
            write!(l.args, " using 1:2:3:4");
            write!(l.args, " with xerrorbars");

            for option in options.iter() {
                match *option {
                    LineType(lt) => {
                        write!(l.args, " lt {}", lt);
                    },
                    PointType(pt) => {
                        write!(l.args, " pt {}", pt);
                    },
                    Title(s) => {
                        write!(l.args, r#" title "{}""#, s);
                    },
                }
            }

            if !options.iter().any(|option| match *option {
                Title(_) => true,
                _ => false,
            }) {
                write!(l.args, " notitle");
            }

            write!(l.args, ",");
        }

        self
    }

    // TODO DRY: This method is similar to the `plot` method
    pub fn yerrorbars<A: Data,
                      B: Data,
                      C: Data,
                      D: Data,
                      X: Iterator<A>,
                      Y: Iterator<B>,
                      YL: Iterator<C>,
                      YH: Iterator<D>>(
                      &mut self,
                      xs: X,
                      ys: Y,
                      ylows: YL,
                      yhighs: YH,
                      options: &[PlotOption])
                      -> &mut Figure {
        self.lines.push(Line::new());

        {
            let l = self.lines.mut_last().unwrap();

            let mut nrecords = 0u;
            for (((x, y), ylow), yhigh) in xs.zip(ys).zip(ylows).zip(yhighs) {
                l.data.write_le_f64(x.get());
                l.data.write_le_f64(y.get());
                l.data.write_le_f64(ylow.get());
                l.data.write_le_f64(yhigh.get());
                nrecords += 1;
            }

            write!(l.args, " binary endian=little");
            write!(l.args, " record={}", nrecords);
            write!(l.args, r#" format="%float64""#);
            write!(l.args, " using 1:2:3:4");
            write!(l.args, " with yerrorbars");

            for option in options.iter() {
                match *option {
                    LineType(lt) => {
                        write!(l.args, " lt {}", lt);
                    },
                    PointType(pt) => {
                        write!(l.args, " pt {}", pt);
                    },
                    Title(s) => {
                        write!(l.args, r#" title "{}""#, s);
                    },
                }
            }

            if !options.iter().any(|option| match *option {
                Title(_) => true,
                _ => false,
            }) {
                write!(l.args, " notitle");
            }

            write!(l.args, ",");
        }

        self
    }

    pub fn echo<W: Writer>(&mut self, dst: &mut W) -> &mut Figure {
        match self.logscale {
            Some((true, true)) => {
                writeln!(dst, "set logscale xy");
            },
            Some((true, false)) => {
                writeln!(dst, "set logscale x");
            },
            Some((false, true)) => {
                writeln!(dst, "set logscale y");
            },
            _ => {},
        }

        match self.output {
            Some(ref output) => {
                writeln!(dst, "set output \"{}\"", output.display());
            },
            None => fail!("No output file specified"),
        }

        if self.terminal.is_none() {
            self.terminal = Some(Png);
        }

        match self.terminal {
            Some(ref terminal) => {
                write!(dst, "set terminal {}", terminal);
            },
            None => {},
        }

        match self.size {
            Some((width, height)) => {
                write!(dst, " dashed size {}, {}", width, height);
            },
            None => {},
        }
        match self.font {
            Some(ref font) => {
                write!(dst, " font '{}'", font);
            },
            None => {},
        }
        writeln!(dst, "");

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

        match self.xrange {
            Some((low, high)) => {
                if low < high {
                    writeln!(dst, "set xrange [{}:{}]", low, high);
                } else {
                    writeln!(dst, "set xrange [{}:{}] reverse", high, low);
                }
            },
            None => {}
        }

        match self.xtics {
            Some(ref tics) => {
                writeln!(dst, "set xtics ({})", tics.iter().map(|&(ref l, ref p)| {
                    format!(r#""{}" {}"#, l, p)
                }).collect::<Vec<String>>().connect(", "));
            },
            None => {}
        }

        match self.ylabel {
            Some(ref label) => {
                writeln!(dst, r#"set ylabel "{}""#, label);
            },
            None => {},
        }

        match self.yrange {
            Some((low, high)) => {
                if low < high {
                    writeln!(dst, "set yrange [{}:{}]", low, high);
                } else {
                    writeln!(dst, "set yrange [{}:{}] reverse", high, low);
                }
            },
            None => {}
        }

        match self.ytics {
            Some(ref tics) => {
                writeln!(dst, "set ytics ({})", tics.iter().map(|&(ref l, ref p)| {
                    format!(r#""{}" {}"#, l, p)
                }).collect::<Vec<String>>().connect(", "));
            },
            None => {}
        }

        if self.lines.len() == 0 {
            fail!("Nothing to plot!");
        }

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

    pub fn save_script(&mut self, path: &Path) -> &mut Figure {
        match File::create(path) {
            Err(e) => fail!("Couldn't create {}: {}", path.display(), e),
            Ok(mut f) => self.echo(&mut f),
        }
    }

    pub fn draw(&mut self) -> &mut Figure {
        let mut p = match Command::new("gnuplot").spawn() {
            Err(e) => fail!("`gnuplot`: {}", e),
            Ok(p) => p,
        };

        self.echo(p.stdin.as_mut().unwrap());

        match p.wait_with_output() {
            Err(e) => fail!("{}", e),
            Ok(ProcessOutput { error: err, status: exit, .. }) => if !exit.success() {
                print!("{}", String::from_utf8_lossy(err.as_slice()));

                fail!("runtime error");
            }
        }

        self
    }
}
