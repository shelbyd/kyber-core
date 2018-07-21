use failure::Error;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub struct PluginAction {
    plugin_name: String,
    action_name: String,
}

impl PluginAction {
    pub fn parse(plugin: String, action: String) -> Result<PluginAction, Error> {
        ensure!(
            action != "" && !action.contains(char::is_whitespace)
                && !action.contains(char::is_control),
            "Invalid action: {:?}",
            action
        );

        Ok(PluginAction {
            plugin_name: plugin,
            action_name: action,
        })
    }
}

impl FromStr for PluginAction {
    type Err = Error;

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

impl fmt::Display for PluginAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.plugin_name, self.action_name)
    }
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

    #[test]
    fn plugin_action_parse() {
        assert_eq!(
            PluginAction::parse("foo".to_string(), "bar".to_string()).unwrap(),
            plugin_action("foo", "bar")
        );
        assert!(PluginAction::parse("foo".to_string(), "".to_string()).is_err());
        assert!(PluginAction::parse("foo".to_string(), "two words".to_string()).is_err());
        assert!(PluginAction::parse("foo".to_string(), "\u{009c}".to_string()).is_err());
    }
}
