extern crate crates_io_api;
extern crate failure;
extern crate regex;
#[macro_use]
extern crate shell;
extern crate walkdir;

use failure::Error;
use std::io::Read;
use std::iter::DoubleEndedIterator;
use walkdir::{DirEntry, WalkDir};

fn main() -> Result<(), Error> {
    cmd!("mkdir -p target/harden").run().unwrap();
    let pwd = cmd!("pwd").stdout_utf8().unwrap();
    let pwd = pwd.trim();
    let harden_dir = format!("{}/target/harden", pwd);
    std::env::set_current_dir(&harden_dir).unwrap();

    let client = crates_io_api::SyncClient::new();
    let summary = client.summary()?;
    let repos = summary
        .most_downloaded
        .iter()
        .filter_map(|k| k.repository.as_ref())
        .filter(|repo| repo.starts_with("https://github.com"))
        .filter(|repo| !repo.ends_with("libc"))
        .take(1);

    for repo in repos {
        clone_repo(repo, &harden_dir)?;
        cmd!("cargo test").run().unwrap();

        for entry in rust_files_pwd() {
            eprintln!("entry: {:#?}", entry);

            let contents = {
                let mut s = String::new();
                std::fs::File::open(entry.path())?.read_to_string(&mut s)?;
                s
            };

            let var_regex = regex::Regex::new(r"let ([^ ]+) = .*;").unwrap();
            for capture in var_regex.captures_iter(&contents) {
                eprintln!("&capture[0]: {:#?}", &capture[0]);
                eprintln!("&capture[1]: {:#?}", &capture[1]);
                unimplemented!();
            }
        }
    }

    Ok(())
}

fn clone_repo(repo: &str, harden_dir: &str) -> Result<(), Error> {
    let new_dir = format!("{}/{}", harden_dir, repo.split('/').next_back().unwrap());
    cmd!("rm -rf {}", &new_dir).run().unwrap();

    cmd!("git clone -q {}", repo).run().unwrap();
    std::env::set_current_dir(format!("{}", new_dir))?;

    Ok(())
}

fn rust_files_pwd() -> impl Iterator<Item = DirEntry> {
    fn matches<P: Fn(&str) -> bool>(e: &DirEntry, pred: P) -> bool {
        let s = e.file_name().to_str().unwrap();
        pred(s)
    }

    WalkDir::new(".")
        .into_iter()
        .filter_entry(|e| !matches(e, |s| s.contains(".git") || s.contains("target")))
        .filter_map(|e| e.ok())
        .filter(|e| e.file_name().to_str().unwrap().ends_with(".rs"))
}
