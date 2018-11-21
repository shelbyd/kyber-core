use failure::Error;
use plugin::{PathExePlugin, Plugin};
use std::env;
use std::fs;

pub fn path_exes() -> Result<Vec<Box<dyn Plugin>>, Error> {
    let path = env::var("PATH")?;
    let path_exes = path
        .split(":")
        .filter_map(|dir| fs::read_dir(dir).ok())
        .flat_map(|x| x)
        .filter_map(|dir_entry| dir_entry.ok())
        .map(|entry| entry.path())
        .filter_map(|path| path.file_name().map(ToOwned::to_owned));

    Ok(path_exes
        .filter(|exe| {
            exe.to_str()
                .map(|exe| exe.starts_with("kyber-plugin-"))
                .unwrap_or(false)
        })
        .map(|path_exe| Box::new(PathExePlugin::new(path_exe)) as Box<Plugin>)
        .collect())
}
