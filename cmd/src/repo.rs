use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;

use anyhow::{anyhow, Context, Result};
use enum_iterator::IntoEnumIterator;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};

use crate::cargo::Mode;
use crate::rustup::Version;

static ARE_WE_FAST_YET: &'static str = "arewefastyet-dir";
static WORKING_DIRECTORY: OnceCell<PathBuf> = OnceCell::new();

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Repo {
    pub name: String,
    sub_directory: String,
    url: String,
    touch_file: String,
    commit_hash: String,
    min_version: Version,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Perf {
    pub repo: Repo,
    check: HashMap<Version, Vec<u32>>,
    check_incremental: HashMap<Version, Vec<u32>>,
    debug: HashMap<Version, Vec<u32>>,
    debug_incremental: HashMap<Version, Vec<u32>>,
    release: HashMap<Version, Vec<u32>>,
    release_incremental: HashMap<Version, Vec<u32>>,
}

impl Perf {
    pub(crate) fn new(repo: Repo) -> Perf {
        Perf {
            repo,
            check: HashMap::new(),
            check_incremental: HashMap::new(),
            debug: HashMap::new(),
            debug_incremental: HashMap::new(),
            release: HashMap::new(),
            release_incremental: HashMap::new(),
        }
    }

    pub(crate) fn add_bench(self: &mut Perf, version: Version, bench: HashMap<Mode, Vec<u32>>) {
        for (mode, times) in bench {
            match mode {
                Mode::Check => self.check.insert(version, times),
                Mode::CheckIncremental => self.check_incremental.insert(version, times),
                Mode::Debug => self.debug.insert(version, times),
                Mode::DebugIncremental => self.debug_incremental.insert(version, times),
                Mode::Release => self.release.insert(version, times),
                Mode::ReleaseIncremental => self.release_incremental.insert(version, times),
            };
        }
    }

    pub(crate) fn versions_to_profile(self: &Perf) -> Vec<Version> {
        let min = self.repo.min_version as u8;
        Version::into_enum_iter()
            .filter(|v| *v as u8 >= min)
            .filter(|v| !self.version_profiled(v))
            .collect()
    }

    fn version_profiled(self: &Perf, version: &Version) -> bool {
        self.check.contains_key(version)
            && self.check_incremental.contains_key(version)
            && self.debug.contains_key(version)
            && self.debug_incremental.contains_key(version)
            && self.release.contains_key(version)
            && self.release_incremental.contains_key(version)
    }

    pub(crate) fn set_repo(self: &mut Perf, repo: Repo) {
        self.repo = repo;
    }
}

impl Repo {
    pub(crate) fn clone_repo(self: &Repo) -> Result<()> {
        let working_directory = WORKING_DIRECTORY
            .get()
            .ok_or_else(|| anyhow!("Working directory not set"))?;
        let repo_dir = self
            .get_base_directory()
            .ok_or_else(|| anyhow!("Could not find repo dir"))?;
        if !repo_dir.exists() {
            println!("Cloning {}", &self.name);
            git(&["clone", &self.url], working_directory)?;
            println!("Successfully cloned repo {}", &self.name);
        }
        git(&["checkout", &self.commit_hash], &repo_dir)?;
        Ok(())
    }

    pub(crate) fn remove_target_dir(self: &Repo) -> Result<()> {
        let target_dir = self
            .get_target_directory()
            .ok_or_else(|| anyhow!("Could not find target directory"))?;
        if !target_dir.exists() {
            println!("Directory {:?} doesn't exist. Skipping delete", &target_dir);
            return Ok(());
        }
        std::fs::remove_dir_all(target_dir).with_context(|| "failed to remove target directory")
    }

    pub(crate) fn touch_src(self: &Repo) -> Result<()> {
        let touch_file = self
            .get_touch_file()
            .ok_or_else(|| anyhow!("Could not find touch file"))?;
        if !touch_file.exists() {
            return Err(anyhow!("Touch file does not exist"));
        }
        let output = Command::new("touch")
            .args(&[&touch_file])
            .output()
            .with_context(|| "failed to execute touch")?;
        if !output.status.success() {
            let stderr = std::str::from_utf8(&output.stderr)
                .with_context(|| "failed to decode touch stderr")?;
            return Err(anyhow!(
                "Failed to touch file. Repo - {}, touch file - {:?}.\n Stderr - {}",
                &self.name,
                touch_file,
                stderr
            ));
        }
        Ok(())
    }

    pub(crate) fn get_base_directory(self: &Repo) -> Option<PathBuf> {
        let dir = WORKING_DIRECTORY.get()?;
        Some(dir.join(&self.name).join(&self.sub_directory))
    }

    fn get_target_directory(self: &Repo) -> Option<PathBuf> {
        let dir = WORKING_DIRECTORY.get()?;
        Some(dir.join(&self.name).join("target"))
    }

    fn get_touch_file(self: &Repo) -> Option<PathBuf> {
        let dir = WORKING_DIRECTORY.get()?;
        Some(
            dir.join(&self.name)
                .join(&self.sub_directory)
                .join(&self.touch_file),
        )
    }
}

pub(crate) fn create_working_directory(mut working_dir: PathBuf) -> Result<()> {
    if !working_dir.ends_with(ARE_WE_FAST_YET) {
        working_dir.push(ARE_WE_FAST_YET);
    }
    if !working_dir.exists() {
        println!("Creating {:?}", &working_dir);
        std::fs::create_dir_all(&working_dir).with_context(|| "Failed to create working directory")?;
    }
    println!("Created working directory - {:?}", working_dir);
    WORKING_DIRECTORY
        .set(working_dir)
        .map_err(|_| anyhow!("Failed to set global variable"))?;
    Ok(())
}

fn git(args: &[&str], dir: &PathBuf) -> Result<()> {
    let output = Command::new("git")
        .current_dir(&dir)
        .args(args)
        .output()
        .with_context(|| "failed to execute git")?;
    if !output.status.success() {
        let stderr =
            std::str::from_utf8(&output.stderr).with_context(|| "failed to decode git stderr")?;
        return Err(anyhow!(
            "Failed to execute git. Args - {:?}.\n Stderr - {}",
            &args,
            stderr
        ));
    }
    Ok(())
}
