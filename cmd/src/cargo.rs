use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;

use anyhow::{anyhow, Context, Result};
use parse_duration::parse;

use crate::repo::Repo;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub(crate) enum Mode {
    Check,
    CheckIncremental,
    CheckPrintIncremental,
    Debug,
    DebugIncremental,
    DebugPrintIncremental,
    Release,
    ReleaseIncremental,
    ReleasePrintIncremental,
}

pub(crate) fn benchmark(repo: &Repo, times: u32) -> Result<(HashMap<Mode, Vec<u32>>, u64, u64)> {
    cargo_check(repo)?; // download dependencies
    let mut results = HashMap::new();

    let (base, incremental, print_incremental) = repeat(cargo_check, repo, times)?;
    results.insert(Mode::Check, base);
    results.insert(Mode::CheckIncremental, incremental);
    results.insert(Mode::CheckPrintIncremental, print_incremental);

    let (base, incremental, print_incremental) = repeat(cargo_debug, repo, times)?;
    results.insert(Mode::Debug, base);
    results.insert(Mode::DebugIncremental, incremental);
    results.insert(Mode::DebugPrintIncremental, print_incremental);
    let binary_path = repo.get_debug_binary_path().ok_or(anyhow!("no debug binary"))?;
    let debug_size = get_file_size(cargo_debug, repo, &binary_path)?;

    let (base, incremental, print_incremental) = repeat(cargo_release, repo, times)?;
    results.insert(Mode::Release, base);
    results.insert(Mode::ReleaseIncremental, incremental);
    results.insert(Mode::ReleasePrintIncremental, print_incremental);
    let binary_path = repo.get_release_binary_path().ok_or(anyhow!("no release binary"))?;
    let release_size = get_file_size(cargo_release, repo, &binary_path)?;

    Ok((results, debug_size, release_size))
}

fn repeat(
    f: fn(&Repo) -> Result<u32>,
    repo: &Repo,
    times: u32,
) -> Result<(Vec<u32>, Vec<u32>, Vec<u32>)> {
    let mut result_base = Vec::new();
    let mut result_incremental = Vec::new();
    let mut result_print_incremental = Vec::new();
    for _ in 0..times {
        repo.remove_target_dir()?;
        result_base.push(f(repo)?);
        repo.touch_src()?;
        result_incremental.push(f(repo)?);
        repo.add_println()?;
        result_print_incremental.push(f(repo)?);
        repo.git_reset()?;
    }
    Ok((result_base, result_incremental, result_print_incremental))
}

fn cargo(dir: &PathBuf, args: &[&str]) -> Result<u32> {
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

fn cargo_check(repo: &Repo) -> Result<u32> {
    println!("{} - Running cargo check", &repo.name);
    let dir = repo
        .get_base_directory()
        .ok_or_else(|| anyhow!("Could not find repo dir"))?;
    cargo(&dir, &["check"])
}

fn cargo_debug(repo: &Repo) -> Result<u32> {
    println!("{} - Running cargo build", &repo.name);
    let dir = repo
        .get_base_directory()
        .ok_or_else(|| anyhow!("Could not find repo dir"))?;
    cargo(&dir, &["build"])
}

fn cargo_release(repo: &Repo) -> Result<u32> {
    println!("{} - Running cargo release", &repo.name);
    let dir = repo
        .get_base_directory()
        .ok_or_else(|| anyhow!("Could not find repo dir"))?;
    cargo(&dir, &["build", "--release"])
}

fn parse_run_time(stderr: &str) -> Option<u32> {
    let line = stderr.lines().last()?;
    let end = line.split("in ").last()?;
    let duration = parse(end).ok()?;

    Some(duration.as_millis() as u32)
}

fn get_file_size(
    f: fn(&Repo) -> Result<u32>,
    repo: &Repo,
    file_name: &PathBuf
) -> Result<u64> {
    f(repo)?;
    let file = std::fs::File::open(file_name)
        .with_context(|| anyhow!("failed to find binary - {:?}", file_name))?;
    let metadata = file.metadata()?;
    Ok(metadata.len())
}

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::Result;
    #[test]
    fn benchmark_hello_world() -> Result<()>{
        crate::rustup::set_profile_minimal()?;
        crate::repo::create_working_directory(std::path::PathBuf::from("/tmp/prof/"))?;

        let repo: crate::repo::Repo = serde_json::from_str(r#"
            {
                "name": "helloworld",
                "sub_directory": "",
                "url": "https://github.com/nindalf/helloworld",
                "touch_file": "src/main.rs",
                "commit_hash": "0ee163a",
                "min_version": "V1_45"
            }"#)?;
        repo.clone_repo()?;
        let bench = benchmark(&repo, 2)?;
        println!("{:?}", bench);
        // TODO - add assertions
        Ok(())
    }
}