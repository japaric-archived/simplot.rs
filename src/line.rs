use std::io::MemWriter;

pub struct Line {
    pub args: MemWriter,
    pub data: MemWriter,
}

impl Line {
    pub fn new() -> Line {
        Line {
            args: MemWriter::new(),
            data: MemWriter::new(),
        }
    }
}
