use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};

use std::process::Command;
#[derive(Debug, Copy, Clone, Hash, Serialize, Deserialize, Eq, PartialEq)]
pub(crate) enum Version {
    V1_34 = 34,
    V1_35 = 35,
    V1_36 = 36,
    V1_37 = 37,
    V1_38 = 38,
    V1_39 = 39,
    V1_40 = 40,
    V1_41 = 41,
    V1_42 = 42,
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

pub(crate) fn set_profile_minimal() {
    println!("Setting profile to minimal");
    let output = Command::new("rustup")
        .args(&["set", "profile", "minimal"])
        .output()
        .expect("failed to execute process");
    if output.status.success() {
        println!("Successfully set profile");
    } else {
        println!("Failed to set profile");
    }
}

pub(crate) fn set_version(version: Version) {
    download_toolchain(version);
    set_toolchain(version);
}

fn download_toolchain(version: Version) {
    let version = version.get_string();
    let output = Command::new("rustup")
        .args(&["toolchain", "install", version])
        .output()
        .expect("failed to execute process");
    if !output.status.success() {
        let stderr = std::str::from_utf8(&output.stderr).expect("failed to decode output");
        println!(
            "Failed to download toolchain {}. Stderr - {}",
            version, stderr
        );
    }
}

fn set_toolchain(version: Version) {
    let version = Version::get_string(version);
    let output = Command::new("rustup")
        .args(&["default", version])
        .output()
        .expect("failed to execute process");
    if !output.status.success() {
        let stderr = std::str::from_utf8(&output.stderr).expect("failed to decode output");
        println!(
            "Failed to switch toolchain {}. Stderr - {}",
            version, stderr
        );
    }
}
