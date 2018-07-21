#[macro_use]
extern crate failure;
#[macro_use]
extern crate structopt;

use std::fmt;
use std::str::FromStr;
use structopt::StructOpt;

mod plugin_action;
pub use self::plugin_action::*;

pub fn main(actions: &[&Action]) {
    let arguments = Arguments::from_args();
    match arguments {
        Arguments::Options(_options) => for action in actions {
            println!("{}", (*action).name());
        },
        Arguments::Do { .. } => {}
    }
}

pub trait Action {
    fn name(&self) -> &'static str;
}

#[derive(StructOpt, Debug)]
#[structopt(name = "kyber")]
pub enum Arguments {
    #[structopt(name = "options")]
    Options(Options),

    #[structopt(name = "do")]
    Do {
        action: PluginAction,
        filename: String,
        line_col: LineCol,
    },
}

#[derive(StructOpt, Debug)]
pub struct Options {
    pub filename: String,
    pub line_col: LineCol,
}

#[derive(Debug)]
pub struct LineCol {
    line: usize,
    column: usize,
}

impl fmt::Display for LineCol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

impl FromStr for LineCol {
    type Err = failure::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(":");

        match (split.next(), split.next(), split.next()) {
            (Some(line), Some(column), None) => Ok(LineCol {
                line: line.parse()?,
                column: column.parse()?,
            }),
            _ => bail!("Invalid line column '{}'", s),
        }
    }
}
