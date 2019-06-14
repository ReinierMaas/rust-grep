use std::fs::File;
use std::io;
use std::io::Read;
use std::path::PathBuf;

use colored::*;
use rayon::prelude::*;
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
    if let Some(path) = opt.file {
        let files = WalkDir::new(path).into_iter().collect::<Vec<_>>();
        let matches = files
            .into_par_iter()
            .filter_map(|e| e.ok())
            .filter(|dir_entry| dir_entry.path().is_file())
            .map(|dir_entry| {
                File::open(dir_entry.path()).map(|mut file| {
                    read_to_string(&mut file).map(|file_content| (dir_entry, file_content))
                })
            })
            .filter_map(|e| e.ok())
            .filter_map(|e| e.ok())
            .map(|(dir_entry, file_content)| {
                find_match(
                    &regex,
                    &format!("{}", dir_entry.path().display()),
                    &file_content,
                )
            })
            .collect::<String>();
        println!("{}", matches);
    } else {
        let mut file_content = String::new();
        let stdin = io::stdin();
        stdin.lock().read_to_string(&mut file_content)?;
        find_match(&regex, "stdin", &file_content);
    }
    Ok(())
}
fn read_to_string(file: &mut File) -> io::Result<String> {
    let mut file_content = String::new();
    match file.read_to_string(&mut file_content) {
        Ok(..) => Ok(file_content),
        Err(err) => Err(err),
    }
}
fn find_match(regex: &Regex, file_path: &str, file_content: &str) -> String {
    let mut matches = String::new();
    for (line_number, line) in file_content.lines().enumerate() {
        if let Some(mat) = regex.find(line) {
            matches.push_str(&format!(
                "[{}:{}:]{}{}{}\n",
                file_path,
                line_number + 1,
                &line[..mat.start()],
                &line[mat.start()..mat.end()].red(),
                &line[mat.end()..]
            ))
        }
    }
    matches
}

//2014-01-01
