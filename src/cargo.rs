use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;

use anyhow::{anyhow, Context, Result};

use crate::repo::Repo;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub(crate) enum Mode {
    Check,
    CheckIncremental,
    Debug,
    DebugIncremental,
    Release,
    ReleaseIncremental,
}

pub(crate) fn benchmark(repo: &Repo, times: u32) -> Result<HashMap<Mode, Vec<String>>> {
    cargo_check(repo)?; // download dependencies
    let mut results = HashMap::new();

    let (base, incremental) = repeat(cargo_check, repo, times)?;
    results.insert(Mode::Check, base);
    results.insert(Mode::CheckIncremental, incremental);

    let (base, incremental) = repeat(cargo_debug, repo, times)?;
    results.insert(Mode::Debug, base);
    results.insert(Mode::DebugIncremental, incremental);

    let (base, incremental) = repeat(cargo_release, repo, times)?;
    results.insert(Mode::Release, base);
    results.insert(Mode::ReleaseIncremental, incremental);

    Ok(results)
}

fn repeat(
    f: fn(&Repo) -> Result<String>,
    repo: &Repo,
    times: u32,
) -> Result<(Vec<String>, Vec<String>)> {
    let mut result_base = Vec::new();
    let mut result_incremental = Vec::new();
    for _ in 0..times {
        repo.remove_target_dir()?;
        result_base.push(f(repo)?);
        repo.touch_src()?;
        result_incremental.push(f(repo)?);
    }
    Ok((result_base, result_incremental))
}

fn cargo(dir: &PathBuf, args: &[&str]) -> Result<String> {
    let output = Command::new("cargo")
        .current_dir(dir)
        .args(args)
        .output()
        .with_context(|| "failed to execute cargo")?;
    if !output.status.success() {
        let stderr =
            std::str::from_utf8(&output.stderr).with_context(|| "failed to decode output")?;
        return Err(anyhow!("Failed to execute cargo. Stderr - {:?}", stderr));
    }
    let stderr =
        std::str::from_utf8(&output.stderr).with_context(|| "failed to decode stderr of cargo")?;
    parse_run_time(stderr).ok_or_else(|| anyhow!("Failed to parse cargo output"))
}

fn cargo_check(repo: &Repo) -> Result<String> {
    println!("{} - Running cargo check", &repo.name);
    let dir = repo
        .get_base_directory()
        .ok_or_else(|| anyhow!("Could not find repo dir"))?;
    cargo(&dir, &["check"])
}

fn cargo_debug(repo: &Repo) -> Result<String> {
    println!("{} - Running cargo build", &repo.name);
    let dir = repo
        .get_base_directory()
        .ok_or_else(|| anyhow!("Could not find repo dir"))?;
    cargo(&dir, &["build"])
}

fn cargo_release(repo: &Repo) -> Result<String> {
    println!("{} - Running cargo release", &repo.name);
    let dir = repo
        .get_base_directory()
        .ok_or_else(|| anyhow!("Could not find repo dir"))?;
    cargo(&dir, &["build", "--release"])
}

fn parse_run_time(stderr: &str) -> Option<String> {
    let line = stderr.lines().last()?;
    let end = line.split("in ").last()?;
    Some(end.to_string())
}
