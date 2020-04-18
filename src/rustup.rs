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
    pub fn get_string(self: Self) -> &'static str {
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

pub(crate) fn set_profile_minimal() -> Result<()> {
    rustup(&["set", "profile", "minimal"])?;
    println!("Set profile to minimal");
    Ok(())
}

pub(crate) fn set_version(version: Version) -> Result<()> {
    let version = version.get_string();
    rustup(&["toolchain", "install", version])?;
    rustup(&["default", version])?;
    println!("Switched to version {}", version);
    Ok(())
}

fn rustup(args: &[&str]) -> Result<()> {
    let output = Command::new("rustup")
        .args(args)
        .output()
        .with_context(|| "failed to execute rustup")?;
    if !output.status.success() {
        let stderr = std::str::from_utf8(&output.stderr).with_context(|| "failed to decode output")?;
        return Err(anyhow!(
            "Failed to execute rustup. Stderr - {}",
            stderr
        ));
    }
    Ok(())
}
