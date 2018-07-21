use kyber_support::{LineCol, PluginAction};
use std::ffi::OsString;
use std::process::Command;

pub trait Plugin {
    fn action_names(&self, filename: &str, line_col: &LineCol) -> Vec<String>;
    fn plugin_name(&self) -> String;

    fn actions(&self, filename: &str, line_col: &LineCol) -> Vec<PluginAction> {
        self.action_names(filename, line_col)
            .into_iter()
            .filter_map(|action| PluginAction::parse(self.plugin_name(), action).ok())
            .collect()
    }
}

pub struct PathExePlugin {
    exe: OsString,
}

impl PathExePlugin {
    pub fn new(exe: OsString) -> PathExePlugin {
        PathExePlugin { exe }
    }
}

impl Plugin for PathExePlugin {
    fn plugin_name(&self) -> String {
        self.exe.clone().to_string_lossy().into_owned()
    }

    fn action_names(&self, filename: &str, line_col: &LineCol) -> Vec<String> {
        Command::new(&self.exe)
            .args(&["options", filename, &line_col.to_string()])
            .output()
            .into_iter()
            .flat_map(|output| String::from_utf8(output.stdout))
            .flat_map(|output| {
                output
                    .split("\n")
                    .map(ToOwned::to_owned)
                    .collect::<Vec<_>>()
            })
            .collect()
    }
}
