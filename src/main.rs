#[macro_use]
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

    let regex = regex::Regex::new(r"(\d+),(\d+):(\d+),(\d+)")?;
    let captures = regex.captures(&options.range).unwrap();

    let start_line = captures[1].parse::<usize>()? - 1;
    let start_col = captures[2].parse::<usize>()? - 1;

    let end_line = captures[3].parse::<usize>()? - 1;
    let end_col = captures[4].parse::<usize>()? - 1;

    let range_content = extract_range(start_line, start_col, end_line, end_col, &contents)?;

    let new_content = if &options.name == "extract_variable" {
        let expression = range_content;
        let variable_declaration = format!("let {} = {};", &options.new_var, expression);

        let new_content = contents.replace(&expression, &options.new_var);
        let mut lines = new_content.split("\n").collect::<Vec<_>>();
        lines.insert(start_line, &variable_declaration);
        lines.join("\n")
    } else if &options.name == "extract_function" {
        let function_call = format!("{}();", &options.new_var);
        let function_declaration = format!("fn {}() {{\n{}\n}}", &options.new_var, range_content);

        let new_content = contents.replace(range_content, &function_call);
        let mut lines = new_content.split("\n").collect::<Vec<_>>();
        lines.insert(start_line, &function_declaration);
        lines.join("\n")
    } else {
        bail!("Unrecognized refactoring {:?}", &options.name);
    };

    File::create(&options.filename)?.write_all(new_content.as_bytes())?;

    Ok(())
}

fn extract_range(
    start_line: usize,
    start_col: usize,
    end_line: usize,
    end_col: usize,
    text: &str,
) -> Result<&str, Error> {
    fn line_index(line: usize, text: &str) -> Result<usize, Error> {
        if line == 0 {
            Ok(0)
        } else {
            let nth_newline = text.match_indices("\n").skip(line - 1).next();
            Ok(nth_newline
                .ok_or(format_err!("Line {} out of range", line))?
                .0
                + 1)
        }
    }

    fn index_of(line: usize, col: usize, text: &str) -> Result<usize, Error> {
        let probably_index = line_index(line, text)? + col;

        if let Some(index) = line_index(line + 1, text).ok() {
            ensure!(
                index > probably_index,
                "Column {} out of range for line {}",
                col,
                line
            );
        }
        ensure!(
            probably_index < text.len(),
            "Location {},{} out of file range",
            line,
            col
        );

        Ok(probably_index)
    }

    let start_index = index_of(start_line, start_col, text)?;
    let last_index = index_of(end_line, end_col, text)?;
    Ok(&text[start_index..=last_index])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    mod extract_range {
        use super::*;

        const TEXT: &'static str = "foo\nbar\nbaz";

        #[test]
        fn first_line() {
            assert_eq!(extract_range(0, 0, 0, 0, TEXT).unwrap(), "f");
            assert_eq!(extract_range(0, 0, 0, 2, TEXT).unwrap(), "foo");
        }

        #[test]
        fn multi_line_start_of_line() {
            assert_eq!(extract_range(0, 0, 1, 0, TEXT).unwrap(), "foo\nb");
        }

        #[test]
        fn second_line_start_of_line() {
            assert_eq!(extract_range(1, 0, 2, 0, TEXT).unwrap(), "bar\nb");
        }

        #[test]
        fn second_line_sub_word() {
            assert_eq!(extract_range(1, 0, 1, 2, TEXT).unwrap(), "bar");
        }

        #[test]
        fn cross_lines_and_words() {
            assert_eq!(extract_range(0, 1, 1, 1, TEXT).unwrap(), "oo\nba");
        }

        #[test]
        fn line_out_of_range() {
            assert!(extract_range(4, 0, 4, 0, TEXT).is_err());
        }

        #[test]
        fn col_out_of_range() {
            assert!(extract_range(0, 0, 0, 4, TEXT).is_err());
        }

        #[test]
        fn last_line_col_out_of_range() {
            assert!(extract_range(2, 0, 2, 4, TEXT).is_err());
        }
    }
}
