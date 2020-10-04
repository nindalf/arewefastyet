use std::collections::HashMap;
use std::process::Command;

use anyhow::{anyhow, Context, Result};
use enum_iterator::IntoEnumIterator;
use once_cell::unsync::Lazy;
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
    PatchIncremental,
}

#[derive(Debug, Copy, Clone, Serialize, PartialOrd, PartialEq, Deserialize)]
pub(crate) struct Bytes(u64);

#[derive(Debug, Copy, Clone, Serialize, PartialOrd, PartialEq, Deserialize)]
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

pub(crate) fn size_profile(repo: &Repo) -> Result<(Bytes, Bytes)> {
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
            .entry(ProfileMode::PatchIncremental)
            .or_insert(Vec::with_capacity(times as usize))
            .push(f(repo)?);
        repo.git_reset()?;
    }
    Ok(result)
}

fn cargo(repo: &Repo, mode: CompilerMode) -> Result<Milliseconds> {
    let dir = repo
        .get_base_directory()
        .ok_or_else(|| anyhow!("Could not find repo dir"))?;
    let args: &[&str] = match mode {
        CompilerMode::Check => &["check"],
        CompilerMode::Debug => &["build"],
        CompilerMode::Release => &["build", "--release"],
    };

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
    log::info!("{} - Running cargo check", &repo.name);
    cargo(repo, CompilerMode::Check)
}

fn cargo_debug(repo: &Repo) -> Result<Milliseconds> {
    log::info!("{} - Running cargo build", &repo.name);
    cargo(repo, CompilerMode::Debug)
}

fn cargo_release(repo: &Repo) -> Result<Milliseconds> {
    log::info!("{} - Running cargo release", &repo.name);
    cargo(repo, CompilerMode::Release)
}

fn parse_run_time(stderr: &str) -> Option<Milliseconds> {
    let re = Lazy::new(|| {
        regex::Regex::new(r#"Finished .* target\(s\) in ([0-9\.ms ]*)"#).unwrap()
    });
    re.captures(stderr)
        .and_then(|capture| capture.get(1))
        .map(|m| m.as_str())
        .and_then(|c| parse(c).ok())
        .map(|d| Milliseconds(d.as_millis() as u64))
}

fn get_file_size(repo: &Repo, compiler_mode: CompilerMode) -> Result<Bytes> {
    let output_path = match compiler_mode {
        CompilerMode::Debug => {
            cargo_debug(repo)?;
            repo.get_debug_output_path()
        }
        CompilerMode::Release => {
            cargo_release(repo)?;
            repo.get_release_output_path()
        }
        _ => None,
    }
    .ok_or(anyhow!("No associated output"))?;
    let file = std::fs::File::open(&output_path)
        .with_context(|| anyhow!("failed to find output - {:?}", output_path))?;
    let metadata = file.metadata()?;
    Ok(Bytes(metadata.len()))
}

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::Result;

    #[test]
    fn compile_time_hello_world() -> Result<()> {
        let repo = init_repo()?;
        let times: usize = 2;
        let compile_times = compile_time_profile(&repo, times as u32)?; // run once on any version

        for compiler_mode in CompilerMode::into_enum_iter() {
            for profile_mode in ProfileMode::into_enum_iter() {
                let result_times = compile_times
                    .get(&compiler_mode)
                    .unwrap()
                    .get(&profile_mode)
                    .unwrap();
                assert_eq!(result_times.len(), times);
                assert!(result_times[0] > Milliseconds(0));
            }
        }

        Ok(())
    }

    #[test]
    fn output_size_hello_world() -> Result<()> {
        let repo = init_repo()?;

        let (debug_size, release_size) = size_profile(&repo)?;

        assert!(release_size > Bytes(0));
        assert!(debug_size > release_size);

        Ok(())
    }

    use std::sync::Once;
    static INIT: Once = Once::new();
    fn init_repo() -> Result<Repo> {
        let repo: crate::repo::Repo = serde_json::from_str(
            r#"
            {
                "name": "helloworld",
                "sub_directory": "",
                "url": "https://github.com/nindalf/helloworld",
                "touch_file": "src/main.rs",
                "output": "helloworld",
                "commit": "v1.0",
                "min_version": "V1_45"
            }"#,
        )?;
        INIT.call_once(|| {
            crate::rustup::set_profile_minimal().unwrap();
            crate::repo::create_working_directory(std::path::PathBuf::from("/tmp/prof/")).unwrap();
            repo.clone_repo().unwrap();
        });
        Ok(repo)
    }

    #[test]
    fn parse_compile_times() -> Result<()> {
        let inputs = [
            ("Finished dev [unoptimized + debuginfo] target(s) in 4m 26s", Milliseconds(266000)),
            ("Finished dev [unoptimized + debuginfo] target(s) in 0.86s", Milliseconds(860)),
            ("Finished release [optimized] target(s) in 1.33s", Milliseconds(1330)),
        ];

        for (input, expected) in inputs.iter() {
            let output = super::parse_run_time(input)
                .ok_or(anyhow!("Failed to parse {}", input))?;
            assert_eq!(&output, expected);
        }
        Ok(())
    }
}
