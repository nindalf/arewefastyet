use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::fs::File;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Repo {
    pub name: String,
    pub url: String,
    touch_file: String,
    pub commit_hash: String,
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
    pub(crate) fn get_base_directory(self: &Repo) -> String {
        let home = dirs::home_dir().unwrap();
        home.join("arewefast-workdir")
            .join(&self.name)
            .to_str()
            .unwrap()
            .to_string()
    }

    pub(crate) fn get_target_directory(self: &Repo) -> String {
        let home = dirs::home_dir().unwrap();
        home.join("arewefast-workdir")
            .join(&self.name)
            .join("target")
            .to_str()
            .unwrap()
            .to_string()
    }

    pub(crate) fn get_touch_file(self: &Repo) -> String {
        let home = dirs::home_dir().unwrap();
        home.join("arewefast-workdir")
            .join(&self.name)
            .join(&self.touch_file)
            .to_str()
            .unwrap()
            .to_string()
    }

    pub(crate) fn supported_versions(self: &Repo) -> Vec<Version> {
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
    }
}

#[derive(Debug, Copy, Clone, Hash, Serialize, Deserialize, Eq, PartialEq)]
pub(crate) enum Version {
    V1_34,
    V1_35,
    V1_36,
    V1_37,
    V1_38,
    V1_39,
    V1_40,
    V1_41,
    V1_42,
}

impl Version {
    pub(crate) fn get_string(self: Self) -> &'static str {
        match self {
            Version::V1_34 => "1.34.0",
            Version::V1_35 => "1.35.0",
            Version::V1_36 => "1.36.0",
            Version::V1_37 => "1.37.0",
            Version::V1_38 => "1.38.0",
            Version::V1_39 => "1.39.0",
            Version::V1_40 => "1.40.0",
            Version::V1_41 => "1.41.0",
            Version::V1_42 => "1.42.0",
        }
    }
}

#[derive(Debug, Copy, Clone, Hash, Serialize, Deserialize, Eq, PartialEq)]
pub(crate) enum Mode {
    Check,
    CheckIncremental,
    Debug,
    DebugIncremental,
    Release,
    ReleaseIncremental,
}

pub(crate) fn get_repos(repo_file: &str) -> Result<Vec<Repo>, &'static str> {
    let file = File::open(repo_file).map_err(|_| "failed to open file")?;
    let repos = serde_json::from_reader(file).map_err(|_| "failed to deserialise")?;
    Ok(repos)
}
