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

pub struct Figure {
    lines: Vec<Line>,
    logscale: Option<(bool, bool)>,
    output: Option<Path>,
    size: Option<(uint, uint)>,
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
            lines: Vec::new(),
            logscale: None,
            output: None,
            size: None,
            title: None,
            xlabel: None,
            xrange: None,
            xtics: None,
            ylabel: None,
            yrange: None,
            ytics: None,
        }
    }

    pub fn set_logscale<'a>(&'a mut self, axes: (bool, bool)) -> &'a mut Figure {
        self.logscale = Some(axes);

        self
    }

    pub fn set_output_file<'a>(&'a mut self, path: Path) -> &'a mut Figure {
        self.output = Some(path);

        self
    }

    pub fn set_size<'a>(&'a mut self, size: (uint, uint)) -> &'a mut Figure {
        self.size = Some(size);

        self
    }

    pub fn set_title<'a,
                     T: ToString>(
                     &'a mut self,
                     title: T)
                     -> &'a mut Figure {
        self.title = Some(title.to_string());

        self
    }

    pub fn set_xlabel<'a,
                      T: ToString>(
                      &'a mut self,
                      label: T)
                      -> &'a mut Figure {
        self.xlabel = Some(label.to_string());

        self
    }

    pub fn set_xrange<'a,
                      A: Data>(
                      &'a mut self,
                      range: (A, A))
                      -> &'a mut Figure {
        let (low, high) = range;
        self.xrange = Some((low.get(), high.get()));

        self
    }

    pub fn set_xtics<'a,
                     A: Data,
                     S: ToString,
                     L: Iterator<S>,
                     P: Iterator<A>>(
                     &'a mut self,
                     labels: L,
                     positions: P)
                     -> &'a mut Figure {
        self.xtics = Some(labels.zip(positions).map(|(l, p)| {
            (l.to_string(), format!("{}", p.get()))
        }).collect());

        self
    }

    pub fn set_ylabel<'a,
                      T: ToString>(
                      &'a mut self,
                      label: T)
                      -> &'a mut Figure {
        self.ylabel = Some(label.to_string());

        self
    }

    pub fn set_yrange<'a,
                      A: Data>(
                      &'a mut self,
                      range: (A, A))
                      -> &'a mut Figure {
        let (low, high) = range;
        self.yrange = Some((low.get(), high.get()));

        self
    }

    pub fn set_ytics<'a,
                     A: Data,
                     S: ToString,
                     L: Iterator<S>,
                     P: Iterator<A>>(
                     &'a mut self,
                     labels: L,
                     positions: P)
                     -> &'a mut Figure {
        self.ytics = Some(labels.zip(positions).map(|(l, p)| {
            (l.to_string(), format!("{}", p.get()))
        }).collect());

        self
    }

    pub fn plot<'a,
                A: Data,
                B: Data,
                X: Iterator<A>,
                Y: Iterator<B>>(
                &'a mut self,
                plot_type: PlotType,
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
    pub fn xerrorbars<'a,
                      A: Data,
                      B: Data,
                      C: Data,
                      D: Data,
                      X: Iterator<A>,
                      Y: Iterator<B>,
                      YL: Iterator<C>,
                      YH: Iterator<D>>(
                      &'a mut self,
                      xs: X,
                      ys: Y,
                      ylows: YL,
                      yhighs: YH,
                      options: &[PlotOption])
                      -> &'a mut Figure {
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
    pub fn yerrorbars<'a,
                      A: Data,
                      B: Data,
                      C: Data,
                      D: Data,
                      X: Iterator<A>,
                      Y: Iterator<B>,
                      YL: Iterator<C>,
                      YH: Iterator<D>>(
                      &'a mut self,
                      xs: X,
                      ys: Y,
                      ylows: YL,
                      yhighs: YH,
                      options: &[PlotOption])
                      -> &'a mut Figure {
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

    pub fn echo<'a, W: Writer>(&'a mut self, dst: &mut W) -> &'a mut Figure {
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
