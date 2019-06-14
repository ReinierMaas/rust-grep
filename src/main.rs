extern crate colored;
extern crate regex;
extern crate structopt;
extern crate walkdir;

use std::fs::File;
use std::io;
use std::io::Read;
use std::path::PathBuf;

use colored::*;
use regex::Regex;
use structopt::StructOpt;
use walkdir::WalkDir;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(parse(from_str))]
    regex: String,
    #[structopt(parse(from_os_str))]
    file: Option<PathBuf>,
}

fn main() -> io::Result<()> {
    let opt = Opt::from_args();
    let regex = Regex::new(&opt.regex).expect("Could not parse regex!");
    let mut file_content = String::new();
    if let Some(path) = opt.file {
        WalkDir::new(path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|dir_entry| dir_entry.path().is_file())
            .for_each(|dir_entry| {
                file_content.truncate(0);
                if let Ok(..) = File::open(dir_entry.path())
                    .expect("Could not open file!")
                    .read_to_string(&mut file_content)
                {
                    find_match(
                        &regex,
                        &format!("{}", dir_entry.path().display()),
                        &file_content,
                    );
                }
            });
    } else {
        let stdin = io::stdin();
        stdin.lock().read_to_string(&mut file_content)?;
        find_match(&regex, "stdin", &file_content);
    }
    Ok(())
}
fn find_match(regex: &Regex, file_path: &str, file_content: &str) {
    for (line_number, line) in file_content.lines().enumerate() {
        if let Some(mat) = regex.find(line) {
            println!(
                "[{}:{}:]{}{}{}",
                file_path,
                line_number + 1,
                &line[..mat.start()],
                &line[mat.start()..mat.end()].red(),
                &line[mat.end()..]
            )
        }
    }
}

//2014-01-01
