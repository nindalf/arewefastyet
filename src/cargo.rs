use crate::repo::Repo;
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;

use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Hash, Serialize, Deserialize, Eq, PartialEq)]
pub(crate) enum Mode {
    Check,
    CheckIncremental,
    Debug,
    DebugIncremental,
    Release,
    ReleaseIncremental,
}

pub(crate) fn benchmark(repo: &Repo, times: u32) -> HashMap<Mode, Vec<String>> {
    cargo_check(repo); // download dependencies
    let mut results = HashMap::new();

    let (base, incremental) = repeat(cargo_check, repo, times);
    results.insert(Mode::Check, base);
    results.insert(Mode::CheckIncremental, incremental);

    let (base, incremental) = repeat(cargo_debug, repo, times);
    results.insert(Mode::Debug, base);
    results.insert(Mode::DebugIncremental, incremental);

    let (base, incremental) = repeat(cargo_release, repo, times);
    results.insert(Mode::Release, base);
    results.insert(Mode::ReleaseIncremental, incremental);

    results
}

fn repeat(f: fn(&Repo) -> String, repo: &Repo, times: u32) -> (Vec<String>, Vec<String>) {
    let mut result_base = Vec::new();
    let mut result_incremental = Vec::new();
    for _ in 0..times {
        repo.remove_target_folder();
        result_base.push(f(repo));
        repo.touch_src();
        result_incremental.push(f(repo));
    }
    (result_base, result_incremental)
}

fn cargo(dir: &PathBuf, args: &[&str]) -> String {
    let output = Command::new("cargo")
        .current_dir(dir)
        .args(args)
        .output()
        .expect("failed to execute cargo");
    let stderr = std::str::from_utf8(&output.stderr).expect("failed to decode output");
    parse_run_time(stderr)
}

fn cargo_check(repo: &Repo) -> String {
    println!("{} - Running cargo check", &repo.name);
    let dir = repo.get_base_directory();
    cargo(&dir, &["check"])
}

fn cargo_debug(repo: &Repo) -> String {
    println!("{} - Running cargo build", &repo.name);
    let dir = repo.get_base_directory();
    cargo(&dir, &["build"])
}

fn cargo_release(repo: &Repo) -> String {
    println!("{} - Running cargo release", &repo.name);
    let dir = repo.get_base_directory();
    cargo(&dir, &["build", "--release"])
}

fn parse_run_time(stderr: &str) -> String {
    let line = stderr.lines().last().unwrap();
    line.split("in ").last().unwrap().to_string()
}
