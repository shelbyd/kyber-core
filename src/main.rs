#[macro_use]
extern crate failure;
extern crate regex;
extern crate structopt;

mod containing_scope;
mod file_location;
mod replace_range;

use self::containing_scope::*;
use self::file_location::{get, parse_range};
use self::replace_range::*;

use failure::Error;
use std::fs::File;
use std::io::{Read, Write};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Options {
    #[structopt(short = "f", long = "file")]
    filename: String,
    #[structopt(long = "range")]
    range: String,

    #[structopt(subcommand)]
    refactor: Refactor,
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

    let new_content = match &options.refactor {
        Refactor::ExtractVariable { new_name } => {
            let expression = range_content;
            let variable_declaration = format!("let {} = {};", new_name, expression);

            let new_content = contents.replace(&expression, new_name);
            let mut lines = new_content.split("\n").collect::<Vec<_>>();
            lines.insert(start.line, &variable_declaration);
            lines.join("\n")
        }
        Refactor::ExtractFunction { new_name } => {
            let function_call = format!("{}();", new_name);
            let function_declaration = format!("fn {}() {{\n{}\n}}", new_name, range_content);

            let new_content = contents.replace(range_content, &function_call);
            let mut lines = new_content.split("\n").collect::<Vec<_>>();
            lines.insert(start.line, &function_declaration);
            lines.join("\n")
        }
        Refactor::RenameVariable { new_name } => {
            let start_index = start.index(&contents)?;
            let end_index = end.index(&contents)?;
            let input_range = start_index..(end_index + 1);
            contents
                .replace_range(containing_scope(&contents, input_range), |s| {
                    s.replace(range_content, new_name)
                })
                .unwrap()
        }
        Refactor::InlineVariable => {
            let variable_name = range_content;
            let expression_matcher =
                regex::Regex::new(&format!("let {} = (?P<expr>.+);", variable_name))?;
            let expression = &expression_matcher.captures(&contents).unwrap()["expr"];
            let new_content = expression_matcher.replace(&contents, "");
            new_content.replace(variable_name, expression)
        }
    };

    File::create(&options.filename)?.write_all(new_content.as_bytes())?;

    Ok(())
}
