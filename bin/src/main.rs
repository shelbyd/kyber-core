#[macro_use]
extern crate failure;
#[macro_use]
extern crate structopt;

use failure::Error;
use std::str::FromStr;
use structopt::StructOpt;

mod plugin;
mod plugin_locator;

use plugin::PluginAction;

#[derive(StructOpt, Debug)]
#[structopt(name = "kyber")]
enum Arguments {
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
struct Options {
    filename: String,
    line_col: LineCol,
}

#[derive(Debug)]
pub struct LineCol {
    line: usize,
    column: usize,
}

impl LineCol {
    fn to_string(&self) -> String {
        format!("{}:{}", self.line, self.column)
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

fn main() {
    match run() {
        Ok(()) => {}
        Err(e) => {
            println!("{}", e);
            std::process::exit(1)
        }
    }
}

fn run() -> Result<(), Error> {
    let arguments = Arguments::from_args();
    match arguments {
        Arguments::Options(options) => {
            let actions = get_actions(&options)?;
            for action in actions {
                println!("{}", action);
            }
        }
        Arguments::Do { .. } => {}
    }

    Ok(())
}

fn get_actions(options: &Options) -> Result<Vec<PluginAction>, Error> {
    Ok(plugin_locator::path_exes()?
        .into_iter()
        .flat_map(|plugin| plugin.actions(&options.filename, &options.line_col))
        .collect::<Vec<_>>())
}
