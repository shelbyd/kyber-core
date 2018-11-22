#[macro_use]
extern crate failure;
extern crate regex;
extern crate structopt;

mod file_location;
use self::file_location::{get, parse_range};

use failure::Error;
use std::fs::File;
use std::io::{Read, Write};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Options {
    _should_do: String,
    name: String,
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

    let (start, end) = parse_range(&options.range)?;
    let range_content = get(&contents, start..=end)?;

    let new_content = if &options.name == "extract_variable" {
        let expression = range_content;
        let variable_declaration = format!("let {} = {};", &options.new_var, expression);

        let new_content = contents.replace(&expression, &options.new_var);
        let mut lines = new_content.split("\n").collect::<Vec<_>>();
        lines.insert(start.line, &variable_declaration);
        lines.join("\n")
    } else if &options.name == "extract_function" {
        let function_call = format!("{}();", &options.new_var);
        let function_declaration = format!("fn {}() {{\n{}\n}}", &options.new_var, range_content);

        let new_content = contents.replace(range_content, &function_call);
        let mut lines = new_content.split("\n").collect::<Vec<_>>();
        lines.insert(start.line, &function_declaration);
        lines.join("\n")
    } else if &options.name == "rename_variable" {
        let old_name = range_content;
        let new_name = &options.new_var;
        contents.replace(old_name, new_name)
    } else {
        bail!("Unrecognized refactoring {:?}", &options.name);
    };

    File::create(&options.filename)?.write_all(new_content.as_bytes())?;

    Ok(())
}
