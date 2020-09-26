use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;

use anyhow::{anyhow, Context, Result};
use enum_iterator::IntoEnumIterator;
use parse_duration::parse;
use serde::{Deserialize, Serialize};

use crate::repo::Repo;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, IntoEnumIterator)]
pub(crate) enum CompilerMode {
    Check,
    Debug,
    Release,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, IntoEnumIterator)]
pub(crate) enum ProfileMode {
    Clean,
    Incremental,
    PrintIncremental,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Bytes(u64);

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Milliseconds(u64);

pub(crate) fn compile_time_profile(
    repo: &Repo,
    times: u32,
) -> Result<HashMap<CompilerMode, HashMap<ProfileMode, Vec<Milliseconds>>>> {
    cargo_check(repo)?; // download dependencies

    let mut results = HashMap::new();

    results.insert(CompilerMode::Check, repeat(cargo_check, repo, times)?);
    results.insert(CompilerMode::Debug, repeat(cargo_debug, repo, times)?);
    results.insert(CompilerMode::Release, repeat(cargo_release, repo, times)?);

    Ok(results)
}

pub(crate) fn size_profile(
    repo: &Repo,
) -> Result<(Bytes, Bytes)> {
    let debug_size = get_file_size(repo, CompilerMode::Debug)?;
    let release_size = get_file_size(repo, CompilerMode::Release)?;
    Ok((debug_size, release_size))
}

fn repeat(
    f: fn(&Repo) -> Result<Milliseconds>,
    repo: &Repo,
    times: u32,
) -> Result<HashMap<ProfileMode, Vec<Milliseconds>>> {
    let mut result = HashMap::with_capacity(3);
    for _ in 0..times {
        repo.remove_target_dir()?;
        result
            .entry(ProfileMode::Clean)
            .or_insert(Vec::with_capacity(times as usize))
            .push(f(repo)?);

        repo.touch_src()?;
        result
            .entry(ProfileMode::Incremental)
            .or_insert(Vec::with_capacity(times as usize))
            .push(f(repo)?);

        repo.add_println()?;
        result
            .entry(ProfileMode::PrintIncremental)
            .or_insert(Vec::with_capacity(times as usize))
            .push(f(repo)?);
        repo.git_reset()?;
    }
    Ok(result)
}

fn cargo(dir: &PathBuf, args: &[&str]) -> Result<Milliseconds> {
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

fn cargo_check(repo: &Repo) -> Result<Milliseconds> {
    println!("{} - Running cargo check", &repo.name);
    let dir = repo
        .get_base_directory()
        .ok_or_else(|| anyhow!("Could not find repo dir"))?;
    cargo(&dir, &["check"])
}

fn cargo_debug(repo: &Repo) -> Result<Milliseconds> {
    println!("{} - Running cargo build", &repo.name);
    let dir = repo
        .get_base_directory()
        .ok_or_else(|| anyhow!("Could not find repo dir"))?;
    cargo(&dir, &["build"])
}

fn cargo_release(repo: &Repo) -> Result<Milliseconds> {
    println!("{} - Running cargo release", &repo.name);
    let dir = repo
        .get_base_directory()
        .ok_or_else(|| anyhow!("Could not find repo dir"))?;
    cargo(&dir, &["build", "--release"])
}

fn parse_run_time(stderr: &str) -> Option<Milliseconds> {
    let line = stderr.lines().last()?;
    let end = line.split("in ").last()?;
    let duration = parse(end).ok()?;

    Some(Milliseconds(duration.as_millis() as u64))
}

fn get_file_size(repo: &Repo, compiler_mode: CompilerMode) -> Result<Bytes> {
    let binary_path = match compiler_mode {
        CompilerMode::Debug => {
            cargo_debug(repo)?;
            repo.get_debug_binary_path()
        }
        CompilerMode::Release => {
            cargo_release(repo)?;
            repo.get_release_binary_path()
        }
        _ => None,
    }
    .ok_or(anyhow!("No associated binary"))?;
    let file = std::fs::File::open(&binary_path)
        .with_context(|| anyhow!("failed to find binary - {:?}", binary_path))?;
    let metadata = file.metadata()?;
    Ok(Bytes(metadata.len()))
}

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::Result;
    #[test]
    fn cpu_profile_hello_world() -> Result<()> {
        crate::rustup::set_profile_minimal()?;
        crate::repo::create_working_directory(std::path::PathBuf::from("/tmp/prof/"))?;

        let repo: crate::repo::Repo = serde_json::from_str(
            r#"
            {
                "name": "helloworld",
                "sub_directory": "",
                "url": "https://github.com/nindalf/helloworld",
                "touch_file": "src/main.rs",
                "output": "helloworld",
                "commit_hash": "0ee163a",
                "min_version": "V1_45"
            }"#,
        )?;
        repo.clone_repo()?;
        let bench = compile_time_profile(&repo, 2)?;
        // TODO - add assertions
        Ok(())
    }
}
