use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::File;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Repo {
    pub name: String,
    pub url: String,
    touch_file: String,
    pub commit_hash: String,
    min_version: Version,
    max_version: Version,
}

impl Repo {
    pub(crate) fn get_base_directory(self: &Repo) -> String {
        let home = dirs::home_dir().unwrap();
        home.join("arewefast-workdir").join(&self.name).to_str().unwrap().to_string()
        // format!("/home/nindalf/arewefast-workdir/{}", self.name)
    }

    pub(crate) fn get_target_directory(self: &Repo) -> String {
        let home = dirs::home_dir().unwrap();
        home.join("arewefast-workdir").join(&self.name).join("target").to_str().unwrap().to_string()
    }

    pub(crate) fn get_touch_file(self: &Repo) -> String {
        let home = dirs::home_dir().unwrap();
        home.join("arewefast-workdir").join(&self.name).join(&self.touch_file).to_str().unwrap().to_string()
    }
}

#[derive(Debug, Copy, Clone, Hash, Serialize, Deserialize, Eq, PartialEq)]
pub (crate) enum Version {
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
