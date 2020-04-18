use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use crate::cargo::Mode;
use crate::rustup::Version;

use once_cell::sync::OnceCell;

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
    max_version: Version,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Perf {
    repo: Repo,
    check: HashMap<Version, Vec<String>>,
    check_incremental: HashMap<Version, Vec<String>>,
    debug: HashMap<Version, Vec<String>>,
    debug_incremental: HashMap<Version, Vec<String>>,
    release: HashMap<Version, Vec<String>>,
    release_incremental: HashMap<Version, Vec<String>>,
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

    pub(crate) fn add_bench(self: &mut Perf, version: Version, bench: HashMap<Mode, Vec<String>>) {
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
}

impl Repo {
    pub(crate) fn clone_repo(self: &Repo) {
        let repo_path = self.get_base_directory();
        let working_directory = WORKING_DIRECTORY
            .get()
            .expect("Working directory not created yet");
        if !Path::new(&repo_path).exists() {
            println!("Cloning {}", &self.name);
            let output = Command::new("git")
                .current_dir(working_directory)
                .args(&["clone", &self.url])
                .output()
                .expect("failed to git clone");
            if output.status.success() {
                println!("Successfully cloned repo {}", &self.name);
            } else {
                let stderr = std::str::from_utf8(&output.stderr).expect("failed to decode output");
                println!("Failed to clone repo {}. Stderr - {}", &self.name, stderr);
                return;
            }
        }

        Command::new("git")
            .current_dir(&repo_path)
            .args(&["checkout", &self.commit_hash])
            .output()
            .expect("failed to git checkout");
    }

    pub(crate) fn remove_target_folder(self: &Repo) {
        let path = self.get_target_directory();
        if !path.exists() {
            println!("Directory {:?} doesn't exist. Skipping delete", &path);
            return;
        }
        std::fs::remove_dir_all(path).unwrap();
    }

    pub(crate) fn touch_src(self: &Repo) {
        Command::new("touch")
            .args(&[self.get_touch_file()])
            .output()
            .expect("failed to execute touch");
    }

    pub(crate) fn get_base_directory(self: &Repo) -> PathBuf {
        WORKING_DIRECTORY
            .get()
            .expect("Working directory not created yet")
            .join(&self.name)
            .join(&self.sub_directory)
    }

    fn get_target_directory(self: &Repo) -> PathBuf {
        WORKING_DIRECTORY
            .get()
            .expect("Working directory not created yet")
            .join(&self.name)
            .join("target")
    }

    fn get_touch_file(self: &Repo) -> PathBuf {
        WORKING_DIRECTORY
            .get()
            .expect("Working directory not created yet")
            .join(&self.name)
            .join(&self.sub_directory)
            .join(&self.touch_file)
    }

    pub(crate) fn supported_versions(self: &Repo) -> Vec<Version> {
        let min = self.min_version as u8;
        let max = self.max_version as u8;
        vec![
            Version::V1_34,
            Version::V1_35,
            Version::V1_36,
            Version::V1_37,
            Version::V1_38,
            Version::V1_39,
            Version::V1_40,
            Version::V1_41,
            Version::V1_42,
        ]
        .iter()
        .map(|v| *v)
        .filter(|v| *v as u8 >= min && *v as u8 <= max)
        .collect()
    }
}

pub(crate) fn create_working_directory(working_directory: &str) -> Result<()> {
    let mut working_dir = PathBuf::new();
    working_dir.push(working_directory);
    if !working_directory.ends_with(ARE_WE_FAST_YET) {
        working_dir.push(ARE_WE_FAST_YET);
    }
    if !working_dir.exists() {
        std::fs::create_dir(&working_dir)
            .with_context(|| "Failed to create working directory")?;
    }
    println!("Successfully created working directory - {:?}", working_dir);
    WORKING_DIRECTORY
        .set(working_dir)
        .map_err(|_| anyhow!("Failed to set global variable"))?;
    Ok(())
}

pub(crate) fn get_repos(repo_file: &str) -> Result<Vec<Repo>> {
    let file = File::open(repo_file)
        .with_context(|| "Failed to open repo file")?;
    let repos = serde_json::from_reader(file)?;
    Ok(repos)
}
