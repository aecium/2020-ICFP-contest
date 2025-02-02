use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use stacker;

mod galaxy_parse;
use galaxy_parse::*;
mod galaxy_eval;
use galaxy_eval::*;

fn read_file_lines(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("No such file");
    let buf = io::BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not load line"))
        .collect()
}
pub struct Galaxy {
    pub data : HashMap<String, Ops>
}

impl Galaxy {
    pub fn import(&mut self, lines : Vec<String>) {
        for line in lines {
            let result = parse_line(&line);
            match result {
                Ok((_, (name, op))) => {
                    self.data.insert(name.to_string(),op)
                },
                err => panic!("couldn't parse this line: \n \n {} \n \n Parser Err: \n \n {:?}", line, err)
            };
        }
    }

    pub fn new() -> Galaxy {
        Galaxy {
            data : HashMap::new(),
        }
    }
}

fn main() {
    let mut galaxy = Galaxy::new();
    stacker::grow(1024 * 1024 * 100, || {
        galaxy.import(read_file_lines("galaxy.txt"));
    });
    println!("{:?}", galaxy.data.get("galaxy"));
    eval("galaxy", &mut galaxy);
    println!("{:?}", galaxy.data.get("galaxy"));
}