extern crate failure;
extern crate regex;
extern crate structopt;

use failure::Error;
use std::fs::File;
use std::io::{Read, Write};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Options {
    _should_do: String,
    _name: String,
    filename: String,
    range: String,
    new_var: String,
}

fn main() -> Result<(), Error> {
    let options = Options::from_args();

    let contents = {
        let mut s = String::new();
        File::open(&options.filename)?.read_to_string(&mut s)?;
        s
    };

    let regex = regex::Regex::new(r"(\d+),(\d+):(\d+),(\d+)")?;
    let captures = regex.captures(&options.range).unwrap();

    let start_line = captures[1].parse::<usize>()? - 1;
    let start_col = captures[2].parse::<usize>()? - 1;

    let _end_line = captures[3].parse::<usize>()? - 1;
    let end_col = captures[4].parse::<usize>()? - 1;

    let contents = {
        let mut lines = contents
            .split("\n")
            .map(ToOwned::to_owned)
            .collect::<Vec<_>>();

        let to_replace = {
            let start_line = &lines[start_line];
            start_line[start_col..(end_col + 1)].to_owned()
        };

        let mut lines = lines
            .into_iter()
            .map(|l| l.replace(&to_replace, &options.new_var))
            .collect::<Vec<_>>();

        let new_line = format!("let {} = {};", &options.new_var, to_replace);

        lines.insert(start_line, new_line);
        lines.join("\n")
    };

    File::create(&options.filename)?.write_all(contents.as_bytes())?;

    Ok(())
}
