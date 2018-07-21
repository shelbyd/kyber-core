#[macro_use]
extern crate failure;
#[macro_use]
extern crate structopt;

use std::str::FromStr;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "kyber")]
enum Arguments {
    #[structopt(name = "options")]
    Options {
        filename: String,
        location: Location,
    },

    #[structopt(name = "do")]
    Do {
        action: PluginAction,
        filename: String,
        location: Location,
    },
}

#[derive(Debug)]
struct Location {
    line: usize,
    column: usize,
}

impl FromStr for Location {
    type Err = failure::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(":");

        match (split.next(), split.next(), split.next()) {
            (Some(line), Some(column), None) => Ok(Location {
                line: line.parse()?,
                column: column.parse()?,
            }),
            _ => bail!("Invalid line column '{}'", s),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct PluginAction {
    plugin_name: String,
    action_name: String,
}

impl FromStr for PluginAction {
    type Err = failure::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split("/");

        match (split.next(), split.next(), split.next()) {
            (Some(plugin), Some(action), None) => Ok(PluginAction {
                plugin_name: plugin.to_string(),
                action_name: action.to_string(),
            }),
            _ => bail!("Invalid plugin action '{}'", s),
        }
    }
}

fn main() {
    let arguments = Arguments::from_args();
    println!("{:?}", arguments);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn plugin_action(p: &str, a: &str) -> PluginAction {
        PluginAction {
            plugin_name: String::from(p),
            action_name: String::from(a),
        }
    }

    #[test]
    fn action_from_str() {
        assert_eq!(
            PluginAction::from_str("foo/bar").unwrap(),
            plugin_action("foo", "bar")
        );
        assert_eq!(
            PluginAction::from_str("foo-bar/baz").unwrap(),
            plugin_action("foo-bar", "baz")
        );
        assert_eq!(
            PluginAction::from_str("foo/baz-qux").unwrap(),
            plugin_action("foo", "baz-qux")
        );
        assert!(PluginAction::from_str("foo/bar/baz").is_err());
        assert!(PluginAction::from_str("foo").is_err());
    }
}
