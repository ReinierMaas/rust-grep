#[macro_use]
extern crate structopt;
extern crate regex;
extern crate termion;

use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::path::PathBuf;

use structopt::StructOpt;
use regex::Regex;
use termion::{color, style};

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(parse(from_str))]
    regex: String,
    #[structopt(parse(from_os_str))]
    file: Option<PathBuf>,
}

fn main() {
    let opt = Opt::from_args();
    let regex = Regex::new(&opt.regex).unwrap();
    let lines: Vec<_>;
    if let &Some(ref path) = &opt.file {
        lines = BufReader::new(File::open(path).unwrap())
            .lines().flat_map(|x|x).collect();
    } else {
        let stdin = io::stdin();
        lines = stdin.lock().lines().flat_map(|x| x).collect();
    };
    for ref line in lines {
        if let Some(mat) = regex.find(line) {
            println!("{}{}{}{}{}"
                , &line[..mat.start()]
                , color::Fg(color::Red)
                , &line[mat.start()..mat.end()]
                , style::Reset
                , &line[mat.end()..])
        }
    }
}

//2014-01-01
