extern crate failure;
extern crate kyber_support;
extern crate structopt;

use failure::Error;
use structopt::StructOpt;

mod plugin;
mod plugin_locator;

use kyber_support::{Arguments, Options, PluginAction};

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
