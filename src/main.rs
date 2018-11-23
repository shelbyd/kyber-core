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
    #[structopt(short = "f", long = "file")]
    filename: String,
    #[structopt(long = "range")]
    range: String,

    refactor: String,
    new_var: String,
}

#[derive(StructOpt)]
enum Refactor {
    #[structopt(name = "extract_variable")]
    ExtractVariable { new_name: String },
    #[structopt(name = "extract_function")]
    ExtractFunction { new_name: String },
    #[structopt(name = "rename_variable")]
    RenameVariable { new_name: String },
    #[structopt(name = "inline_variable")]
    InlineVariable,
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

    let new_content = if &options.refactor == "extract_variable" {
        let expression = range_content;
        let variable_declaration = format!("let {} = {};", &options.new_var, expression);

        let new_content = contents.replace(&expression, &options.new_var);
        let mut lines = new_content.split("\n").collect::<Vec<_>>();
        lines.insert(start.line, &variable_declaration);
        lines.join("\n")
    } else if &options.refactor == "extract_function" {
        let function_call = format!("{}();", &options.new_var);
        let function_declaration = format!("fn {}() {{\n{}\n}}", &options.new_var, range_content);

        let new_content = contents.replace(range_content, &function_call);
        let mut lines = new_content.split("\n").collect::<Vec<_>>();
        lines.insert(start.line, &function_declaration);
        lines.join("\n")
    } else if &options.refactor == "rename_variable" {
        let old_name = range_content;
        let new_name = &options.new_var;
        contents.replace(old_name, new_name)
    } else if &options.refactor == "inline_variable" {
        let variable_name = range_content;
        let expression_matcher =
            regex::Regex::new(&format!("let {} = (?P<expr>.+);", variable_name))?;
        let expression = &expression_matcher.captures(&contents).unwrap()["expr"];
        let new_content = expression_matcher.replace(&contents, "");
        new_content.replace(variable_name, expression)
    } else {
        bail!("Unrecognized refactoring {:?}", &options.refactor);
    };

    File::create(&options.filename)?.write_all(new_content.as_bytes())?;

    Ok(())
}
